use ethers::core::types::U256;
use fixed_point::FixedPoint;
use pyo3::{exceptions::PyValueError, prelude::*};

use crate::HyperdriveState;

#[pymethods]
impl HyperdriveState {
    pub fn calculate_close_short(
        &self,
        bond_amount: &str,
        open_vault_share_price: &str,
        close_vault_share_price: &str,
        maturity_time: &str,
        current_time: &str,
    ) -> PyResult<String> {
        let bond_amount_fp = FixedPoint::from(U256::from_dec_str(bond_amount).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert bond_amount string to U256")
        })?);
        let open_vault_share_price_fp =
            FixedPoint::from(U256::from_dec_str(open_vault_share_price).map_err(|_| {
                PyErr::new::<PyValueError, _>(
                    "Failed to convert open_vault_share_price string to U256",
                )
            })?);
        let close_vault_share_price_fp =
            FixedPoint::from(U256::from_dec_str(close_vault_share_price).map_err(|_| {
                PyErr::new::<PyValueError, _>(
                    "Failed to convert close_vault_share_price string to U256",
                )
            })?);
        let maturity_time = U256::from_dec_str(maturity_time).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert maturity_time string to U256")
        })?;
        let current_time = U256::from_dec_str(current_time).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert current_time string to U256")
        })?;
        let result_fp = self.state.calculate_close_short(
            bond_amount_fp,
            open_vault_share_price_fp,
            close_vault_share_price_fp,
            maturity_time,
            current_time,
        );
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }
}
