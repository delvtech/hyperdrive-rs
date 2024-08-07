mod macros;
mod rng;
mod sign;
mod utils;
mod value;

use std::{
    fmt,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

use ethers::types::{I256, U256, U512};
use eyre::{bail, eyre, Result};
pub use sign::*;
pub use utils::*;
pub use value::*;

/// A fixed point wrapper around the `U256` type from ethers-rs.
///
/// This fixed point type is a heavily based on Solidity's FixedPointMath
/// library with a few changes:
/// - The outward type of the underlying value is generic, allowing the library
///   to be used with any type that implements `FixedPointValue`, including
///   signed integers, and ensuring that the instance is bounded by the generic
///   type's limits.
/// - Support for overflowing intermediate operations in `mul_div_down` and
///  `mul_div_up` via `U512`.
///
/// Each of the functions is fuzz tested against the Solidity implementation to
/// ensure that the behavior is identical given values bounded by the Solidity
/// implementation's limits.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct FixedPoint<T: FixedPointValue> {
    value: T,
    decimals: u8,
}

impl<T: FixedPointValue> Default for FixedPoint<T> {
    fn default() -> Self {
        Self {
            value: T::default(),
            decimals: T::MAX_DECIMALS,
        }
    }
}

impl<T: FixedPointValue> FixedPoint<T> {
    const MIN: Self = Self {
        value: T::MIN,
        decimals: T::MAX_DECIMALS,
    };

    const MAX: Self = Self {
        value: T::MAX,
        decimals: T::MAX_DECIMALS,
    };

    const MAX_DECIMALS: u8 = T::MAX_DECIMALS;

    // Constructors //

    /// The primary constructor for `FixedPoint`. All instances should be
    /// created using this method to ensure consistent behavior.
    pub fn new<V: Into<T>>(value: V) -> Result<Self> {
        let value = value.into();

        if (value > T::MAX) || (value < T::MIN) {
            bail!(
                r#"Value {value:?} is out of FixedPoint range:
    min: {min:?}
    max: {max:?}"#,
                min = T::MIN,
                max = T::MAX,
            );
        }

        Ok(Self {
            value,
            // TODO: Add support for variable decimals.
            decimals: Self::MAX_DECIMALS,
        })
    }

    pub fn zero() -> Self {
        Self::new(0).unwrap()
    }

    pub fn from_dec_str(s: &str) -> Result<Self> {
        if s.starts_with('-') {
            let uint = u256_from_str(&s[1..])?;
            let raw = T::from_u256(uint)?.flip_sign();
            return Self::new(raw);
        }
        let uint = u256_from_str(s)?;
        let raw = T::from_u256(uint)?;
        Self::new(raw)
    }

    pub fn saturate_sign(sign: FixedPointSign) -> Self {
        match sign {
            FixedPointSign::Positive => Self::MAX,
            FixedPointSign::Negative => Self::MIN,
        }
    }

    // Getters //

    pub fn raw(&self) -> T {
        self.value
    }

    pub fn decimals(&self) -> u8 {
        self.decimals
    }

    pub fn sign(&self) -> FixedPointSign {
        if self.is_negative() {
            return FixedPointSign::Negative;
        }
        FixedPointSign::Positive
    }

    // Predicates //

    pub fn is_negative(&self) -> bool {
        self.raw().is_negative()
    }

    pub fn is_positive(&self) -> bool {
        !self.is_negative()
    }

    pub fn is_zero(&self) -> bool {
        self.raw().is_zero()
    }

    // Math //

    /// One with the same scale as this fixed point number, i.e., `1.0`.
    ///
    /// # Panics
    ///
    /// Panics if `10` to the power of `self.decimals()` overflows `T`.
    pub fn one(&self) -> Self {
        let uint = U256::from(10).pow(self.decimals().into());
        Self::new(T::from_u256(uint).unwrap()).unwrap()
    }

    /// Computes the absolute value of self.
    ///
    /// # Panics
    ///
    /// If the absolute value of self overflows `T`, e.g., if self is the
    /// minimum value of a signed integer.
    pub fn abs(&self) -> Self {
        Self::new(self.raw().abs()).unwrap()
    }

    /// Computes the absolute value of self as a `U256` to avoid overflow.
    pub fn unsigned_abs(&self) -> FixedPoint<U256> {
        FixedPoint::new(self.raw().unsigned_abs()).unwrap()
    }

    pub fn abs_diff(&self, other: Self) -> FixedPoint<U256> {
        if self.sign() != other.sign() {
            return FixedPoint::new(self.unsigned_abs().raw() + other.unsigned_abs().raw())
                .unwrap();
        }
        let diff = if self > &other {
            self.raw() - other.raw()
        } else {
            other.raw() - self.raw()
        };
        FixedPoint::new(diff.to_u256().unwrap()).unwrap()
    }

    pub fn mul_div_down(&self, other: Self, divisor: Self) -> Result<Self> {
        if divisor.is_zero() {
            bail!("Cannot divide by zero.");
        }
        let u256 = U256::try_from(
            self.raw()
                .abs()
                .to_u256()?
                .full_mul(other.raw().abs().to_u256()?.into())
                .div(divisor.raw().abs().to_u256()?),
        )
        .or_else(|_| bail!("FixedPoint operation overflowed: {self} * {other} / {divisor}"))?;
        let sign = self.sign().flip_if(other.sign() != divisor.sign());
        let raw = T::from_u256(u256)?.flip_sign_if(sign.is_negative());
        Self::new(raw)
    }

    pub fn mul_div_up(&self, other: Self, divisor: Self) -> Result<Self> {
        if divisor.is_zero() {
            bail!("Cannot divide by zero.");
        }
        let (u512, remainder) = self
            .raw()
            .abs()
            .to_u256()?
            .full_mul(other.raw().abs().to_u256()?.into())
            .div_mod(divisor.raw().abs().to_u256()?.into());
        let rounded_u256 = U256::try_from(u512 + (remainder.gt(&U512::zero()) as u128))
            .or_else(|_| bail!("FixedPoint operation overflowed: {self} * {other} / {divisor}"))?;
        let sign = self.sign().flip_if(other.sign() != divisor.sign());
        let raw = T::from_u256(rounded_u256)?.flip_sign_if(sign.is_negative());
        Self::new(raw)
    }

    pub fn mul_down(&self, other: Self) -> Result<Self> {
        self.mul_div_down(other, self.one())
    }

    pub fn mul_up(&self, other: Self) -> Result<Self> {
        self.mul_div_up(other, self.one())
    }

    pub fn div_down(self, other: Self) -> Result<Self> {
        self.mul_div_down(self.one(), other)
    }

    pub fn div_up(self, other: Self) -> Result<Self> {
        self.mul_div_up(self.one(), other)
    }

    pub fn pow(self, y: Self) -> Result<Self> {
        let one = self.one();

        // If the exponent is negative, return 1 / x^abs(y).
        if y.is_negative() {
            let abs_result = self.pow(y.abs())?;

            // FIXME: What's the correct return value here?
            if abs_result.is_zero() {
                return Ok(Self::zero());
            }

            return Ok(one.div_down(abs_result)?);
        }

        // If the exponent is 0, return 1.
        if y.is_zero() {
            return Ok(one);
        }

        // If the base is 0, return 0.
        if self.is_zero() {
            return Ok(Self::zero());
        }

        // Using properties of logarithms we calculate x^y: -> ln(x^y) = y *
        // ln(x) -> e^(y * ln(x)) = x^y
        let y_int256 = y.to_i256()?;

        // Compute y*ln(x) Any overflow for x will be caught in _ln() in the
        // initial bounds check
        let lnx = ln(self.to_i256()?)?;
        let mut ylnx = y_int256.wrapping_mul(lnx);
        ylnx = ylnx.wrapping_div(one.to_i256()?);

        // Calculate exp(y * ln(x)) to get x^y
        let (sign, abs) = exp(ylnx)?.into_sign_and_abs();
        let raw = T::from_u256(abs)?.flip_sign_if(sign.is_negative());
        Self::new(raw)
    }

    // Conversion to unsigned & signed ethers types //

    pub fn to_u256(&self) -> Result<U256> {
        if self.is_negative() {
            bail!("Cannot convert negative FixedPoint {self} to U256.");
        }
        self.raw().to_u256()
    }

    pub fn to_i256(&self) -> Result<I256> {
        let abs = self.unsigned_abs().raw();
        let abs_max = FixedPoint::<I256>::saturate_sign(self.sign())
            .raw()
            .unsigned_abs();
        if abs > abs_max {
            bail!("FixedPoint {self} is too large to convert to I256.");
        }
        I256::checked_from_sign_and_abs(self.sign().into(), abs)
            .ok_or(eyre!("Failed to convert FixedPoint {self} to I256."))
    }

    // Conversion to unsigned and signed std types //

    pub fn to_u128(&self) -> Result<u128> {
        if self.is_negative() {
            bail!("Cannot convert negative FixedPoint {self} to u128.");
        }
        self.raw().to_u128()
    }

    pub fn to_i128(&self) -> Result<i128> {
        let i256 = self.to_i256()?;
        i128::try_from(i256)
            .or_else(|_| bail!("FixedPoint {self} is too large to convert to i128."))
    }

    // Formatting //

    pub fn to_scaled_string(&self) -> String {
        let decimals = (self.decimals()) as usize;
        let zero = T::from(0);
        let ten = T::from(10);
        let char_code_zero = T::from(48);
        let mut value = self.raw().abs();
        let mut digits = 0;
        let mut result = vec![];
        while value > zero {
            if digits == decimals && decimals > 0 {
                result.push('.');
            }

            let char_code = (value % ten + char_code_zero).to_u128().unwrap();
            result.push(char_code as u8 as char);
            value /= ten;
            digits += 1;
        }

        // Add leading zeros.
        if digits < decimals {
            result.resize(result.len() + decimals - digits, '0');
            digits += decimals - digits;
        }

        // Add the decimal point and leading zero.
        if digits == decimals {
            if decimals > 0 {
                result.push('.');
            }
            result.push('0');
        }

        format!("{}{}", self.sign(), result.iter().rev().collect::<String>())
    }
}

// Comparisons //

// impl<T: FixedPointValue> PartialEq for FixedPoint<T> {
//     fn eq(&self, other: &Self) -> bool {
//         self.sign() == other.sign() && self.raw().abs() == other.raw().abs()
//     }
// }
// impl<T: FixedPointValue> Eq for FixedPoint<T> {}

// Formatting //

impl<T: FixedPointValue> fmt::Debug for FixedPoint<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FixedPoint({})", self.to_scaled_string())
    }
}

impl<T: FixedPointValue> fmt::Display for FixedPoint<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_scaled_string())
    }
}

// Conversions //

impl<T: FixedPointValue> From<T> for FixedPoint<T> {
    fn from(value: T) -> Self {
        Self::new(value).unwrap()
    }
}

/// Takes a list of raw types and implements conversions to `FixedPoint<T>` for
/// each one that can be converted to `T`.
macro_rules! impl_from_nested_raw {
    ($($t:ty),*) => {
        $(
            impl<T: FixedPointValue + From<$t>> From<$t> for FixedPoint<T> {
                fn from(u: $t) -> Self {
                    Self::new(u).unwrap()
                }
            }
        )*
    };
    ($($tt:tt)*) => {};
}

// Any FixedPointValue that can be created from these types can be converted to
// a FixedPoint.
impl_from_nested_raw!(usize, isize, u64, i64, u32, i32, u16, i16, u8, i8, [u8; 32], bool);

/// Takes a mapping of raw types to `FixedPoint` methods and implements
/// conversions to each raw type using the corresponding method.
macro_rules! impl_mapped_try_into {
    ($($t:ty => $fn:ident),*) => {
        $(
            impl<T: FixedPointValue> TryFrom<FixedPoint<T>> for $t {
                type Error = eyre::ErrReport;

                fn try_from(f: FixedPoint<T>) -> eyre::Result<Self> {
                    f.$fn()
                }
            }
        )*
    };
    ($($tt:tt)*) => {};
}

impl_mapped_try_into!(
    U256 => to_u256,
    I256 => to_i256,
    u128 => to_u128,
    i128 => to_i128
);

// Operator overloads //

impl<T: FixedPointValue> Neg for FixedPoint<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(self.raw().flip_sign()).unwrap()
    }
}

/// Takes a mapping of operator traits to `FixedPoint` methods and implements
/// the operator and assignment operator for each one.
macro_rules! impl_mapped_operator {
    ($($trait:ident => $fn:ident),*) => {
        $(
            paste::paste! {

                impl<T: FixedPointValue> std::ops::$trait for FixedPoint<T> {
                    type Output = Self;

                    fn [<$trait:lower>](self, other: Self) -> Self::Output {
                        self.$fn(other).unwrap()
                    }
                }

                impl<T: FixedPointValue> std::ops::[<$trait Assign>] for FixedPoint<T> {
                    fn [<$trait:lower _assign>](&mut self, other: Self) {
                        *self = self.[<$trait:lower>](other);
                    }
                }
            }
        )*
    };
    ($($tt:tt)*) => {};
}

impl_mapped_operator!(
    // use `mul_down` for `*` and `*=`.
    Mul => mul_down,
    // use `div_down` for `/` and `/=`.
    Div => div_down
);

/// Takes a list of operator traits and implements the operator and assignment
/// operator for each one by forwarding to the corresponding method on the
/// underlying `FixedPointValue`.
macro_rules! impl_forwarded_operator {
    ($($trait:ident),*) => {
        $(
            paste::paste! {

                impl<T: FixedPointValue> std::ops::$trait for FixedPoint<T> {
                    type Output = Self;

                    fn [<$trait:lower>](self, other: Self) -> Self::Output {
                        Self::new(self.raw().[<$trait:lower>](other.raw())).unwrap()
                    }
                }

                impl<T: FixedPointValue> std::ops::[<$trait Assign>] for FixedPoint<T> {
                    fn [<$trait:lower _assign>](&mut self, other: Self) {
                        *self = self.[<$trait:lower>](other);
                    }
                }
            }
        )*
    };
    ($($tt:tt)*) => {};
}

// Forward these operators to the underlying `FixedPointValue`.
impl_forwarded_operator!(Add, Sub, Rem);

#[cfg(test)]
mod tests {
    use std::{panic, u128};

    use ethers::signers::Signer;
    use hyperdrive_wrappers::wrappers::mock_fixed_point_math::MockFixedPointMath;
    use rand::{thread_rng, Rng};
    use test_utils::{chain::Chain, constants::DEPLOYER};

    use super::*;
    use crate::uint256;

    /// The maximum number that can be divided by another in the Solidity
    /// implementation.
    fn max_sol_numerator() -> FixedPoint<U256> {
        FixedPoint::from(U256::MAX / uint256!(1e18))
    }

    #[test]
    fn test_fixed_point_fmt() {
        // fmt::Debug
        assert_eq!(
            format!("{:?}", fixed_u128!(1)),
            "FixedPoint(0.000000000000000001)"
        );
        assert_eq!(
            format!("{:?}", fixed_u128!(1.23456e18)),
            "FixedPoint(1.234560000000000000)"
        );
        assert_eq!(
            format!("{:?}", fixed_u128!(50_000.234_56e18)),
            "FixedPoint(50000.234560000000000000)"
        );

        // fmt::Display
        assert_eq!(format!("{}", fixed_u128!(1)), "0.000000000000000001");
        assert_eq!(
            format!("{}", fixed_u128!(1.23456e18)),
            "1.234560000000000000"
        );
        assert_eq!(
            format!("{}", fixed_u128!(50_000.234_56e18)),
            "50000.234560000000000000"
        );
    }

    #[test]
    fn test_sub_failure() {
        // Ensure that subtraction producing negative numbers fails.
        assert!(panic::catch_unwind(|| fixed_u128!(1e18) - fixed!(2e18)).is_err());
    }

    #[test]
    fn test_mul_div_down_failure() {
        // Ensure that division by zero fails.
        assert!(fixed_u128!(1e18)
            .mul_div_down(fixed!(1e18), fixed!(0))
            .is_err());
    }

    #[tokio::test]
    async fn fuzz_mul_div_down() -> Result<()> {
        let chain = Chain::connect(None, None).await?;
        chain.deal(DEPLOYER.address(), uint256!(100_000e18)).await?;
        let client = chain.client(DEPLOYER.clone()).await?;
        let mock_fixed_point_math = MockFixedPointMath::deploy(client, ())?.send().await?;

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..10_000 {
            let max = max_sol_numerator();
            let a = rng.gen_range(0.into()..=max);
            let b = rng.gen_range(0.into()..=max / a);
            let c = rng.gen_range(0.into()..=max);
            let actual = a.mul_div_down(b, c);
            match mock_fixed_point_math
                .mul_div_down(a.raw(), b.raw(), c.raw())
                .call()
                .await
            {
                Ok(expected) => assert_eq!(actual.unwrap(), FixedPoint::from(expected)),
                Err(_) => assert!(actual.is_err()),
            }
        }

        Ok(())
    }

    #[test]
    fn test_mul_div_up_failure() {
        // Ensure that division by zero fails.
        assert!(fixed_u128!(1e18)
            .mul_div_up(fixed!(1e18), fixed!(0))
            .is_err());
    }

    #[tokio::test]
    async fn fuzz_mul_div_up() -> Result<()> {
        let chain = Chain::connect(None, None).await?;
        chain.deal(DEPLOYER.address(), uint256!(100_000e18)).await?;
        let client = chain.client(DEPLOYER.clone()).await?;
        let mock_fixed_point_math = MockFixedPointMath::deploy(client, ())?.send().await?;

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..10_000 {
            let max = max_sol_numerator();
            let a = rng.gen_range(0.into()..=max);
            let b = rng.gen_range(0.into()..=max / a);
            let c = rng.gen_range(0.into()..=max);
            let actual = a.mul_div_up(b, c);
            match mock_fixed_point_math
                .mul_div_up(a.raw(), b.raw(), c.raw())
                .call()
                .await
            {
                Ok(expected) => assert_eq!(actual.unwrap(), FixedPoint::from(expected)),
                Err(_) => assert!(actual.is_err()),
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn fuzz_mul_down() -> Result<()> {
        let chain = Chain::connect(None, None).await?;
        chain.deal(DEPLOYER.address(), uint256!(100_000e18)).await?;
        let client = chain.client(DEPLOYER.clone()).await?;
        let mock_fixed_point_math = MockFixedPointMath::deploy(client, ())?.send().await?;

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..10_000 {
            let a: FixedPoint<U256> = rng.gen();
            let b: FixedPoint<U256> = rng.gen();
            let actual = a.mul_down(b);
            match mock_fixed_point_math
                .mul_down(a.raw(), b.raw())
                .call()
                .await
            {
                Ok(expected) => assert_eq!(actual.unwrap(), FixedPoint::from(expected)),
                Err(_) => assert!(actual.is_err()),
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn fuzz_mul_up() -> Result<()> {
        let chain = Chain::connect(None, None).await?;
        chain.deal(DEPLOYER.address(), uint256!(100_000e18)).await?;
        let client = chain.client(DEPLOYER.clone()).await?;
        let mock_fixed_point_math = MockFixedPointMath::deploy(client, ())?.send().await?;

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..10_000 {
            let a: FixedPoint<U256> = rng.gen();
            let b: FixedPoint<U256> = rng.gen();
            let actual = a.mul_up(b);
            match mock_fixed_point_math.mul_up(a.raw(), b.raw()).call().await {
                Ok(expected) => assert_eq!(actual.unwrap(), FixedPoint::from(expected)),
                Err(_) => assert!(actual.is_err()),
            }
        }

        Ok(())
    }

    #[test]
    fn test_div_down_failure() {
        // Ensure that division by zero fails.
        assert!(fixed_u128!(1e18).div_down(fixed!(0)).is_err());
    }

    #[tokio::test]
    async fn fuzz_div_down() -> Result<()> {
        let chain = Chain::connect(None, None).await?;
        chain.deal(DEPLOYER.address(), uint256!(100_000e18)).await?;
        let client = chain.client(DEPLOYER.clone()).await?;
        let mock_fixed_point_math = MockFixedPointMath::deploy(client, ())?.send().await?;

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..10_000 {
            let max = max_sol_numerator();
            let a = rng.gen_range(0.into()..=max);
            let b = rng.gen_range(0.into()..=max);
            let actual = a.div_down(b);
            match mock_fixed_point_math
                .div_down(a.raw(), b.raw())
                .call()
                .await
            {
                Ok(expected) => assert_eq!(actual.unwrap(), FixedPoint::from(expected)),
                Err(_) => assert!(actual.is_err()),
            }
        }

        Ok(())
    }

    #[test]
    fn test_div_up_failure() {
        // Ensure that division by zero fails.
        assert!(fixed_u128!(1e18).div_up(fixed!(0)).is_err());
    }

    #[tokio::test]
    async fn fuzz_div_up() -> Result<()> {
        let chain = Chain::connect(None, None).await?;
        chain.deal(DEPLOYER.address(), uint256!(100_000e18)).await?;
        let client = chain.client(DEPLOYER.clone()).await?;
        let mock_fixed_point_math = MockFixedPointMath::deploy(client, ())?.send().await?;

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..10_000 {
            let max = max_sol_numerator();
            let a = rng.gen_range(0.into()..=max);
            let b = rng.gen_range(0.into()..=max);
            let actual = a.div_up(b);
            match mock_fixed_point_math.div_up(a.raw(), b.raw()).call().await {
                Ok(expected) => assert_eq!(actual.unwrap(), FixedPoint::from(expected)),
                Err(_) => assert!(actual.is_err()),
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn fuzz_pow_narrow() -> Result<()> {
        let chain = Chain::connect(None, None).await?;
        chain.deal(DEPLOYER.address(), uint256!(100_000e18)).await?;
        let client = chain.client(DEPLOYER.clone()).await?;
        let mock_fixed_point_math = MockFixedPointMath::deploy(client, ())?.send().await?;

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..10_000 {
            let x = rng.gen_range(fixed!(0)..=fixed!(1e18));
            let y = rng.gen_range(fixed!(0)..=fixed!(1e18));
            let actual = x.pow(y);
            match mock_fixed_point_math.pow(x.raw(), y.raw()).call().await {
                Ok(expected) => {
                    assert_eq!(actual.unwrap(), FixedPoint::from(expected));
                }
                Err(_) => assert!(actual.is_err()),
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn fuzz_pow() -> Result<()> {
        let chain = Chain::connect(None, None).await?;
        chain.deal(DEPLOYER.address(), uint256!(100_000e18)).await?;
        let client = chain.client(DEPLOYER.clone()).await?;
        let mock_fixed_point_math = MockFixedPointMath::deploy(client, ())?.send().await?;

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..10_000 {
            let x: FixedPoint<U256> = rng.gen();
            let y: FixedPoint<U256> = rng.gen();
            let actual = x.pow(y);
            match mock_fixed_point_math.pow(x.raw(), y.raw()).call().await {
                Ok(expected) => assert_eq!(actual.unwrap(), FixedPoint::from(expected)),
                Err(_) => assert!(actual.is_err()),
            }
        }

        Ok(())
    }
}
