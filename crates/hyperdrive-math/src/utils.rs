use ethers::types::{I256, U256};
use eyre::{eyre, Result};
use fixed_point::{fixed, uint256, FixedPoint};

pub fn calculate_time_stretch(
    rate: FixedPoint,
    position_duration: FixedPoint,
) -> Result<FixedPoint> {
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
    Ok((FixedPoint::try_from(FixedPoint::ln(
        I256::try_from(fixed!(1e18) + rate.mul_div_down(position_duration, seconds_in_a_year))
            .unwrap(),
    )?)? / FixedPoint::try_from(FixedPoint::ln(I256::try_from(fixed!(1e18) + rate)?)?)?)
        * time_stretch)
}

pub fn calculate_effective_share_reserves(
    share_reserves: FixedPoint,
    share_adjustment: I256,
) -> Result<FixedPoint> {
    let effective_share_reserves = I256::try_from(share_reserves).unwrap() - share_adjustment;
    if effective_share_reserves < I256::from(0) {
        return Err(eyre!("effective share reserves cannot be negative"));
    }
    effective_share_reserves.try_into()
}

pub fn calculate_effective_share_reserves_safe(
    share_reserves: FixedPoint,
    share_adjustment: I256,
) -> Result<FixedPoint> {
    let effective_share_reserves = I256::try_from(share_reserves).unwrap() - share_adjustment;
    if effective_share_reserves < I256::from(0) {
        return Err(eyre!("effective share reserves cannot be negative"));
    }
    effective_share_reserves.try_into()
}

/// Calculates the bond reserves assuming that the pool has a given
/// effective share reserves and fixed rate APR.
///
/// NOTE: This function should not be used for computing reserve levels when
/// initializing a pool. Instead use `lp::calculate_initial_reserves`.
///
/// r = ((1 / p) - 1) / t = (1 - p) / (pt)
/// p = ((u * z) / y) ** t
///
/// Arguments:
///
/// * effective_share_reserves : The pool's effective share reserves. The
/// effective share reserves are a modified version of the share
/// reserves used when pricing trades.
/// * target_rate : The target pool's fixed APR.
/// * initial_vault_share_price : The pool's initial vault share price.
/// * position_duration : The amount of time until maturity in seconds.
/// * time_stretch : The time stretch parameter.
///
/// Returns:
///
/// * bond_reserves : The bond reserves that make the pool have a specified APR.
pub fn calculate_bonds_given_effective_shares_and_rate(
    effective_share_reserves: FixedPoint,
    target_rate: FixedPoint,
    initial_vault_share_price: FixedPoint,
    position_duration: FixedPoint,
    time_stretch: FixedPoint,
) -> Result<FixedPoint> {
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
/// $$
/// r = (1 - p) / (p t)
/// $$
///
/// where $p$ is the price and $t$ is the length of time that this price is
/// assumed to be constant, in units of years. For example, if the price is
/// constant for 6 months, then $t=0.5$.
/// In our case, $t = \text{position_duration} / (60*60*24*365)$.
pub fn calculate_rate_given_fixed_price(
    price: FixedPoint,
    position_duration: FixedPoint,
) -> FixedPoint {
    let fixed_price_duration_in_years =
        position_duration / FixedPoint::from(U256::from(60 * 60 * 24 * 365));
    (fixed!(1e18) - price) / (price * fixed_price_duration_in_years)
}

/// Calculate the holding period return (HPR) given a non-compounding, annualized rate (APR).
///
/// Since the rate is non-compounding, we calculate the hpr as:
///
/// $$
/// hpr = apr * t
/// $$
///
/// where $t$ is the holding period, in units of years. For example, if the
/// holding period is 6 months, then $t=0.5$.
pub fn calculate_hpr_given_apr(apr: FixedPoint, position_duration: FixedPoint) -> FixedPoint {
    let holding_period_in_years =
        position_duration / FixedPoint::from(U256::from(60 * 60 * 24 * 365));
    apr * holding_period_in_years
}

/// Calculate the holding period return (HPR) given a compounding, annualized rate (APY).
///
/// Since the rate is compounding, we calculate the hpr as:
///
/// $$
/// hpr = (1 + apy) ^ (t) - 1
/// $$
///
/// where $t$ is the holding period, in units of years. For example, if the
/// holding period is 6 months, then $t=0.5$.
pub fn calculate_hpr_given_apy(
    apy: FixedPoint,
    position_duration: FixedPoint,
) -> Result<FixedPoint> {
    let holding_period_in_years =
        position_duration / FixedPoint::from(U256::from(60 * 60 * 24 * 365));
    Ok((fixed!(1e18) + apy).pow(holding_period_in_years)? - fixed!(1e18))
}

#[cfg(test)]
mod tests {
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
            let checkpoint_exposure = {
                let value = rng.gen_range(fixed!(0)..=FixedPoint::try_from(I256::MAX)?);
                let sign = rng.gen::<bool>();
                if sign {
                    -I256::try_from(value).unwrap()
                } else {
                    I256::try_from(value).unwrap()
                }
            };
            let open_vault_share_price = rng.gen_range(fixed!(0)..=state.vault_share_price());

            // Get the min rate.
            let max_long = match state.calculate_max_long(U256::MAX, checkpoint_exposure, None) {
                Ok(max_long) => max_long,
                Err(_) => continue, // Don't finish this fuzz iteration.
            };
            let min_rate = state.calculate_spot_rate_after_long(max_long, None)?;

            // Get the max rate.
            let max_short = match state.calculate_max_short(
                U256::MAX,
                open_vault_share_price,
                checkpoint_exposure,
                None,
                None,
            ) {
                Ok(max_short) => max_short,
                Err(_) => continue, // Don't finish this fuzz iteration.
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
            assert_eq!(new_state.calculate_spot_rate()?, target_rate)
        }
        Ok(())
    }
}
