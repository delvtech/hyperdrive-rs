use ethers::types::{I256, U256};
use eyre::{eyre, Result};
use fixed_point::FixedPoint;
use fixed_point_macros::{fixed, uint256};

pub fn calculate_time_stretch(rate: FixedPoint, position_duration: FixedPoint) -> FixedPoint {
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
    (FixedPoint::from(FixedPoint::ln(
        I256::try_from(fixed!(1e18) + rate.mul_div_down(position_duration, seconds_in_a_year))
            .unwrap(),
    )) / FixedPoint::from(FixedPoint::ln(I256::try_from(fixed!(1e18) + rate).unwrap())))
        * time_stretch
}

pub fn calculate_effective_share_reserves(
    share_reserves: FixedPoint,
    share_adjustment: I256,
) -> FixedPoint {
    let effective_share_reserves = I256::try_from(share_reserves).unwrap() - share_adjustment;
    if effective_share_reserves < I256::from(0) {
        panic!("effective share reserves cannot be negative");
    }
    effective_share_reserves.into()
}

pub fn calculate_effective_share_reserves_safe(
    share_reserves: FixedPoint,
    share_adjustment: I256,
) -> Result<FixedPoint> {
    let effective_share_reserves = I256::try_from(share_reserves).unwrap() - share_adjustment;
    if effective_share_reserves < I256::from(0) {
        return Err(eyre!("effective share reserves cannot be negative"));
    }
    Ok(effective_share_reserves.into())
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

#[cfg(test)]
mod tests {
    use std::panic;

    use eyre::Result;
    use rand::{thread_rng, Rng};
    use test_utils::{chain::TestChain, constants::FAST_FUZZ_RUNS};

    use super::*;

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
                    assert_eq!(actual_t, FixedPoint::from(expected_t));
                }
                Err(_) => panic!("Test failed."),
            }
        }

        Ok(())
    }
}
