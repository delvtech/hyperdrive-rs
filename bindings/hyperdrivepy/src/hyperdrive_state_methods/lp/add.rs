use ethers::core::types::U256;
use fixed_point::FixedPoint;
use pyo3::{exceptions::PyValueError, prelude::*};

use crate::HyperdriveState;

#[pymethods]
impl HyperdriveState {
    pub fn calculate_add_liquidity(
        &self,
        current_block_timestamp: &str,
        contribution: &str,
        min_lp_share_price: &str,
        min_apr: &str,
        max_apr: &str,
        as_base: &str,
    ) -> PyResult<String> {
        let current_block_timestamp =
            U256::from_dec_str(current_block_timestamp).map_err(|err| {
                PyErr::new::<PyValueError, _>(format!(
                    "Failed to convert current_block_timestamp string to U256: {}",
                    err
                ))
            })?;
        let contribution_fp =
            FixedPoint::from(U256::from_dec_str(contribution).map_err(|err| {
                PyErr::new::<PyValueError, _>(format!(
                    "Failed to convert contribution string to U256: {}",
                    err
                ))
            })?);
        let min_lp_share_price_fp =
            FixedPoint::from(U256::from_dec_str(min_lp_share_price).map_err(|err| {
                PyErr::new::<PyValueError, _>(format!(
                    "Failed to convert min_lp_share_price string to U256: {}",
                    err
                ))
            })?);
        let min_apr_fp = FixedPoint::from(U256::from_dec_str(min_apr).map_err(|err| {
            PyErr::new::<PyValueError, _>(format!(
                "Failed to convert min_apr string to U256: {}",
                err
            ))
        })?);
        let max_apr_fp = FixedPoint::from(U256::from_dec_str(max_apr).map_err(|err| {
            PyErr::new::<PyValueError, _>(format!(
                "Failed to convert max_apr string to U256: {}",
                err
            ))
        })?);
        let as_base = as_base.parse::<bool>().map_err(|err| {
            PyErr::new::<PyValueError, _>(format!(
                "Failed to convert as_base string to bool: {}",
                err
            ))
        })?;
        let result_fp = self
            .state
            .calculate_add_liquidity(
                current_block_timestamp,
                contribution_fp,
                min_lp_share_price_fp,
                min_apr_fp,
                max_apr_fp,
                as_base,
            )
            .map_err(|err| {
                PyErr::new::<PyValueError, _>(format!(
                    "Failed to run calculate_add_liquidity: {}",
                    err
                ))
            })?;
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }

    pub fn calculate_pool_deltas_after_add_liquidity(
        &self,
        contribution: &str,
        as_base: &str,
    ) -> PyResult<(String, String, String)> {
        let contribution_fp = FixedPoint::from(U256::from_dec_str(contribution).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert contribution string to U256")
        })?);
        let as_base = as_base.parse::<bool>().map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert as_base string to bool")
        })?;
        let (result_fp1, result_i256, result_fp2) = self
            .state
            .calculate_pool_deltas_after_add_liquidity(contribution_fp, as_base)
            .map_err(|err| {
                PyErr::new::<PyValueError, _>(format!(
                    "calculate_pool_deltas_after_add_liquidity returned the error: {:?}",
                    err
                ))
            })?;
        let result1 = U256::from(result_fp1).to_string();
        let result2 = result_i256.to_string();
        let result3 = U256::from(result_fp2).to_string();
        Ok((result1, result2, result3))
    }
}
