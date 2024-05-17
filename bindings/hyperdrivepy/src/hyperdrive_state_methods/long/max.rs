use ethers::core::types::{I256, U256};
use fixed_point::FixedPoint;
use pyo3::{exceptions::PyValueError, prelude::*};

use crate::HyperdriveState;

#[pymethods]
impl HyperdriveState {
    pub fn calculate_max_long(
        &self,
        budget: &str,
        checkpoint_exposure: &str,
        maybe_max_iterations: Option<usize>,
    ) -> PyResult<String> {
        let budget_fp = FixedPoint::from(U256::from_dec_str(budget).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert budget string to U256")
        })?);
        let checkpoint_exposure_i = I256::from_dec_str(checkpoint_exposure).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert checkpoint_exposure string to I256")
        })?;
        let result_fp = self
            .state
            .calculate_max_long(budget_fp, checkpoint_exposure_i, maybe_max_iterations)
            .map_err(|e| {
                PyErr::new::<PyValueError, _>(format!("Failed to calculate max long: {}", e))
            })?;
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }
}
