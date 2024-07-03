use ethers::core::types::U256;
use fixedpointmath::FixedPoint;
use hyperdrive_math::YieldSpace;
use pyo3::{exceptions::PyValueError, prelude::*};

use crate::HyperdriveState;

#[pymethods]
impl HyperdriveState {
    pub fn calculate_bonds_out_given_shares_in_down(&self, amount_in: &str) -> PyResult<String> {
        let amount_in_fp = FixedPoint::from(U256::from_dec_str(amount_in).map_err(|err| {
            PyErr::new::<PyValueError, _>(format!(
                "Failed to convert amount_in string {} to U256: {}",
                amount_in, err
            ))
        })?);
        let result_fp = self
            .state
            .calculate_bonds_out_given_shares_in_down(amount_in_fp)
            .map_err(|err| {
                PyErr::new::<PyValueError, _>(format!(
                    "calculate_bonds_out_given_shares_in_down: {}",
                    err
                ))
            })?;
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }

    pub fn calculate_shares_in_given_bonds_out_up(&self, amount_in: &str) -> PyResult<String> {
        let amount_in_fp = FixedPoint::from(U256::from_dec_str(amount_in).map_err(|err| {
            PyErr::new::<PyValueError, _>(format!(
                "Failed to convert amount_in string {} to U256: {}",
                amount_in, err
            ))
        })?);
        let result_fp = self
            .state
            .calculate_shares_in_given_bonds_out_up_safe(amount_in_fp)
            .map_err(|err| {
                PyErr::new::<PyValueError, _>(format!(
                    "Failed to execute calculate_shares_in_given_bonds_out_up_safe: {}",
                    err
                ))
            })?;
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }

    pub fn calculate_shares_in_given_bonds_out_down(&self, amount_in: &str) -> PyResult<String> {
        let amount_in_fp = FixedPoint::from(U256::from_dec_str(amount_in).map_err(|err| {
            PyErr::new::<PyValueError, _>(format!(
                "Failed to convert amount_in string {} to U256: {}",
                amount_in, err
            ))
        })?);
        let result_fp = self
            .state
            .calculate_shares_in_given_bonds_out_down(amount_in_fp)
            .map_err(|err| {
                PyErr::new::<PyValueError, _>(format!(
                    "calculate_shares_in_given_bonds_out_down: {}",
                    err
                ))
            })?;
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }

    pub fn calculate_shares_out_given_bonds_in_down(&self, amount_in: &str) -> PyResult<String> {
        let amount_in_fp = FixedPoint::from(U256::from_dec_str(amount_in).map_err(|err| {
            PyErr::new::<PyValueError, _>(format!(
                "Failed to convert amount_in string {} to U256: {}",
                amount_in, err
            ))
        })?);
        let result_fp = self
            .state
            .calculate_shares_out_given_bonds_in_down(amount_in_fp)
            .map_err(|err| {
                PyErr::new::<PyValueError, _>(format!(
                    "calculate_shares_out_given_bonds_in_down: {}",
                    err
                ))
            })?;
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }
}
