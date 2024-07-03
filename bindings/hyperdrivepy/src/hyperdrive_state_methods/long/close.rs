use ethers::core::types::U256;
use fixedpointmath::FixedPoint;
use pyo3::{exceptions::PyValueError, prelude::*};

use crate::HyperdriveState;

#[pymethods]
impl HyperdriveState {
    pub fn calculate_close_long(
        &self,
        bond_amount: &str,
        maturity_time: &str,
        current_time: &str,
    ) -> PyResult<String> {
        let bond_amount_fp = FixedPoint::from(U256::from_dec_str(bond_amount).map_err(|err| {
            PyErr::new::<PyValueError, _>(format!(
                "Failed to convert bond_amount string {} to U256: {}",
                bond_amount, err
            ))
        })?);
        let maturity_time = U256::from_dec_str(maturity_time).map_err(|err| {
            PyErr::new::<PyValueError, _>(format!(
                "Failed to convert maturity_time string {} to U256: {}",
                maturity_time, err
            ))
        })?;
        let current_time = U256::from_dec_str(current_time).map_err(|err| {
            PyErr::new::<PyValueError, _>(format!(
                "Failed to convert current_time string {} to U256: {}",
                current_time, err
            ))
        })?;

        let result_fp = self
            .state
            .calculate_close_long(bond_amount_fp, maturity_time, current_time)
            .map_err(|err| {
                PyErr::new::<PyValueError, _>(format!("calculate_close_long: {}", err))
            })?;
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }
}
