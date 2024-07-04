use ethers::core::types::{I256, U256};
use fixedpointmath::FixedPoint;
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
        let budget_fp = FixedPoint::from(U256::from_dec_str(budget).map_err(|err| {
            PyErr::new::<PyValueError, _>(format!(
                "Failed to convert budget string {} to U256: {}",
                budget, err
            ))
        })?);
        let checkpoint_exposure_i = I256::from_dec_str(checkpoint_exposure).map_err(|err| {
            PyErr::new::<PyValueError, _>(format!(
                "Failed to convert checkpoint_exposure string {} to I256: {}",
                checkpoint_exposure, err
            ))
        })?;
        let result_fp = self
            .state
            .calculate_max_long(budget_fp, checkpoint_exposure_i, maybe_max_iterations)
            .map_err(|err| PyErr::new::<PyValueError, _>(format!("calculate_max_long: {}", err)))?;
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }
}
