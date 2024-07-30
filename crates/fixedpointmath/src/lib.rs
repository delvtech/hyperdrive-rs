mod macros;
mod sign;
mod utils;
mod value;

use core::panic;
use std::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use ethers::types::{I256, U256, U512};
use eyre::{bail, eyre, Error, Report, Result};
use rand::{
    distributions::{
        uniform::{SampleBorrow, SampleUniform, UniformFloat, UniformSampler},
        Distribution, Standard,
    },
    Rng,
};
pub use sign::*;
pub use utils::*;
pub use value::*;

/// A fixed point wrapper around ethers-rs.
///
/// This fixed point type is a heavily based on Solidity's FixedPointMath
/// library with a few changes:
/// - Support for negative numbers.
/// - Support for overflowing intermediate operations in `mul_div_down` and
///  `mul_div_up`.
///
/// Each of the functions is fuzz tested against the Solidity implementation to
/// ensure that the behavior is identical given values bounded by both the
/// Solidity and Rust limits.
#[derive(PartialOrd, Ord, Clone, Copy)]
pub struct FixedPoint<T: FixedPointValue> {
    sign: FixedPointSign,
    value: T,
    decimals: u8,
}

impl<T: FixedPointValue> Default for FixedPoint<T> {
    fn default() -> Self {
        Self {
            sign: FixedPointSign::Positive,
            value: T::default(),
            decimals: T::MAX_DECIMALS,
        }
    }
}

impl<T: FixedPointValue> FixedPoint<T> {
    const MAX: Self = Self {
        sign: FixedPointSign::Positive,
        value: T::MAX,
        decimals: T::MAX_DECIMALS,
    };

    const MIN: Self = Self {
        sign: FixedPointSign::Negative,
        value: T::MIN,
        decimals: T::MAX_DECIMALS,
    };

    const MAX_DECIMALS: u8 = T::MAX_DECIMALS;

    // Constructors //

    /// The primary constructor for `FixedPoint`. All instances should be
    /// created using this method to ensure consistent behavior.
    pub fn new<S, V>(sign: S, value: V) -> Result<Self>
    where
        S: Into<FixedPointSign>,
        V: Into<T>,
    {
        let sign = sign.into();
        let value = value.into();

        if (sign == FixedPointSign::Positive && value > T::MAX)
            || (sign == FixedPointSign::Negative && value < T::MIN)
        {
            bail!(
                r#"value {value:?} is out of FixedPoint range:
    min: {min:?}
    max: {max:?}"#,
                min = T::MIN,
                max = T::MAX,
            );
        }

        Ok(Self {
            sign,
            value,
            // TODO: Add support for variable decimals.
            decimals: Self::MAX_DECIMALS,
        })
    }

    pub fn from_raw<V: Into<T>>(value: V) -> Result<Self> {
        let value: T = value.into();
        Self::new(!value.is_negative(), value)
    }

    pub fn zero() -> Self {
        Self::from_raw(0).unwrap()
    }

    pub fn from_dec_str(s: &str) -> Result<Self> {
        if s.starts_with('-') {
            return Self::new(
                FixedPointSign::Negative,
                T::from_u256(u256_from_str(&s[1..])?)?,
            );
        }
        Self::from_raw(T::from_u256(u256_from_str(s)?)?)
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
        if self.is_zero() {
            return FixedPointSign::Positive;
        }
        self.sign
    }

    // Predicates //

    pub fn is_positive(&self) -> bool {
        self.sign() == FixedPointSign::Positive
    }

    pub fn is_negative(&self) -> bool {
        !self.is_positive()
    }

    pub fn is_zero(&self) -> bool {
        self.raw() == 0.into()
    }

    // Conversions //

    pub fn to_u256(&self) -> Result<U256> {
        if self.is_negative() {
            bail!(
                "cannot convert negative FixedPoint {} to U256",
                self.to_scaled_string(),
            );
        }
        self.raw().to_u256()
    }

    pub fn to_i256(&self) -> Result<I256> {
        let u = self.raw().to_u256()?;
        if u > I256::MAX.unsigned_abs() {
            bail!(
                "FixedPoint {} is too large to convert to I256",
                self.to_scaled_string()
            );
        }
        I256::checked_from_sign_and_abs(self.sign().into(), u).ok_or(eyre!(
            "failed to convert FixedPoint {} to I256",
            self.to_scaled_string()
        ))
    }

    pub fn to_u128(&self) -> Result<u128> {
        if self.is_negative() {
            bail!(
                "cannot convert negative FixedPoint {} to u128",
                self.to_scaled_string()
            );
        }
        self.raw().to_u128()
    }

    pub fn to_i128(&self) -> Result<i128> {
        let mut i = self.raw().to_i128()?;
        if self.is_negative() && i.is_positive() {
            i = -i
        }
        Ok(i)
    }

    // Math //

    /// `FixedPoint(1.0)` with the same scale as this fixed point number.
    ///
    /// # Panics
    ///
    /// Panics if `10` to the power of `self.decimals()` overflows `T`.
    pub fn one(&self) -> Self {
        let u256 = uint256!(10).pow(self.decimals().into());
        Self::from_raw(T::from_u256(u256).unwrap()).unwrap()
    }

    pub fn checked_add(&self, other: Self) -> Option<Self> {
        if self.sign() == other.sign() {
            let lhs = self.raw().abs();
            let rhs = other.raw().abs();
            let max_rhs = match self.sign() {
                FixedPointSign::Positive => T::MAX - lhs,
                FixedPointSign::Negative => T::MIN + lhs,
            };
            if rhs > max_rhs {
                return None;
            }

            Self::new(self.sign(), lhs + rhs).ok()
        } else {
            Self::new(self.max(&other).sign(), self.abs_diff(other).raw()).ok()
        }
    }

    pub fn saturating_add(&self, other: Self) -> Self {
        self.checked_add(other)
            .unwrap_or(Self::saturate_sign(self.sign()))
    }

    pub fn try_add(&self, other: Self) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            eyre!(
                "FixedPoint operation overflowed: {} + {}",
                self.to_scaled_string(),
                other.to_scaled_string()
            )
        })
    }

    pub fn checked_sub(&self, other: Self) -> Option<Self> {
        if self.sign() == other.sign() {
            Self::new(
                self.sign().flip_if(other.raw() > self.raw()),
                self.abs_diff(other).raw(),
            )
            .ok()
        } else {
            Self::new(self.sign(), self.abs().checked_add(other.abs())?.raw()).ok()
        }
    }

    pub fn saturating_sub(&self, other: Self) -> Self {
        self.checked_sub(other)
            .unwrap_or(Self::saturate_sign(self.sign()))
    }

    pub fn try_sub(&self, other: Self) -> Result<Self> {
        self.checked_sub(other).ok_or_else(|| {
            eyre!(
                "FixedPoint operation overflowed: {} - {}",
                self.to_scaled_string(),
                other.to_scaled_string()
            )
        })
    }

    pub fn abs(&self) -> Self {
        Self::from_raw(self.raw().abs()).unwrap()
    }

    pub fn abs_diff(&self, other: Self) -> Self {
        if self > &other {
            self.try_sub(other).unwrap()
        } else {
            other.try_sub(*self).unwrap()
        }
    }

    pub fn mul_div_down(&self, other: Self, divisor: Self) -> Result<Self> {
        if divisor.is_zero() {
            bail!("cannot divide by zero");
        }
        let u512 = self
            .abs()
            .to_u256()?
            .full_mul(other.abs().to_u256()?.into())
            .div(divisor.abs().to_u256()?);
        Self::new(
            self.sign().flip_if(other.sign() != divisor.sign()),
            T::from_u256(U256::try_from(u512).or_else(|_| {
                bail!(
                    "FixedPoint operation overflowed: {} * {} / {}",
                    self.to_scaled_string(),
                    other.to_scaled_string(),
                    divisor.to_scaled_string(),
                )
            })?)?,
        )
    }

    pub fn mul_div_up(&self, other: Self, divisor: Self) -> Result<Self> {
        if divisor.is_zero() {
            bail!("cannot divide by zero");
        }
        let (u512, remainder) = self
            .abs()
            .to_u256()?
            .full_mul(other.abs().to_u256()?.into())
            .div_mod(divisor.abs().to_u256()?.into());
        let rounded_u512 = u512 + (remainder.gt(&U512::zero()) as u128);
        Self::new(
            self.sign().flip_if(other.sign() != divisor.sign()),
            T::from_u256(U256::try_from(rounded_u512).or_else(|_| {
                bail!(
                    "FixedPoint operation overflowed: {} * {} / {}",
                    self.to_scaled_string(),
                    other.to_scaled_string(),
                    divisor.to_scaled_string()
                )
            })?)?,
        )
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
        Self::new(sign, T::from_u256(abs)?)
    }

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

impl<T: FixedPointValue> PartialEq for FixedPoint<T> {
    fn eq(&self, other: &Self) -> bool {
        self.sign() == other.sign() && self.raw().abs() == other.raw().abs()
    }
}
impl<T: FixedPointValue> Eq for FixedPoint<T> {}

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

macro_rules! impl_from_raw {
    ($($t:ty),*) => {
        $(
            impl<T: FixedPointValue + From<$t>> From<$t> for FixedPoint<T> {
                fn from(u: $t) -> Self {
                    Self::from_raw(u).unwrap()
                }
            }
        )*
    };
}

impl_from_raw!(U256, u128, u64, u32, [u8; 32], [u64; 4]);

macro_rules! impl_from_signum_raw {
    ($($t:ty),*) => {
        $(
            impl<T: FixedPointValue + From<$t>> From<$t> for FixedPoint<T> {
                fn from(i: $t) -> Self {
                    Self::new(i.signum() as i8, i).unwrap()
                }
            }
        )*
    };
}

impl_from_signum_raw!(i128, i64, i32);

impl<T> From<I256> for FixedPoint<T>
where
    T: FixedPointValue + From<I256>,
{
    fn from(i: I256) -> Self {
        Self::new(i.sign(), i).unwrap()
    }
}

macro_rules! impl_mapped_try_into {
    ($($t:ty => $fn:ident),*) => {
        $(
            impl<T: $crate::FixedPointValue> TryFrom<$crate::FixedPoint<T>> for $t {
                type Error = eyre::ErrReport;

                fn try_from(f: $crate::FixedPoint<T>) -> eyre::Result<Self> {
                    f.$fn()
                }
            }
        )*
    };
    () => {};
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
        Self::new(self.sign().flip(), self.raw().flip_sign()).unwrap()
    }
}

macro_rules! impl_mapped_operator {
    ($($trait:ident => $fn:ident),*) => {
        $(
            paste::paste! {

                impl<T: $crate::FixedPointValue> std::ops::$trait for $crate::FixedPoint<T> {
                    type Output = Self;

                    fn [<$trait:lower>](self, other: Self) -> Self::Output {
                        self.$fn(other).unwrap()
                    }
                }

                impl<T: $crate::FixedPointValue> std::ops::[<$trait Assign>] for $crate::FixedPoint<T> {
                    fn [<$trait:lower _assign>](&mut self, other: Self) {
                        *self = self.[<$trait:lower>](other);
                    }
                }
            }
        )*
    };
}

impl_mapped_operator!(
    Add => try_add,
    Sub => try_sub,
    Mul => mul_down,
    Div => div_down
);

// Sampling //

// impl<T: FixedPointValue> Distribution<FixedPoint<T>> for Standard {
//     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> FixedPoint<T> {
//         FixedPoint::new(
//             rng.gen_bool(0.5),
//             T::from_u256(rng.gen::<[u8; 32]>().into()).unwrap(),
//         )
//         .unwrap()
//     }
// }

impl<T: FixedPointValue> Distribution<FixedPoint<T>> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> FixedPoint<T> {
        let u: U256 = rng.gen::<[u8; 32]>().into();
        println!("sample--\nu: {}", u);
        FixedPoint::new(
            rng.gen_bool(0.5),
            T::from_u256(u % (T::MAX.to_u256().unwrap() + U256::from(1))).unwrap(),
        )
        .unwrap()
    }
}

pub struct UniformFixedPoint<T: FixedPointValue> {
    low: FixedPoint<T>,
    high: FixedPoint<T>,
}

impl<T: FixedPointValue> SampleUniform for FixedPoint<T> {
    type Sampler = UniformFixedPoint<T>;
}

impl<T: FixedPointValue> UniformSampler for UniformFixedPoint<T> {
    type X = FixedPoint<T>;

    #[inline]
    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();
        if low >= high {
            panic!("UniformFixedPoint::new called with invalid range");
        }
        UniformFixedPoint { low, high }
    }

    #[inline]
    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();
        if low > high {
            panic!("UniformFixedPoint::new called with invalid range");
        }
        UniformFixedPoint::new(low, high + FixedPoint::<T>::from(1))
    }

    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> FixedPoint<T> {
        println!("sample(inline)--\nlow: {}, high: {}", self.low, self.high);

        let range = self.high.abs_diff(self.low);
        if range.is_zero() {
            return self.low;
        }

        let max_value = T::MAX;
        let raw_rand = U256::from(rng.gen::<[u8; 32]>())
            % (max_value
                .to_u256()
                .unwrap_or_else(|_| panic!("Failed to convert max_value to U256"))
                + U256::from(1));

        let scaled_rand = range
            .mul_div_down(
                FixedPoint::<T>::from_raw(
                    T::from_u256(raw_rand)
                        .unwrap_or_else(|_| panic!("Failed to convert raw_rand to T")),
                )
                .unwrap_or_else(|_| panic!("Failed to convert raw_rand to FixedPoint")),
                FixedPoint::<T>::from_raw(max_value)
                    .unwrap_or_else(|_| panic!("Failed to convert max_value to FixedPoint")),
            )
            .unwrap_or_else(|_| panic!("Failed to scale random value"));

        self.low + scaled_rand
    }
}

// #[cfg(test)]
// mod tests {
//     use std::u128;

//     use ethers::signers::Signer;
//     use hyperdrive_wrappers::wrappers::mock_fixed_point_math::MockFixedPointMath;
//     use rand::thread_rng;
//     use test_utils::{chain::Chain, constants::DEPLOYER};

//     use super::*;
//     use crate::uint256;

//     fn max_sol_divisible() -> U256 {
//         U256::MAX / uint256!(1e18)
//     }

//     #[test]
//     fn test_fixed_point_fmt() {
//         // fmt::Debug
//         assert_eq!(
//             format!("{:?}", fixed!(1)),
//             "FixedPoint(0.000000000000000001)"
//         );
//         assert_eq!(
//             format!("{:?}", fixed!(1.23456e18)),
//             "FixedPoint(1.234560000000000000)"
//         );
//         assert_eq!(
//             format!("{:?}", fixed!(50_000.234_56e18)),
//             "FixedPoint(50000.234560000000000000)"
//         );

//         // fmt::Display
//         assert_eq!(format!("{}", fixed!(1)), "0.000000000000000001");
//         assert_eq!(format!("{}", fixed!(1.23456e18)), "1.234560000000000000");
//         assert_eq!(
//             format!("{}", fixed!(50_000.234_56e18)),
//             "50000.234560000000000000"
//         );
//     }

//     #[test]
//     fn test_mul_div_down_failure() {
//         // Ensure that division by zero fails.
//         assert!(fixed!(1e18).mul_div_down(fixed!(1e18), 0.into()).is_err());
//     }

//     #[tokio::test]
//     async fn fuzz_mul_div_down() -> Result<()> {
//         let chain = Chain::connect(None, None).await?;
//         chain.deal(DEPLOYER.address(), uint256!(100_000e18)).await?;
//         let client = chain.client(DEPLOYER.clone()).await?;
//         let mock_fixed_point_math = MockFixedPointMath::deploy(client, ())?.send().await?;

//         println!(
//             "Deployed mock contract at {}",
//             mock_fixed_point_math.address()
//         );

//         // Fuzz the rust and solidity implementations against each other.
//         let mut rng = thread_rng();
//         for _ in 0..10_000 {
//             let a = rng.gen_range(fixed!(0)..=Self::from(u128::MAX));
//             let b = rng.gen_range(fixed!(0)..Self::from(u128::MAX));
//             let c = rng.gen::<FixedPoint>();
//             let actual = a.mul_div_down(b, c);
//             match mock_fixed_point_math
//                 .mul_div_down(
//                     a.abs().try_into()?,
//                     b.abs().try_into()?,
//                     c.abs().try_into()?,
//                 )
//                 .call()
//                 .await
//             {
//                 Ok(expected) => {
//                     assert_eq!(actual?.abs(), Self::from(expected))
//                 }
//                 Err(_) => assert!(actual.is_err()),
//             }
//         }

//         Ok(())
//     }

//     #[test]
//     fn test_mul_div_up_failure() {
//         // Ensure that division by zero fails.
//         assert!(fixed!(1e18).mul_div_up(fixed!(1e18), 0.into()).is_err());
//     }

//     #[tokio::test]
//     async fn fuzz_mul_div_up() -> Result<()> {
//         let chain = Chain::connect(None, None).await?;
//         chain.deal(DEPLOYER.address(), uint256!(100_000e18)).await?;
//         let client = chain.client(DEPLOYER.clone()).await?;
//         let mock_fixed_point_math = MockFixedPointMath::deploy(client, ())?.send().await?;

//         // Fuzz the rust and solidity implementations against each other.
//         let mut rng = thread_rng();
//         for _ in 0..10_000 {
//             let a = rng.gen_range(fixed!(0)..=Self::from(u128::MAX));
//             let b = rng.gen_range(fixed!(0)..Self::from(u128::MAX));
//             let c = rng.gen::<FixedPoint>();
//             let actual = a.mul_div_up(b, c);
//             match mock_fixed_point_math
//                 .mul_div_up(
//                     a.abs().try_into()?,
//                     b.abs().try_into()?,
//                     c.abs().try_into()?,
//                 )
//                 .call()
//                 .await
//             {
//                 Ok(expected) => assert_eq!(actual?.abs(), Self::from(expected)),
//                 Err(_) => assert!(actual.is_err()),
//             }
//         }

//         Ok(())
//     }

//     #[tokio::test]
//     async fn fuzz_mul_down() -> Result<()> {
//         let chain = Chain::connect(None, None).await?;
//         chain.deal(DEPLOYER.address(), uint256!(100_000e18)).await?;
//         let client = chain.client(DEPLOYER.clone()).await?;
//         let mock_fixed_point_math = MockFixedPointMath::deploy(client, ())?.send().await?;

//         // Fuzz the rust and solidity implementations against each other.
//         let mut rng = thread_rng();
//         for _ in 0..10_000 {
//             let a = rng.gen_range(fixed!(0)..=Self::from(u128::MAX));
//             let b = rng.gen_range(fixed!(0)..Self::from(u128::MAX));
//             let actual = a.mul_down(b);

//             match mock_fixed_point_math
//                 .mul_down(a.abs().try_into()?, b.abs().try_into()?)
//                 .call()
//                 .await
//             {
//                 Ok(expected) => assert_eq!(actual?.abs(), Self::from(expected)),
//                 Err(_) => assert!(actual.is_err()),
//             }
//         }

//         Ok(())
//     }

//     #[tokio::test]
//     async fn fuzz_mul_up() -> Result<()> {
//         let chain = Chain::connect(None, None).await?;
//         chain.deal(DEPLOYER.address(), uint256!(100_000e18)).await?;
//         let client = chain.client(DEPLOYER.clone()).await?;
//         let mock_fixed_point_math = MockFixedPointMath::deploy(client, ())?.send().await?;

//         // Fuzz the rust and solidity implementations against each other.
//         let mut rng = thread_rng();
//         for _ in 0..10_000 {
//             let a = rng.gen_range(fixed!(0)..=Self::from(u128::MAX));
//             let b = rng.gen_range(fixed!(0)..Self::from(u128::MAX));
//             let actual = a.mul_up(b);
//             match mock_fixed_point_math
//                 .mul_up(a.abs().try_into()?, b.abs().try_into()?)
//                 .call()
//                 .await
//             {
//                 Ok(expected) => assert_eq!(actual?.abs(), Self::from(expected)),
//                 Err(_) => assert!(actual.is_err()),
//             }
//         }

//         Ok(())
//     }

//     #[test]
//     fn test_div_down_failure() {
//         // Ensure that division by zero fails.
//         assert!(fixed!(1e18).div_down(0.into()).is_err());
//     }

//     #[tokio::test]
//     async fn fuzz_div_down() -> Result<()> {
//         let chain = Chain::connect(None, None).await?;
//         chain.deal(DEPLOYER.address(), uint256!(100_000e18)).await?;
//         let client = chain.client(DEPLOYER.clone()).await?;
//         let mock_fixed_point_math = MockFixedPointMath::deploy(client, ())?.send().await?;

//         // Fuzz the rust and solidity implementations against each other.
//         let mut rng = thread_rng();
//         for _ in 0..10_000 {
//             let a = rng.gen_range(fixed!(0)..=Self::from(max_sol_divisible()));
//             let b = rng.gen_range(fixed!(0)..Self::from(max_sol_divisible()));
//             let actual = a.div_down(b);
//             match mock_fixed_point_math
//                 .div_down(a.abs().try_into()?, b.abs().try_into()?)
//                 .call()
//                 .await
//             {
//                 Ok(expected) => assert_eq!(actual?.abs(), Self::from(expected)),
//                 Err(_) => assert!(actual.is_err()),
//             }
//         }

//         Ok(())
//     }

//     #[test]
//     fn test_div_up_failure() {
//         // Ensure that division by zero fails.
//         assert!(fixed!(1e18).div_up(0.into()).is_err());
//     }

//     #[tokio::test]
//     async fn fuzz_div_up() -> Result<()> {
//         let chain = Chain::connect(None, None).await?;
//         chain.deal(DEPLOYER.address(), uint256!(100_000e18)).await?;
//         let client = chain.client(DEPLOYER.clone()).await?;
//         let mock_fixed_point_math = MockFixedPointMath::deploy(client, ())?.send().await?;

//         // Fuzz the rust and solidity implementations against each other.
//         let mut rng = thread_rng();
//         for _ in 0..10_000 {
//             let a = rng.gen_range(fixed!(0)..=Self::from(max_sol_divisible()));
//             let b = rng.gen_range(fixed!(0)..Self::from(max_sol_divisible()));
//             let actual = a.div_up(b);
//             match mock_fixed_point_math
//                 .div_up(a.abs().try_into()?, b.abs().try_into()?)
//                 .call()
//                 .await
//             {
//                 Ok(expected) => assert_eq!(actual?.abs(), Self::from(expected)),
//                 Err(_) => assert!(actual.is_err()),
//             }
//         }

//         Ok(())
//     }

//     #[tokio::test]
//     async fn fuzz_pow_narrow() -> Result<()> {
//         let chain = Chain::connect(None, None).await?;
//         chain.deal(DEPLOYER.address(), uint256!(100_000e18)).await?;
//         let client = chain.client(DEPLOYER.clone()).await?;
//         let mock_fixed_point_math = MockFixedPointMath::deploy(client, ())?.send().await?;

//         // Fuzz the rust and solidity implementations against each other.
//         let mut rng = thread_rng();
//         for _ in 0..10_000 {
//             let x = rng.gen_range(fixed!(0)..=fixed!(1e18));
//             let y = rng.gen_range(fixed!(0)..=fixed!(1e18)).abs();
//             let actual = x.pow(y);
//             match mock_fixed_point_math
//                 .pow(x.abs().try_into()?, y.try_into()?)
//                 .call()
//                 .await
//             {
//                 Ok(expected) => {
//                     assert_eq!(actual?, Self::from(expected));
//                 }
//                 Err(_) => assert!(actual.is_err()),
//             }
//         }

//         Ok(())
//     }

//     #[tokio::test]
//     async fn fuzz_pow() -> Result<()> {
//         let chain = Chain::connect(None, None).await?;
//         chain.deal(DEPLOYER.address(), uint256!(100_000e18)).await?;
//         let client = chain.client(DEPLOYER.clone()).await?;
//         let mock_fixed_point_math = MockFixedPointMath::deploy(client, ())?.send().await?;

//         // Fuzz the rust and solidity implementations against each other.
//         let mut rng = thread_rng();
//         for _ in 0..10_000 {
//             let x = rng.gen::<FixedPoint>();
//             let y = rng.gen::<FixedPoint>();
//             let actual = x.pow(y);
//             match mock_fixed_point_math
//                 .pow(x.abs().try_into()?, y.abs().try_into()?)
//                 .call()
//                 .await
//             {
//                 Ok(expected) => {
//                     assert_eq!(actual?, Self::from(expected))
//                 }
//                 Err(_) => assert!(actual.is_err()),
//             }
//         }

//         Ok(())
//     }
// }
