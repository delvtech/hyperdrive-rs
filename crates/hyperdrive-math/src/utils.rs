use ethers::types::{I256, U256};
use eyre::{eyre, Result};
use fixedpointmath::{fixed, ln, uint256, FixedPoint};

pub fn calculate_time_stretch(
    rate: FixedPoint<U256>,
    position_duration: FixedPoint<U256>,
) -> Result<FixedPoint<U256>> {
    let seconds_in_a_year = FixedPoint::from(U256::from(60 * 60 * 24 * 365));
    // Calculate the benchmark time stretch. This time stretch is tuned for
    // a position duration of 1 year.
    let time_stretch = fixed!(5.24592e18)
        / (fixed!(0.04665e18) * FixedPoint::from(U256::from(rate) * uint256!(100)));
    let time_stretch = fixed!(1e18) / time_stretch;

    // We know that the following simultaneous equations hold:
    //
    // (1 + apr) * A ** timeStretch = 1
    //
    // and
    //
    // (1 + apr * (positionDuration / 365 days)) * A ** targetTimeStretch = 1
    //
    // where A is the reserve ratio. We can solve these equations for the
    // target time stretch as follows:
    //
    // targetTimeStretch = (
    //     ln(1 + apr * (positionDuration / 365 days)) /
    //     ln(1 + apr)
    // ) * timeStretch
    //
    // NOTE: Round down so that the output is an underestimate.
    Ok((FixedPoint::try_from(ln(I256::try_from(
        fixed!(1e18) + rate.mul_div_down(position_duration, seconds_in_a_year),
    )?)?)?
        / FixedPoint::try_from(ln(I256::try_from(fixed!(1e18) + rate)?)?)?)
        * time_stretch)
}

/// Calculates the share reserves after zeta adjustment, aka the effective share
/// reserves: `$z_e = z - zeta$`.
pub fn calculate_effective_share_reserves(
    share_reserves: FixedPoint<U256>,
    share_adjustment: I256,
) -> Result<FixedPoint<U256>> {
    let effective_share_reserves = I256::try_from(share_reserves)? - share_adjustment;
    if effective_share_reserves < I256::from(0) {
        return Err(eyre!("effective share reserves cannot be negative"));
    }
    effective_share_reserves.try_into()
}

/// Calculates the bond reserves assuming that the pool has a given
/// effective share reserves and fixed rate APR.
///
/// NOTE: This function should not be used for computing reserve levels when
/// initializing a pool. Instead use
/// (calculate_initial_reserves)[State::calculate_initial_reserves].
///
/// ```math
/// \begin{aligned}
/// r &= \tfrac{(1 / p) - 1}{t} \\
/// &= \frac{1 - p}{p \cdot t}
/// ```
///
/// ```math
/// p = \left( \tfrac{u * z}{y} \right)^{t}
/// ```
///
/// Returns the bond reserves that make the pool have a specified APR.
pub fn calculate_bonds_given_effective_shares_and_rate(
    effective_share_reserves: FixedPoint<U256>,
    target_rate: FixedPoint<U256>,
    initial_vault_share_price: FixedPoint<U256>,
    position_duration: FixedPoint<U256>,
    time_stretch: FixedPoint<U256>,
) -> Result<FixedPoint<U256>> {
    // NOTE: Round down to underestimate the initial bond reserves.
    //
    // Normalize the time to maturity to fractions of a year since the provided
    // rate is an APR.
    let t = position_duration / FixedPoint::from(U256::from(60 * 60 * 24 * 365));

    // NOTE: Round down to underestimate the initial bond reserves.
    //
    // inner = (1 + apr * t) ** (1 / t_s)
    let mut inner = fixed!(1e18) + target_rate.mul_down(t);
    if inner >= fixed!(1e18) {
        // Rounding down the exponent results in a smaller result.
        inner = inner.pow(fixed!(1e18) / time_stretch)?;
    } else {
        // Rounding up the exponent results in a smaller result.
        inner = inner.pow(fixed!(1e18).div_up(time_stretch))?;
    }

    // NOTE: Round down to underestimate the initial bond reserves.
    //
    // mu * (z - zeta) * (1 + apr * t) ** (1 / tau)
    Ok(initial_vault_share_price
        .mul_down(effective_share_reserves)
        .mul_down(inner))
}

/// Calculate the rate assuming a given price is constant for some annualized duration.
///
/// We calculate the rate for a fixed length of time as:
///
/// ```math
/// r = \frac{(1 - p)}{p \cdot t}
/// ```
///
/// where $p$ is the price and $t$ is the length of time that this price is
/// assumed to be constant, in units of years. For example, if the price is
/// constant for 6 months, then `$t=0.5$`.
/// In our case, `$t = \text{position_duration} / (60*60*24*365)$`.
pub fn calculate_rate_given_fixed_price(
    price: FixedPoint<U256>,
    position_duration: FixedPoint<U256>,
) -> FixedPoint<U256> {
    let fixed_price_duration_in_years =
        position_duration / FixedPoint::from(U256::from(60 * 60 * 24 * 365));
    (fixed!(1e18) - price) / (price * fixed_price_duration_in_years)
}

/// Calculate the holding period return (HPR) given a non-compounding, annualized rate (APR).
///
/// Since the rate is non-compounding, we calculate the hpr as:
///
/// ```math
/// \text{hpr} = \text{apr} \cdot t
/// ```
///
/// where `$t$` is the holding period, in units of years. For example, if the
/// holding period is 6 months, then `$t=0.5$`.
pub fn calculate_hpr_given_apr(apr: I256, position_duration: FixedPoint<U256>) -> Result<I256> {
    let holding_period_in_years =
        position_duration / FixedPoint::from(U256::from(60 * 60 * 24 * 365));
    let (sign, apr_abs) = apr.into_sign_and_abs();
    let hpr = FixedPoint::from(apr_abs) * holding_period_in_years;
    Ok(I256::checked_from_sign_and_abs(sign, hpr.into()).unwrap())
}

/// Calculate the holding period return (HPR) given a compounding, annualized rate (APY).
///
/// Since the rate is compounding, we calculate the hpr as:
///
/// ```math
/// \text{hpr} = (1 +  \text{apy})^{t} - 1
/// ```
///
/// where `$t$` is the holding period, in units of years. For example, if the
/// holding period is 6 months, then `$t=0.5$`.
pub fn calculate_hpr_given_apy(apy: I256, position_duration: FixedPoint<U256>) -> Result<I256> {
    let holding_period_in_years =
        position_duration / FixedPoint::from(U256::from(60 * 60 * 24 * 365));
    let (sign, apy_abs) = apy.into_sign_and_abs();
    let hpr =
        (fixed!(1e18) + FixedPoint::from(apy_abs)).pow(holding_period_in_years)? - fixed!(1e18);
    Ok(I256::checked_from_sign_and_abs(sign, hpr.into()).unwrap())
}

#[cfg(test)]
mod tests {
    use std::panic;

    use fixedpointmath::FixedPointValue;
    use hyperdrive_test_utils::{
        chain::TestChain,
        constants::{FAST_FUZZ_RUNS, FUZZ_RUNS},
    };
    use rand::{thread_rng, Rng};

    use super::*;
    use crate::State;

    #[tokio::test]
    async fn fuzz_calculate_time_stretch() -> Result<()> {
        let chain = TestChain::new().await?;

        // Fuzz the rust and solidity implementations against each other.
        let seconds_in_ten_years = U256::from(10 * 60 * 60 * 24 * 365);
        let seconds_in_a_day = U256::from(60 * 60 * 24);
        let mut rng = thread_rng();
        for _ in 0..*FAST_FUZZ_RUNS {
            // Get the current state of the mock contract
            let position_duration = rng.gen_range(
                FixedPoint::from(seconds_in_a_day)..=FixedPoint::from(seconds_in_ten_years),
            );
            let apr = rng.gen_range(fixed!(0.001e18)..=fixed!(10.0e18));
            let actual_t = calculate_time_stretch(apr, position_duration);
            match chain
                .mock_hyperdrive_math()
                .calculate_time_stretch(apr.into(), position_duration.into())
                .call()
                .await
            {
                Ok(expected_t) => {
                    assert_eq!(actual_t.unwrap(), FixedPoint::from(expected_t));
                }
                Err(_) => assert!(actual_t.is_err()),
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn fuzz_calculate_bonds_given_effective_shares_and_rate() -> Result<()> {
        let mut rng = thread_rng();
        for _ in 0..*FUZZ_RUNS {
            // Gen the random state.
            let state = rng.gen::<State>();
            let checkpoint_exposure = rng
                .gen_range(fixed!(0)..=FixedPoint::<I256>::MAX)
                .raw()
                .flip_sign_if(rng.gen());
            let open_vault_share_price = rng.gen_range(fixed!(0)..=state.vault_share_price());

            // Get the min rate.
            // We need to catch panics because of overflows.
            let max_long = match state.calculate_max_long(U256::MAX, checkpoint_exposure, None) {
                Ok(max_long) => max_long,
                Err(_) => continue, // Max threw an Err. Don't finish this fuzz iteration.
            };
            let min_rate = state.calculate_spot_rate_after_long(max_long, None)?;

            // Get the max rate.
            // We need to catch panics because of overflows.
            let max_short = match panic::catch_unwind(|| {
                state.calculate_max_short(
                    U256::MAX,
                    open_vault_share_price,
                    checkpoint_exposure,
                    None,
                    None,
                )
            }) {
                Ok(max_short) => match max_short {
                    Ok(max_short) => max_short,
                    Err(_) => continue, // Max threw an Err; don't finish this fuzz iteration.
                },
                Err(_) => continue, // Max threw a panic; don't finish this fuzz iteration.
            };
            let max_rate = state.calculate_spot_rate_after_short(max_short, None)?;

            // Get a random target rate that is allowable.
            let target_rate = rng.gen_range(min_rate..=max_rate);

            // Calculate the new bond reserves.
            let bond_reserves = calculate_bonds_given_effective_shares_and_rate(
                state.effective_share_reserves()?,
                target_rate,
                state.initial_vault_share_price(),
                state.position_duration(),
                state.time_stretch(),
            )?;

            // Make a new state with the updated reserves & check the spot rate.
            let mut new_state: State = state.clone();
            new_state.info.bond_reserves = bond_reserves.into();
            let new_spot_rate = new_state.calculate_spot_rate()?;
            let tolerance = fixed!(1e8);
            assert!(
                target_rate.abs_diff(new_spot_rate) < tolerance,
                r#"
      target rate: {target_rate}
    new spot rate: {new_spot_rate}
             diff: {diff}
        tolerance: {tolerance}
"#,
                diff = target_rate.abs_diff(new_spot_rate),
            );
        }
        Ok(())
    }
}
