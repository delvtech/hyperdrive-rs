use ethers::types::{I256, U256};
use eyre::{eyre, Result};
use fixed_point::{fixed, FixedPoint};

use super::math::SHARE_PROCEEDS_MAX_ITERATIONS;
use crate::State;

impl State {
    /// Allows an LP to burn shares and withdraw from the pool.
    pub fn calculate_remove_liquidity(
        &self,
        current_block_timestamp: U256,
        active_lp_total_supply: FixedPoint,
        withdrawal_shares_total_supply: FixedPoint,
        lp_shares: FixedPoint,
        total_vault_shares: FixedPoint,
        total_vault_assets: FixedPoint,
        min_output_per_share: FixedPoint,
        minimum_transaction_amount: FixedPoint,
        as_base: bool,
    ) -> Result<(FixedPoint, FixedPoint)> {
        // Ensure that the amount of LP shares to remove is greater than or
        // equal to the minimum transaction amount.
        if lp_shares < minimum_transaction_amount {
            return Err(eyre!("Minimum transaction amount not met"));
        }

        // Get vault share price and proceed without checkpoint.
        let vault_share_price = self.vault_share_price();

        let mut state = self.clone();
        // Burn the LP's shares.
        state.info.lp_total_supply -= lp_shares.into();
        let active_lp_total_supply = active_lp_total_supply - lp_shares;

        // Mint an equivalent amount of withdrawal shares.
        let withdrawal_shares_total_supply = withdrawal_shares_total_supply + lp_shares;

        // Redeem as many of the withdrawal shares as possible.
        let (proceeds, withdrawal_shares_redeemed) = state.calculate_redeem_withdrawal_shares(
            current_block_timestamp,
            active_lp_total_supply,
            withdrawal_shares_total_supply,
            lp_shares,
            vault_share_price,
            total_vault_shares,
            total_vault_assets,
            min_output_per_share,
            as_base,
        )?;
        let withdrawal_shares = lp_shares - withdrawal_shares_redeemed;

        Ok((proceeds, withdrawal_shares))
    }

    /// Redeems withdrawal shares by giving the LP a pro-rata amount of the
    /// withdrawal pool's proceeds. This function redeems the maximum amount of
    /// the specified withdrawal shares given the amount of withdrawal shares
    /// ready to withdraw.
    pub fn calculate_redeem_withdrawal_shares(
        &self,
        current_block_timestamp: U256,
        active_lp_total_supply: FixedPoint,
        withdrawal_shares_total_supply: FixedPoint,
        withdrawal_shares: FixedPoint,
        vault_share_price: FixedPoint,
        total_supply: FixedPoint,
        total_assets: FixedPoint,
        min_output_per_share: FixedPoint,
        as_base: bool,
    ) -> Result<(FixedPoint, FixedPoint)> {
        // Distribute the excess idle to the withdrawal pool. If the distribute
        // excess idle calculation fails, we proceed with the calculation since
        // LPs should be able to redeem their withdrawal shares for existing
        // withdrawal proceeds regardless of whether or not idle could be
        // distributed.
        let (withdrawal_shares_redeemed, share_proceeds, _success) = self
            .calculate_distribute_excess_idle_safe(
                current_block_timestamp,
                active_lp_total_supply,
                withdrawal_shares_total_supply,
                SHARE_PROCEEDS_MAX_ITERATIONS,
            )?;

        // Update withdrawal pool after distributing excess idle.
        let ready_to_withdraw =
            self.withdrawal_shares_ready_to_withdraw() + withdrawal_shares_redeemed;
        let withdrawal_share_proceeds = self.withdrawal_shares_proceeds() + share_proceeds;

        // Clamp the shares to the total amount of shares ready for withdrawal
        // to avoid unnecessary reverts. We exit early if the user has no shares
        // available to redeem.
        let mut withdrawal_shares_redeemed = withdrawal_shares;
        if withdrawal_shares_redeemed > ready_to_withdraw {
            withdrawal_shares_redeemed = ready_to_withdraw;
        }
        if withdrawal_shares_redeemed == fixed!(0) {
            return Ok((fixed!(0), fixed!(0)));
        }

        // NOTE: Round down to underestimate the share proceeds.
        //
        // The LP gets the pro-rata amount of the collected proceeds.
        let vault_share_price = self.vault_share_price();
        let share_proceeds =
            withdrawal_shares_redeemed.mul_div_down(withdrawal_share_proceeds, ready_to_withdraw);

        // Withdraw the share proceeds to the user
        let proceeds = self.withdraw(
            share_proceeds,
            vault_share_price,
            total_supply,
            total_assets,
            as_base,
        )?;

        // NOTE: Round up to make the check more conservative.
        //
        // Ensure proceeds meet minimum output per share
        if proceeds < min_output_per_share.mul_up(withdrawal_shares_redeemed) {
            return Err(eyre!("Output limit not met"));
        }

        Ok((proceeds, withdrawal_shares_redeemed))
    }

    /// Distribute as much of the excess idle as possible to the withdrawal
    /// pool while holding the LP share price constant.
    fn calculate_distribute_excess_idle_safe(
        &self,
        current_block_timestamp: U256,
        withdrawal_shares_total_supply: FixedPoint,
        vault_share_price: FixedPoint,
        max_iterations: u64,
    ) -> Result<(FixedPoint, FixedPoint, bool)> {
        let withdrawal_shares_total_supply =
            withdrawal_shares_total_supply - self.withdrawal_shares_ready_to_withdraw();

        // If there are no withdrawal shares, then there is nothing to
        // distribute.
        if withdrawal_shares_total_supply == fixed!(0) {
            return Ok((fixed!(0), fixed!(0), true));
        }

        // If there is no excess idle, then there is nothing to distribute.
        let idle = self.calculate_idle_share_reserves();
        if idle == fixed!(0) {
            return Ok((fixed!(0), fixed!(0), true));
        }

        // Calculate the amount of withdrawal shares that should be redeemed
        // and their share proceeds.
        let (withdrawal_shares_redeemed, share_proceeds) = self.calculate_distribute_excess_idle(
            current_block_timestamp,
            withdrawal_shares_total_supply,
            vault_share_price,
            max_iterations,
        )?;

        // Remove the withdrawal pool proceeds from the reserves.
        match self.calculate_update_liquidity_safe(
            self.share_reserves(),
            self.share_adjustment(),
            self.bond_reserves(),
            self.minimum_share_reserves(),
            -I256::try_from(share_proceeds)?,
        ) {
            // NOTE: we don't need to do anything with the result, if there is
            // an error, we exit early.
            Ok(_) => {}
            Err(_) => return Ok((fixed!(0), fixed!(0), false)),
        };

        Ok((withdrawal_shares_redeemed, share_proceeds, true))
    }

    fn withdraw(
        &self,
        shares: FixedPoint,
        vault_share_price: FixedPoint,
        total_shares: FixedPoint,
        total_assets: FixedPoint,
        as_base: bool,
    ) -> Result<FixedPoint> {
        // Withdraw logic here, returning the amount withdrawn
        let base_amount = shares.mul_down(vault_share_price);
        let shares = self.convert_to_shares(base_amount, total_shares, total_assets)?;

        if as_base {
            let amount_withdrawn = self.convert_to_assets(shares, total_shares, total_assets)?;
            return Ok(amount_withdrawn);
        }
        Ok(shares)
    }

    fn convert_to_shares(
        &self,
        base_amount: FixedPoint,
        total_supply: FixedPoint,
        total_assets: FixedPoint,
    ) -> Result<FixedPoint> {
        Ok(base_amount.mul_div_down(total_supply, total_assets))
    }

    fn convert_to_assets(
        &self,
        share_amount: FixedPoint,
        total_supply: FixedPoint,
        total_assets: FixedPoint,
    ) -> Result<FixedPoint> {
        Ok(share_amount.mul_div_down(total_assets, total_supply))
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::min;

    use fixed_point::uint256;
    use hyperdrive_test_utils::{chain::TestChain, constants::FUZZ_RUNS};
    use hyperdrive_wrappers::wrappers::ihyperdrive::Options;
    use rand::{thread_rng, Rng};

    use super::*;
    use crate::test_utils::agent::HyperdriveMathAgent;

    #[tokio::test]
    async fn fuzz_test_calculate_remove_liquidity() -> Result<()> {
        // Spawn a test chain and create two agents -- Alice and Bob.
        let mut rng = thread_rng();
        let chain = TestChain::new().await?;
        let mut alice = chain.alice().await?;
        let mut bob = chain.bob().await?;
        let config = bob.get_config().clone();

        for _ in 0..*FUZZ_RUNS {
            // Snapshot the chain.
            let id = chain.snapshot().await?;

            // Fund Alice and Bob.
            let fixed_rate = rng.gen_range(fixed!(0.01e18)..=fixed!(0.1e18));
            let contribution = rng.gen_range(fixed!(10_000e18)..=fixed!(500_000_000e18));
            let budget = rng.gen_range(fixed!(10e18)..=fixed!(500_000_000e18));
            alice.fund(contribution).await?;
            bob.fund(budget).await?;

            // Alice initializes the pool.
            alice.initialize(fixed_rate, contribution, None).await?;

            // Some of the checkpoint passes and variable interest accrues.
            alice
                .checkpoint(alice.latest_checkpoint().await?, uint256!(0), None)
                .await?;
            let rate = rng.gen_range(fixed!(0)..=fixed!(0.5e18));
            let timestamp = alice.now().await?;
            alice
                .advance_time(
                    rate,
                    FixedPoint::from(config.checkpoint_duration) * fixed!(0.5e18),
                )
                .await?;

            // Bob adds liquidity
            bob.add_liquidity(budget, None).await?;

            // Get total supply and total assets for the next block to make
            // sure rust has the same values as the solidity does.
            let timestamp = alice.now().await?;
            let total_supply: FixedPoint = bob.vault().total_supply().call().await?.into();
            let total_assets: FixedPoint = bob
                .vault()
                .total_assets_with_timestamp(timestamp + uint256!(1))
                .call()
                .await?
                .into();

            // Get the State from solidity before removing liquidity.
            let hd_state = bob.get_state().await?;
            let mut state = State {
                config: hd_state.config.clone(),
                info: hd_state.info.clone(),
            };

            // active_lp_total_supply and withdrawal_shares_total_supply are
            // just the token supplies.  Get them from the contract.
            let lp_token_asset_id = U256::zero();
            let active_lp_total_supply: FixedPoint = bob
                .hyperdrive()
                .total_supply(lp_token_asset_id)
                .await?
                .into();
            let withdrawal_share_asset_id = U256::from(3) << 248;
            let withdrawal_shares_total_supply: FixedPoint = bob
                .hyperdrive()
                .total_supply(withdrawal_share_asset_id)
                .await?
                .into();

            // Get the amount to remove, hit the budget 1% of the time to test
            // that case but don't remove more than is possible.
            let remove_budget = min(
                rng.gen_range(fixed!(0)..=fixed!(1.01e18) * bob.wallet.lp_shares),
                bob.wallet.lp_shares,
            );
            let remove_budget = min(
                active_lp_total_supply - fixed!(2e18) * state.minimum_share_reserves(),
                remove_budget,
            );

            // Bob removes liquidity.
            let as_base = true;
            let options = Options {
                destination: bob.client().address(),
                as_base,
                extra_data: [].into(),
            };
            let tx_result = bob
                .remove_liquidity(remove_budget, Some(options), None)
                .await;

            // Get values for the block at which solidity code ran.
            let current_block_timestamp = bob.now().await?;
            let vault_share_price = bob.get_state().await?.info.vault_share_price;
            state.info.vault_share_price = vault_share_price;

            // Calculate shares and withdrawal shares from the rust function.
            let result = std::panic::catch_unwind(|| {
                state
                    .calculate_remove_liquidity(
                        current_block_timestamp,
                        active_lp_total_supply,
                        withdrawal_shares_total_supply,
                        remove_budget,
                        total_supply,
                        total_assets,
                        fixed!(0),
                        fixed!(1),
                        as_base,
                    )
                    .unwrap()
            });

            match result {
                Ok((rust_amount, rust_withdrawal_shares)) => {
                    let (sol_amount, sol_withdrawal_shares) = tx_result?;
                    // Assert amounts redeemed match between rust and solidity.
                    assert!(rust_amount == sol_amount.into());

                    // Assert withdrawal shares results match between rust and
                    // solidity
                    assert!(rust_withdrawal_shares == sol_withdrawal_shares.into());
                }
                Err(_) => assert!(tx_result.is_err()),
            }

            // Revert to the snapshot and reset the agent's wallets.
            chain.revert(id).await?;
            alice.reset(Default::default()).await?;
            bob.reset(Default::default()).await?;
        }

        Ok(())
    }
}
