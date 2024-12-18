use ethers::core::types::{I256, U256};
use fixedpointmath::FixedPoint;
use pyo3::{exceptions::PyValueError, prelude::*};

use crate::HyperdriveState;

#[pymethods]
impl HyperdriveState {
    pub fn calculate_targeted_long_with_budget(
        &self,
        budget: &str,
        target_rate: &str,
        maybe_max_iterations: Option<usize>,
        maybe_allowable_error: Option<&str>,
    ) -> PyResult<String> {
        let budget_fp = FixedPoint::from(U256::from_dec_str(budget).map_err(|err| {
            PyErr::new::<PyValueError, _>(format!(
                "Failed to convert budget string {} to U256: {}",
                budget, err
            ))
        })?);
        let target_rate_fp = FixedPoint::from(U256::from_dec_str(target_rate).map_err(|err| {
            PyErr::new::<PyValueError, _>(format!(
                "Failed to convert target_rate string {} to U256: {}",
                target_rate, err
            ))
        })?);
        let maybe_allowable_error_fp = if let Some(allowable_error) = maybe_allowable_error {
            Some(FixedPoint::from(
                U256::from_dec_str(allowable_error).map_err(|err| {
                    PyErr::new::<PyValueError, _>(format!(
                        "Failed to convert maybe_allowable_error string {} to U256: {}",
                        allowable_error, err
                    ))
                })?,
            ))
        } else {
            None
        };
        let result_fp = self
            .state
            .calculate_targeted_long_with_budget(
                budget_fp,
                target_rate_fp,
                maybe_max_iterations,
                maybe_allowable_error_fp,
            )
            .map_err(|err| {
                PyErr::new::<PyValueError, _>(format!(
                    "calculate_targeted_long_with_budget: {:?}",
                    err
                ))
            })?;
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }
}
