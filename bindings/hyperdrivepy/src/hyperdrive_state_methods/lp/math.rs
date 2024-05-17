use ethers::core::types::U256;
use pyo3::{exceptions::PyValueError, prelude::*};

use crate::HyperdriveState;

#[pymethods]
impl HyperdriveState {
    pub fn calculate_present_value(&self, current_block_timestamp: &str) -> PyResult<String> {
        let current_block_timestamp_int =
            U256::from_dec_str(current_block_timestamp).map_err(|_| {
                PyErr::new::<PyValueError, _>(
                    "Failed to convert current_block_timestamp string to U256",
                )
            })?;
        match self
            .state
            .calculate_present_value(current_block_timestamp_int)
        {
            Ok(result) => Ok(U256::from(result).to_string()),
            Err(err) => Err(PyErr::new::<PyValueError, _>(format!("{:?}", err))),
        }
    }
}
