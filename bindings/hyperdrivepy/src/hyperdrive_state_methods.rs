mod long;
mod lp;
mod short;
mod yield_space;

use ethers::core::types::U256;
use pyo3::{exceptions::PyValueError, prelude::*};

pub use crate::utils::*;
use crate::HyperdriveState;

#[pymethods]
impl HyperdriveState {
    #[new]
    pub fn __init__(pool_config: &PyAny, pool_info: &PyAny) -> PyResult<Self> {
        HyperdriveState::new_from_pool(pool_config, pool_info)
    }

    pub fn to_checkpoint(&self, time: &str) -> PyResult<String> {
        let time_int = U256::from_dec_str(time).map_err(|err| {
            PyErr::new::<PyValueError, _>(format!(
                "Failed to convert time string {} to U256: {}",
                time, err
            ))
        })?;
        let result_int = self.state.to_checkpoint(time_int);
        let result = result_int.to_string();
        Ok(result)
    }

    pub fn calculate_solvency(&self) -> PyResult<String> {
        let result_fp = self.state.calculate_solvency();
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }

    pub fn calculate_spot_price(&self) -> PyResult<String> {
        let result_fp = self.state.calculate_spot_price().map_err(|err| {
            PyErr::new::<PyValueError, _>(format!("calculate_spot_price: {}", err))
        })?;
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }

    pub fn calculate_spot_rate(&self) -> PyResult<String> {
        let result_fp = self.state.calculate_spot_rate().map_err(|err| {
            PyErr::new::<PyValueError, _>(format!("calculate_spot_rate: {}", err))
        })?;
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }

    pub fn calculate_max_spot_price(&self) -> PyResult<String> {
        let result_fp = self.state.calculate_max_spot_price().map_err(|err| {
            PyErr::new::<PyValueError, _>(format!("calculate_max_spot_price: {}", err))
        })?;
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }

    pub fn calculate_idle_share_reserves_in_base(&self) -> PyResult<String> {
        let result_fp = self.state.calculate_idle_share_reserves_in_base();
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }
}
