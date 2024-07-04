use ethers::core::types::U256;
use fixedpointmath::FixedPoint;
use pyo3::{exceptions::PyValueError, prelude::*};

use crate::HyperdriveState;

#[pymethods]
impl HyperdriveState {
    pub fn calculate_remove_liquidity(
        &self,
        current_block_timestamp: &str,
        active_lp_total_supply: &str,
        withdrawal_shares_total_supply: &str,
        lp_shares: &str,
        total_vault_shares: &str,
        total_vault_assets: &str,
        min_output_per_share: &str,
        minimum_transaction_amount: &str,
        as_base: &str,
    ) -> PyResult<(String, String)> {
        let current_block_timestamp =
            U256::from_dec_str(current_block_timestamp).map_err(|err| {
                PyErr::new::<PyValueError, _>(format!(
                    "Failed to convert current_block_timestamp string {} to U256: {}",
                    current_block_timestamp, err
                ))
            })?;
        let active_lp_total_supply_fp =
            FixedPoint::from(U256::from_dec_str(active_lp_total_supply).map_err(|err| {
                PyErr::new::<PyValueError, _>(format!(
                    "Failed to convert active_lp_total_supply string {} to U256: {}",
                    active_lp_total_supply, err
                ))
            })?);
        let withdrawal_shares_total_supply_fp = FixedPoint::from(
            U256::from_dec_str(withdrawal_shares_total_supply).map_err(|err| {
                PyErr::new::<PyValueError, _>(format!(
                    "Failed to convert withdrawal_shares_total_supply string {} to U256: {}",
                    withdrawal_shares_total_supply, err
                ))
            })?,
        );
        let lp_shares_fp = FixedPoint::from(U256::from_dec_str(lp_shares).map_err(|err| {
            PyErr::new::<PyValueError, _>(format!(
                "Failed to convert lp_shares string {} to U256: {}",
                lp_shares, err
            ))
        })?);
        let total_vault_shares_fp =
            FixedPoint::from(U256::from_dec_str(total_vault_shares).map_err(|err| {
                PyErr::new::<PyValueError, _>(format!(
                    "Failed to convert total_vault_shares string {} to U256: {}",
                    total_vault_shares, err
                ))
            })?);
        let total_vault_assets_fp =
            FixedPoint::from(U256::from_dec_str(total_vault_assets).map_err(|err| {
                PyErr::new::<PyValueError, _>(format!(
                    "Failed to convert total_vault_assets string {} to U256: {}",
                    total_vault_assets, err
                ))
            })?);
        let min_output_per_share_fp =
            FixedPoint::from(U256::from_dec_str(min_output_per_share).map_err(|err| {
                PyErr::new::<PyValueError, _>(format!(
                    "Failed to convert min_output_per_share string {} to U256: {}",
                    min_output_per_share, err
                ))
            })?);
        let minimum_transaction_amount_fp = FixedPoint::from(
            U256::from_dec_str(minimum_transaction_amount).map_err(|err| {
                PyErr::new::<PyValueError, _>(format!(
                    "Failed to convert minimum_transaction_amount string {} to U256: {}",
                    minimum_transaction_amount, err
                ))
            })?,
        );

        let as_base = as_base.parse::<bool>().map_err(|err| {
            PyErr::new::<PyValueError, _>(format!(
                "Failed to convert as_base string {} to bool: {}",
                as_base, err
            ))
        })?;

        let result = self
            .state
            .calculate_remove_liquidity(
                current_block_timestamp,
                active_lp_total_supply_fp,
                withdrawal_shares_total_supply_fp,
                lp_shares_fp,
                total_vault_shares_fp,
                total_vault_assets_fp,
                min_output_per_share_fp,
                minimum_transaction_amount_fp,
                as_base,
            )
            .map_err(|err| {
                PyErr::new::<PyValueError, _>(format!("calculate_remove_liquidity: {}", err))
            })?;
        let (withdrawal_shares_redeemed, share_proceeds, _state) = result;

        Ok((
            U256::from(share_proceeds).to_string(),
            U256::from(withdrawal_shares_redeemed).to_string(),
        ))
    }
}
