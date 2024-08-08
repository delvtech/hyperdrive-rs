pub mod conversions;
pub mod fixed_point;
pub mod macros;
pub mod operators;
pub mod rng;
pub mod sign;
pub mod utils;
pub mod value;

pub use conversions::*;
pub use fixed_point::*;
pub use rng::*;
pub use sign::*;
pub use utils::*;
pub use value::*;

pub mod prelude {
    pub use super::{Fixed, FixedPoint, FixedPointSign};
}

// Library level tests which use the full API.
#[cfg(test)]
mod tests {
    use std::{panic, u128};

    use ethers::{signers::Signer, types::U256};
    use eyre::Result;
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
