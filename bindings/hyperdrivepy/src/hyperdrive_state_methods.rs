mod long;
mod lp;
mod short;
mod yield_space;

use ethers::core::types::U256;
use hyperdrive_math::State;
use pyo3::{exceptions::PyValueError, prelude::*};

use crate::HyperdriveState;
pub use crate::{utils::*, PyPoolConfig, PyPoolInfo};

#[pymethods]
impl HyperdriveState {
    #[new]
    pub fn __init__(pool_config: &PyAny, pool_info: &PyAny) -> PyResult<Self> {
        let rust_pool_config = PyPoolConfig::extract(pool_config)?.pool_config;
        let rust_pool_info = PyPoolInfo::extract(pool_info)?.pool_info;
        let state = State::new(rust_pool_config, rust_pool_info);
        Ok(HyperdriveState::new(state))
    }

    pub fn to_checkpoint(&self, time: &str) -> PyResult<String> {
        let time_int = U256::from_dec_str(time)
            .map_err(|_| PyErr::new::<PyValueError, _>("Failed to convert time string to U256"))?;
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
        let result_fp = self.state.calculate_spot_price();
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }

    pub fn calculate_spot_rate(&self) -> PyResult<String> {
        let result_fp = self.state.calculate_spot_rate();
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }

    pub fn calculate_max_spot_price(&self) -> PyResult<String> {
        let result_fp = self.state.calculate_max_spot_price();
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }

    pub fn calculate_idle_share_reserves_in_base(&self) -> PyResult<String> {
        let result_fp = self.state.calculate_idle_share_reserves_in_base();
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }
}
