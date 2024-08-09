use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use ethers::types::{U256, U512};
use eyre::{bail, Result};

use crate::{exp, ln, FixedPoint, FixedPointValue};

impl<T: FixedPointValue> FixedPoint<T> {
    /// Computes the absolute value of self.
    ///
    /// # Panics
    ///
    /// If the absolute value of self overflows `T`, e.g., if self is the
    /// minimum value of a signed integer.
    pub fn abs(&self) -> Self {
        self.raw().abs().into()
    }

    /// Computes the absolute value of self as a `U256` to avoid overflow.
    pub fn unsigned_abs(&self) -> FixedPoint<U256> {
        self.raw().unsigned_abs().into()
    }

    pub fn abs_diff(&self, other: FixedPoint<impl FixedPointValue>) -> FixedPoint<U256> {
        let abs_self = self.unsigned_abs();
        let abs_other = other.unsigned_abs();
        if self.sign() != other.sign() {
            abs_self + abs_other
        } else if abs_self > abs_other {
            abs_self - abs_other
        } else {
            abs_other - abs_self
        }
    }

    pub fn mul_div_down(
        &self,
        other: FixedPoint<impl FixedPointValue>,
        divisor: FixedPoint<impl FixedPointValue>,
    ) -> Result<Self> {
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
        Self::try_from(raw)
    }

    pub fn mul_div_up(
        &self,
        other: FixedPoint<impl FixedPointValue>,
        divisor: FixedPoint<impl FixedPointValue>,
    ) -> Result<Self> {
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
        Self::try_from(raw)
    }

    pub fn mul_down(&self, other: FixedPoint<impl FixedPointValue>) -> Result<Self> {
        self.mul_div_down(other, self.one())
    }

    pub fn mul_up(&self, other: FixedPoint<impl FixedPointValue>) -> Result<Self> {
        self.mul_div_up(other, self.one())
    }

    pub fn div_down(self, other: FixedPoint<impl FixedPointValue>) -> Result<Self> {
        self.mul_div_down(self.one(), other)
    }

    pub fn div_up(self, other: FixedPoint<impl FixedPointValue>) -> Result<Self> {
        self.mul_div_up(self.one(), other)
    }

    pub fn pow(self, y: FixedPoint<impl FixedPointValue>) -> Result<Self> {
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
        Self::try_from(raw)
    }
}

impl<T: FixedPointValue> Neg for FixedPoint<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::from(self.raw().flip_sign())
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
                        Self::from(self.raw().[<$trait:lower>](other.raw()))
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

// impl<T: FixedPointValue, U: FixedPointValue> Add<FixedPoint<U>> for FixedPoint<T> {
//     type Output = Self;
//     fn add(self, other: FixedPoint<U>) -> Self::Output {

//         Self::from(self.raw().add(other.raw()))
//     }
// }

// impl<T: FixedPointValue> Add<U256> for FixedPoint<T> {
//     type Output = Self;
//     fn add(self, other: U256) -> Self::Output {
//         Self::from(self.raw().unsigned_abs().add(other))
//     }
// }

// Forward these operators to the underlying `FixedPointValue`.
impl_forwarded_operator!(Add, Sub, Rem);

#[cfg(test)]
mod tests {
    use std::{panic, u128};

    use ethers::{
        signers::Signer,
        types::{I256, U256},
    };
    use eyre::Result;
    use hyperdrive_wrappers::wrappers::mock_fixed_point_math::MockFixedPointMath;
    use rand::{thread_rng, Rng};
    use test_utils::{chain::Chain, constants::DEPLOYER};

    use super::*;
    use crate::{fixed, fixed_i128, fixed_u128, uint256};

    /// The maximum number that can be divided by another in the Solidity
    /// implementation.
    fn max_sol_numerator() -> FixedPoint<U256> {
        (U256::MAX / uint256!(1e18)).into()
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
            .mul_div_down(fixed_u128!(1e18), fixed_u128!(0))
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
                Ok(expected) => assert_eq!(actual.unwrap(), expected.into()),
                Err(_) => assert!(actual.is_err()),
            }
        }

        Ok(())
    }

    #[test]
    fn test_mul_div_up_failure() {
        // Ensure that division by zero fails.
        assert!(fixed_u128!(1e18)
            .mul_div_up(fixed_u128!(1e18), fixed_u128!(0))
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
                Ok(expected) => assert_eq!(actual.unwrap(), expected.into()),
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
                Ok(expected) => assert_eq!(actual.unwrap(), expected.into()),
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
                Ok(expected) => assert_eq!(actual.unwrap(), expected.into()),
                Err(_) => assert!(actual.is_err()),
            }
        }

        Ok(())
    }

    #[test]
    fn test_div_down_failure() {
        // Ensure that division by zero fails.
        assert!(fixed_u128!(1e18).div_down(fixed_u128!(0)).is_err());
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
                Ok(expected) => assert_eq!(actual.unwrap(), expected.into()),
                Err(_) => assert!(actual.is_err()),
            }
        }

        Ok(())
    }

    #[test]
    fn test_div_up_failure() {
        // Ensure that division by zero fails.
        assert!(fixed_u128!(1e18).div_up(fixed_u128!(0)).is_err());
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
                Ok(expected) => assert_eq!(actual.unwrap(), expected.into()),
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
                    assert_eq!(actual.unwrap(), expected.into());
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
                Ok(expected) => assert_eq!(actual.unwrap(), expected.into()),
                Err(_) => assert!(actual.is_err()),
            }
        }

        Ok(())
    }

    #[test]
    fn foo() -> Result<()> {
        // let new_a = a
        //     .as_fixed()
        //     .mul_div_down(a_fp, share_reserves)?
        //     .abs()
        //     .to_i256()?;

        // let new_a = if a.is_positive() {
        //     let a_fp = a.as_fixed(); // Fixed(100)
        //     I256::try_from(new_share_reserves.mul_div_down(a_fp, share_reserves)?)?
        // } else {
        //     let a_fp = FixedPoint::try_from(-a)?; // Fixed(100)
        //     -I256::try_from(new_share_reserves.mul_div_up(a_fp, share_reserves)?)?
        // };

        Ok(())
    }
}
