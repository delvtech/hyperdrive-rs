use ethers::types::{I256, U256};
use eyre::{eyre, Result};
use fixedpointmath::{fixed, FixedPoint};

use crate::calculate_effective_share_reserves;

pub trait YieldSpace {
    /// The effective share reserves.
    fn ze(&self) -> Result<FixedPoint<U256>> {
        calculate_effective_share_reserves(self.z(), self.zeta())
    }

    /// The share reserves.
    fn z(&self) -> FixedPoint<U256>;

    /// The share adjustment.
    fn zeta(&self) -> I256;

    /// The bond reserves.
    fn y(&self) -> FixedPoint<U256>;

    /// The share price.
    fn c(&self) -> FixedPoint<U256>;

    /// The initial vault share price.
    fn mu(&self) -> FixedPoint<U256>;

    /// The YieldSpace time parameter.
    fn t(&self) -> FixedPoint<U256>;

    // The current spot price ignoring slippage, rounded down.
    fn calculate_spot_price_down(&self) -> Result<FixedPoint<U256>> {
        if self.y() <= fixed!(0) {
            return Err(eyre!("expected y={} > 0", self.y()));
        }
        ((self.mu() * self.ze()?) / self.y()).pow(self.t())
    }

    // The current spot price ignoring slippage, rounded up.
    fn calculate_spot_price_up(&self) -> Result<FixedPoint<U256>> {
        if self.y() <= fixed!(0) {
            return Err(eyre!("expected y={} > 0", self.y()));
        }
        ((self.mu().mul_up(self.ze()?)).div_up(self.y())).pow(self.t())
    }

    /// Calculates the amount of bonds a user will receive from the pool by
    /// providing a specified amount of shares. We underestimate the amount of
    /// bonds out to prevent sandwiches.
    fn calculate_bonds_out_given_shares_in_down(
        &self,
        dz: FixedPoint<U256>,
    ) -> Result<FixedPoint<U256>> {
        // NOTE: We round k up to make the rhs of the equation larger.
        //
        // k = (c / µ) * (µ * ze)^(1 - t) + y^(1 - t)
        let k = self.k_up()?;

        // NOTE: We round z down to make the rhs of the equation larger.
        //
        // (µ * (ze + dz))^(1 - t)
        let mut ze = (self.mu() * (self.ze()? + dz)).pow(fixed!(1e18) - self.t())?;
        // (c / µ) * (µ * (ze + dz))^(1 - t)
        ze = self.c().mul_div_down(ze, self.mu());

        // NOTE: We round _y up to make the rhs of the equation larger.
        //
        // k - (c / µ) * (µ * (ze + dz))^(1 - t))^(1 / (1 - t)))
        if k < ze {
            return Err(eyre!("expected k={} >= ze={}", k, ze));
        }
        let mut y = k - ze;
        if y >= fixed!(1e18) {
            // Rounding up the exponent results in a larger result.
            y = y.pow(fixed!(1e18).div_up(fixed!(1e18) - self.t()))?;
        } else {
            // Rounding down the exponent results in a larger result.
            y = y.pow(fixed!(1e18) / (fixed!(1e18) - self.t()))?;
        }

        // Δy = y - (k - (c / µ) * (µ * (z + dz))^(1 - t))^(1 / (1 - t)))
        if self.y() < y {
            return Err(eyre!("expected y={} >= delta_y={}", self.y(), y));
        }
        Ok(self.y() - y)
    }

    /// Calculates the amount of shares a user must provide the pool to receive
    /// a specified amount of bonds. We overestimate the amount of shares in.
    fn calculate_shares_in_given_bonds_out_up(
        &self,
        dy: FixedPoint<U256>,
    ) -> Result<FixedPoint<U256>> {
        // NOTE: We round k up to make the lhs of the equation larger.
        //
        // k = (c / µ) * (µ * z)^(1 - t) + y^(1 - t)
        let k = self.k_up()?;

        // (y - dy)^(1 - t)
        if self.y() < dy {
            return Err(eyre!(
                "calculate_shares_in_given_bonds_out_up: y = {} < {} = dy",
                self.y(),
                dy,
            ));
        }
        let y = (self.y() - dy).pow(fixed!(1e18) - self.t())?;

        // NOTE: We round _z up to make the lhs of the equation larger.
        //
        // ((k - (y - dy)^(1 - t) ) / (c / µ))^(1 / (1 - t))
        if k < y {
            return Err(eyre!(
                "calculate_shares_in_given_bonds_out_up: k = {} < {} = y",
                k,
                y,
            ));
        }
        let mut _z = (k - y).mul_div_up(self.mu(), self.c());
        if _z >= fixed!(1e18) {
            // Rounding up the exponent results in a larger result.
            _z = _z.pow(fixed!(1e18).div_up(fixed!(1e18) - self.t()))?;
        } else {
            // Rounding down the exponent results in a larger result.
            _z = _z.pow(fixed!(1e18) / (fixed!(1e18) - self.t()))?;
        }
        // ((k - (y - dy)^(1 - t) ) / (c / µ))^(1 / (1 - t))) / µ
        _z = _z.div_up(self.mu());

        // Δz = (((k - (y - dy)^(1 - t) ) / (c / µ))^(1 / (1 - t))) / µ - ze
        if _z < self.ze()? {
            return Err(eyre!(
                "calculate_shares_in_given_bonds_out_up: _z = {} < {} = ze",
                _z,
                self.ze()?,
            ));
        }
        Ok(_z - self.ze()?)
    }

    /// Calculates the amount of shares a user must provide the pool to receive
    /// a specified amount of bonds. We underestimate the amount of shares in.
    fn calculate_shares_in_given_bonds_out_down(
        &self,
        dy: FixedPoint<U256>,
    ) -> Result<FixedPoint<U256>> {
        // NOTE: We round k down to make the lhs of the equation smaller.
        //
        // k = (c / µ) * (µ * ze)^(1 - t) + y^(1 - t)
        let k = self.k_down()?;

        // (y - dy)^(1 - t)
        let y = (self.y() - dy).pow(fixed!(1e18) - self.t())?;

        // NOTE: We round _ze down to make the lhs of the equation smaller.
        //
        // ((k - (y - dy)^(1 - t) ) / (c / µ))^(1 / (1 - t))
        let mut ze = (k - y).mul_div_down(self.mu(), self.c());
        if ze >= fixed!(1e18) {
            // Rounding down the exponent results in a smaller result.
            ze = ze.pow(fixed!(1e18) / (fixed!(1e18) - self.t()))?;
        } else {
            // Rounding up the exponent results in a smaller result.
            ze = ze.pow(fixed!(1e18).div_up(fixed!(1e18) - self.t()))?;
        }
        // ((k - (y - dy)^(1 - t) ) / (c / µ))^(1 / (1 - t))) / µ
        ze /= self.mu();

        // Δz = (((k - (y - dy)^(1 - t) ) / (c / µ))^(1 / (1 - t))) / µ - ze
        Ok(ze - self.ze()?)
    }

    /// Calculates the amount of shares a user will receive from the pool by
    /// providing a specified amount of bonds. This function reverts if an
    /// integer overflow or underflow occurs. We underestimate the amount of
    /// shares out.
    fn calculate_shares_out_given_bonds_in_down(
        &self,
        dy: FixedPoint<U256>,
    ) -> Result<FixedPoint<U256>> {
        // NOTE: We round k up to make the rhs of the equation larger.
        //
        // k = (c / µ) * (µ * ze)^(1 - t) + y^(1 - t)
        let k = self.k_up()?;

        // (y + dy)^(1 - t)
        let y = (self.y() + dy).pow(fixed!(1e18) - self.t())?;

        // If k is less than y, we return with a failure flag.
        if k < y {
            return Err(eyre!(
                "calculate_shares_out_given_bonds_in_down: k = {} < {} = y",
                k,
                y
            ));
        }

        // NOTE: We round _ze up to make the rhs of the equation larger.
        //
        // ((k - (y + dy)^(1 - t)) / (c / µ))^(1 / (1 - t)))
        let mut ze = (k - y).mul_div_up(self.mu(), self.c());
        if ze >= fixed!(1e18) {
            // Rounding the exponent up results in a larger outcome.
            ze = ze.pow(fixed!(1e18).div_up(fixed!(1e18) - self.t()))?;
        } else {
            // Rounding the exponent down results in a larger outcome.
            ze = ze.pow(fixed!(1e18) / (fixed!(1e18) - self.t()))?;
        }
        // ((k - (y + dy)^(1 - t) ) / (c / µ))^(1 / (1 - t))) / µ
        ze = ze.div_up(self.mu());

        // Δz = ze - ((k - (y + dy)^(1 - t) ) / (c / µ))^(1 / (1 - t)) / µ
        if self.ze()? > ze {
            Ok(self.ze()? - ze)
        } else {
            Ok(fixed!(0))
        }
    }

    /// Calculates the share payment required to purchase the maximum
    /// amount of bonds from the pool.
    fn calculate_max_buy_shares_in(&self) -> Result<FixedPoint<U256>> {
        // We solve for the maximum buy using the constraint that the pool's
        // spot price can never exceed 1. We do this by noting that a spot price
        // of 1, ((mu * ze) / y) ** tau = 1, implies that mu * ze = y. This
        // simplifies YieldSpace to:
        //
        // k = ((c / mu) + 1) * (mu * ze') ** (1 - tau),
        //
        // This gives us the maximum share reserves of:
        //
        // ze' = (1 / mu) * (k / ((c / mu) + 1)) ** (1 / (1 - tau)).
        let k = self.k_down()?;
        let mut optimal_ze = k.div_down(self.c().div_up(self.mu()) + fixed!(1e18));
        if optimal_ze >= fixed!(1e18) {
            // Rounding the exponent up results in a larger outcome.
            optimal_ze = optimal_ze.pow(fixed!(1e18).div_down(fixed!(1e18) - self.t()))?;
        } else {
            // Rounding the exponent down results in a larger outcome.
            optimal_ze = optimal_ze.pow(fixed!(1e18) / (fixed!(1e18) - self.t()))?;
        }
        optimal_ze = optimal_ze.div_down(self.mu());

        // The optimal trade size is given by dz = ze' - ze. If the calculation
        // underflows, we return a failure flag.
        if optimal_ze >= self.ze()? {
            Ok(optimal_ze - self.ze()?)
        } else {
            Err(eyre!(
                "calculate_max_buy_shares_in: optimal_ze = {} < {} = ze",
                optimal_ze,
                self.ze()?,
            ))
        }
    }

    /// Calculates the maximum amount of bonds that can be purchased with the
    /// specified reserves. We round so that the max buy amount is
    /// underestimated.
    fn calculate_max_buy_bonds_out(&self) -> Result<FixedPoint<U256>> {
        // We solve for the maximum buy using the constraint that the pool's
        // spot price can never exceed 1. We do this by noting that a spot price
        // of 1, (mu * ze) / y ** tau = 1, implies that mu * ze = y. This
        // simplifies YieldSpace to k = ((c / mu) + 1) * y' ** (1 - tau), and
        // gives us the maximum bond reserves of
        // y' = (k / ((c / mu) + 1)) ** (1 / (1 - tau)) and the maximum share
        // reserves of ze' = y/mu.
        let k = self.k_up()?;
        let mut optimal_y = k.div_up(self.c() / self.mu() + fixed!(1e18));
        if optimal_y >= fixed!(1e18) {
            // Rounding the exponent up results in a larger outcome.
            optimal_y = optimal_y.pow(fixed!(1e18).div_up(fixed!(1e18) - self.t()))?;
        } else {
            // Rounding the exponent down results in a larger outcome.
            optimal_y = optimal_y.pow(fixed!(1e18) / (fixed!(1e18) - self.t()))?;
        }

        // The optimal trade size is given by dy = y - y'. If the calculation
        // underflows, we return a failure flag.
        if self.y() >= optimal_y {
            Ok(self.y() - optimal_y)
        } else {
            Err(eyre!(
                "calculate_max_buy_bonds_out: y = {} < {} = optimal_y",
                self.y(),
                optimal_y,
            ))
        }
    }

    /// Calculates the maximum amount of bonds that can be sold with the
    /// specified reserves. We round so that the max sell amount is
    /// underestimated.
    fn calculate_max_sell_bonds_in(&self, mut z_min: FixedPoint<U256>) -> Result<FixedPoint<U256>> {
        // If the share adjustment is negative, the minimum share reserves is
        // given by `z_min - zeta`, which ensures that the share reserves never
        // fall below the minimum share reserves. Otherwise, the minimum share
        // reserves is just zMin.
        if self.zeta() < I256::zero() {
            z_min += FixedPoint::try_from(-self.zeta())?;
        }

        // We solve for the maximum sell using the constraint that the pool's
        // share reserves can never fall below the minimum share reserves zMin.
        // Substituting z = zMin simplifies YieldSpace to
        // k = (c / mu) * (mu * (zMin)) ** (1 - tau) + y' ** (1 - tau), and
        // gives us the maximum bond reserves of
        // y' = (k - (c / mu) * (mu * (zMin)) ** (1 - tau)) ** (1 / (1 - tau)).
        let k = self.k_down()?;
        let mut optimal_y = k - self.c().mul_div_up(
            self.mu().mul_up(z_min).pow(fixed!(1e18) - self.t())?,
            self.mu(),
        );
        if optimal_y >= fixed!(1e18) {
            // Rounding the exponent down results in a smaller outcome.
            optimal_y = optimal_y.pow(fixed!(1e18) / (fixed!(1e18) - self.t()))?;
        } else {
            // Rounding the exponent up results in a smaller outcome.
            optimal_y = optimal_y.pow(fixed!(1e18).div_up(fixed!(1e18) - self.t()))?;
        }

        // The optimal trade size is given by dy = y' - y. If this subtraction
        // will underflow, we return a failure flag.
        if optimal_y >= self.y() {
            Ok(optimal_y - self.y())
        } else {
            Err(eyre!(
                "calculate_max_sell_bonds_in: optimal_y = {} < {} = y",
                optimal_y,
                self.y(),
            ))
        }
    }

    /// Calculates the YieldSpace invariant k. This invariant is given by:
    ///
    /// k = (c / µ) * (µ * ze)^(1 - t) + y^(1 - t)
    ///
    /// This variant of the calculation overestimates the result.
    fn k_up(&self) -> Result<FixedPoint<U256>> {
        Ok(self.c().mul_div_up(
            (self.mu().mul_up(self.ze()?)).pow(fixed!(1e18) - self.t())?,
            self.mu(),
        ) + self.y().pow(fixed!(1e18) - self.t())?)
    }

    /// Calculates the YieldSpace invariant k. This invariant is given by:
    ///
    /// k = (c / µ) * (µ * ze)^(1 - t) + y^(1 - t)
    ///
    /// This variant of the calculation underestimates the result.
    fn k_down(&self) -> Result<FixedPoint<U256>> {
        Ok(self.c().mul_div_down(
            (self.mu() * self.ze()?).pow(fixed!(1e18) - self.t())?,
            self.mu(),
        ) + self.y().pow(fixed!(1e18) - self.t())?)
    }
}

#[cfg(test)]
mod tests {
    use std::panic;

    use hyperdrive_test_utils::{chain::TestChain, constants::FAST_FUZZ_RUNS};
    use rand::{thread_rng, Rng};

    use super::*;
    use crate::State;

    #[tokio::test]
    async fn fuzz_calculate_bonds_out_given_shares_in() -> Result<()> {
        let chain = TestChain::new().await?;

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            let state = rng.gen::<State>();
            let in_ = rng.gen::<FixedPoint<U256>>();
            // We need to catch panics because of overflows.
            let actual =
                panic::catch_unwind(|| state.calculate_bonds_out_given_shares_in_down(in_));
            match chain
                .mock_yield_space_math()
                .calculate_bonds_out_given_shares_in_down(
                    state.ze()?.into(),
                    state.y().into(),
                    in_.into(),
                    (fixed!(1e18) - state.t()).into(),
                    state.c().into(),
                    state.mu().into(),
                )
                .call()
                .await
            {
                Ok(expected) => assert_eq!(actual.unwrap().unwrap(), FixedPoint::from(expected)),
                Err(_) => assert!(actual.is_err() || actual.unwrap().is_err()),
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn fuzz_calculate_shares_in_given_bonds_out_up() -> Result<()> {
        let chain = TestChain::new().await?;

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            let state = rng.gen::<State>();
            let in_ = rng.gen::<FixedPoint<U256>>();
            let actual = state.calculate_shares_in_given_bonds_out_up(in_);
            match chain
                .mock_yield_space_math()
                .calculate_shares_in_given_bonds_out_up(
                    state.ze()?.into(),
                    state.y().into(),
                    in_.into(),
                    (fixed!(1e18) - state.t()).into(),
                    state.c().into(),
                    state.mu().into(),
                )
                .call()
                .await
            {
                Ok(expected) => {
                    assert_eq!(actual.unwrap(), FixedPoint::from(expected));
                }
                Err(_) => assert!(actual.is_err()),
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn fuzz_calculate_shares_in_given_bonds_out_down() -> Result<()> {
        let chain = TestChain::new().await?;

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            let state = rng.gen::<State>();
            let out = rng.gen::<FixedPoint<U256>>();
            let actual =
                panic::catch_unwind(|| state.calculate_shares_in_given_bonds_out_down(out));
            match chain
                .mock_yield_space_math()
                .calculate_shares_in_given_bonds_out_down(
                    state.ze()?.into(),
                    state.y().into(),
                    out.into(),
                    (fixed!(1e18) - state.t()).into(),
                    state.c().into(),
                    state.mu().into(),
                )
                .call()
                .await
            {
                Ok(expected) => {
                    assert_eq!(actual.unwrap().unwrap(), FixedPoint::from(expected));
                }
                Err(_) => assert!(actual.is_err() || actual.unwrap().is_err()),
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn fuzz_calculate_shares_out_given_bonds_in_down() -> Result<()> {
        let chain = TestChain::new().await?;

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            let state = rng.gen::<State>();
            let in_ = rng.gen::<FixedPoint<U256>>();
            let actual = state.calculate_shares_out_given_bonds_in_down(in_);
            match chain
                .mock_yield_space_math()
                .calculate_shares_out_given_bonds_in_down_safe(
                    state.ze()?.into(),
                    state.y().into(),
                    in_.into(),
                    (fixed!(1e18) - state.t()).into(),
                    state.c().into(),
                    state.mu().into(),
                )
                .call()
                .await
            {
                Ok((expected_out, expected_status)) => {
                    assert_eq!(actual.is_ok(), expected_status);
                    assert_eq!(actual.unwrap_or(fixed!(0)), FixedPoint::from(expected_out));
                }
                Err(_) => assert!(actual.is_err()),
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn fuzz_calculate_max_buy_shares_in() -> Result<()> {
        let chain = TestChain::new().await?;

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            let state = rng.gen::<State>();
            let actual = state.calculate_max_buy_shares_in();
            match chain
                .mock_yield_space_math()
                .calculate_max_buy_shares_in_safe(
                    state.ze()?.into(),
                    state.y().into(),
                    (fixed!(1e18) - state.t()).into(),
                    state.c().into(),
                    state.mu().into(),
                )
                .call()
                .await
            {
                Ok((expected_out, expected_status)) => {
                    assert_eq!(actual.is_ok(), expected_status);
                    assert_eq!(actual.unwrap_or(fixed!(0)), FixedPoint::from(expected_out));
                }
                Err(_) => assert!(actual.is_err()),
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn fuzz_calculate_max_buy_bounds_out() -> Result<()> {
        let chain = TestChain::new().await?;

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            let state = rng.gen::<State>();
            let actual = state.calculate_max_buy_bonds_out();
            match chain
                .mock_yield_space_math()
                .calculate_max_buy_bonds_out_safe(
                    state.ze()?.into(),
                    state.y().into(),
                    (fixed!(1e18) - state.t()).into(),
                    state.c().into(),
                    state.mu().into(),
                )
                .call()
                .await
            {
                Ok((expected_out, expected_status)) => {
                    assert_eq!(actual.is_ok(), expected_status);
                    assert_eq!(actual.unwrap_or(fixed!(0)), FixedPoint::from(expected_out));
                }
                Err(_) => assert!(actual.is_err()),
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn fuzz_calculate_max_sell_bonds_in() -> Result<()> {
        let chain = TestChain::new().await?;

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            let state = rng.gen::<State>();
            let z_min = rng.gen::<FixedPoint<U256>>();
            let actual = panic::catch_unwind(|| state.calculate_max_sell_bonds_in(z_min));
            match chain
                .mock_yield_space_math()
                .calculate_max_sell_bonds_in_safe(
                    state.z().into(),
                    state.zeta(),
                    state.y().into(),
                    z_min.into(),
                    (fixed!(1e18) - state.t()).into(),
                    state.c().into(),
                    state.mu().into(),
                )
                .call()
                .await
            {
                Ok((expected_out, expected_status)) => {
                    let actual = actual.unwrap();
                    assert_eq!(actual.is_ok(), expected_status);
                    assert_eq!(actual.unwrap_or(fixed!(0)), FixedPoint::from(expected_out));
                }
                Err(e) => assert!(actual.is_err() || actual.unwrap().is_err()),
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn fuzz_k_down() -> Result<()> {
        let chain = TestChain::new().await?;

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            let state = rng.gen::<State>();
            let actual = state.k_down();
            match chain
                .mock_yield_space_math()
                .k_down(
                    state.ze()?.into(),
                    state.y().into(),
                    (fixed!(1e18) - state.t()).into(),
                    state.c().into(),
                    state.mu().into(),
                )
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
    async fn fuzz_k_up() -> Result<()> {
        let chain = TestChain::new().await?;

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            let state = rng.gen::<State>();
            let actual = state.k_up();
            match chain
                .mock_yield_space_math()
                .k_up(
                    state.ze()?.into(),
                    state.y().into(),
                    (fixed!(1e18) - state.t()).into(),
                    state.c().into(),
                    state.mu().into(),
                )
                .call()
                .await
            {
                Ok(expected) => assert_eq!(actual.unwrap(), FixedPoint::from(expected)),
                Err(_) => assert!(actual.is_err()),
            }
        }

        Ok(())
    }
}
