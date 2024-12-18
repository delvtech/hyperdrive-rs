use ethers::core::types::{I256, U256};
use fixedpointmath::FixedPoint;
use pyo3::{exceptions::PyValueError, prelude::*};

use crate::HyperdriveState;

#[pymethods]
impl HyperdriveState {
    pub fn calculate_max_long(
        &self,
        budget: &str,
        maybe_max_iterations: Option<usize>,
    ) -> PyResult<String> {
        let budget_fp = FixedPoint::from(U256::from_dec_str(budget).map_err(|err| {
            PyErr::new::<PyValueError, _>(format!(
                "Failed to convert budget string {} to U256: {}",
                budget, err
            ))
        })?);
        let result_fp = self
            .state
            .calculate_max_long(budget_fp, maybe_max_iterations)
            .map_err(|err| PyErr::new::<PyValueError, _>(format!("calculate_max_long: {}", err)))?;
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }
}
