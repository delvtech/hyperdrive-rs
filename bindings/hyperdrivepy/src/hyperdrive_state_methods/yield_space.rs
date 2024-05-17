use ethers::core::types::U256;
use fixed_point::FixedPoint;
use hyperdrive_math::YieldSpace;
use pyo3::{exceptions::PyValueError, prelude::*};

use crate::HyperdriveState;

#[pymethods]
impl HyperdriveState {
    pub fn calculate_bonds_out_given_shares_in_down(&self, amount_in: &str) -> PyResult<String> {
        let amount_in_fp = FixedPoint::from(U256::from_dec_str(amount_in).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert amount_in string to U256")
        })?);
        let result_fp = self
            .state
            .calculate_bonds_out_given_shares_in_down(amount_in_fp);
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }

    pub fn calculate_shares_in_given_bonds_out_up(&self, amount_in: &str) -> PyResult<String> {
        let amount_in_fp = FixedPoint::from(U256::from_dec_str(amount_in).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert amount_in string to U256")
        })?);
        // We unwrap the error here to throw panic error if this fails
        let result_fp = self
            .state
            .calculate_shares_in_given_bonds_out_up_safe(amount_in_fp)
            .unwrap();
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }

    pub fn calculate_shares_in_given_bonds_out_down(&self, amount_in: &str) -> PyResult<String> {
        let amount_in_fp = FixedPoint::from(U256::from_dec_str(amount_in).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert amount_in string to U256")
        })?);
        let result_fp = self
            .state
            .calculate_shares_in_given_bonds_out_down(amount_in_fp);
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }

    pub fn calculate_shares_out_given_bonds_in_down(&self, amount_in: &str) -> PyResult<String> {
        let amount_in_fp = FixedPoint::from(U256::from_dec_str(amount_in).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert amount_in string to U256")
        })?);
        let result_fp = self
            .state
            .calculate_shares_out_given_bonds_in_down(amount_in_fp);
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }
}
