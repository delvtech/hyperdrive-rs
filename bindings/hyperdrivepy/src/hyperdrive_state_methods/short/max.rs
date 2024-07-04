use ethers::core::types::{I256, U256};
use fixedpointmath::FixedPoint;
use pyo3::{exceptions::PyValueError, prelude::*};

use crate::HyperdriveState;

#[pymethods]
impl HyperdriveState {
    pub fn calculate_max_short(
        &self,
        budget: &str,
        open_vault_share_price: &str,
        checkpoint_exposure: &str,
        maybe_conservative_price: Option<&str>,
        maybe_max_iterations: Option<usize>,
    ) -> PyResult<String> {
        let budget_fp = FixedPoint::from(U256::from_dec_str(budget).map_err(|err| {
            PyErr::new::<PyValueError, _>(format!(
                "Failed to convert budget string {} to U256: {}",
                budget, err
            ))
        })?);
        let open_vault_share_price_fp =
            FixedPoint::from(U256::from_dec_str(open_vault_share_price).map_err(|err| {
                PyErr::new::<PyValueError, _>(format!(
                    "Failed to convert open_vault_share_price string {} to U256: {}",
                    open_vault_share_price, err
                ))
            })?);
        let checkpoint_exposure_i = I256::from_dec_str(checkpoint_exposure).map_err(|err| {
            PyErr::new::<PyValueError, _>(format!(
                "Failed to convert checkpoint_exposure string {} to I256: {}",
                checkpoint_exposure, err
            ))
        })?;
        let maybe_conservative_price_fp = if let Some(conservative_price) = maybe_conservative_price
        {
            Some(FixedPoint::from(
                U256::from_dec_str(conservative_price).map_err(|err| {
                    PyErr::new::<PyValueError, _>(format!(
                        "Failed to convert maybe_conservative_price string {} to U256: {}",
                        conservative_price, err
                    ))
                })?,
            ))
        } else {
            None
        };
        let result_fp = self
            .state
            .calculate_max_short(
                budget_fp,
                open_vault_share_price_fp,
                checkpoint_exposure_i,
                maybe_conservative_price_fp,
                maybe_max_iterations,
            )
            .map_err(|err| {
                PyErr::new::<PyValueError, _>(format!("calculate_max_short: {}", err))
            })?;
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }
}
