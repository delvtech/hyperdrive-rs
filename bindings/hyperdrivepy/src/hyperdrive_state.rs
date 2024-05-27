use hyperdrive_math::State;
use pyo3::{exceptions::PyAssertionError, prelude::*};

use crate::{PyPoolConfig, PyPoolInfo};

#[pyclass(module = "hyperdrivepy", name = "HyperdriveState")]
pub struct HyperdriveState {
    pub state: State,
}

impl HyperdriveState {
    pub(crate) fn new(state: State) -> Self {
        HyperdriveState { state }
    }

    pub(crate) fn new_from_pool(pool_config: &PyAny, pool_info: &PyAny) -> PyResult<Self> {
        let rust_pool_config = match PyPoolConfig::extract(pool_config) {
            Ok(py_pool_config) => py_pool_config.pool_config,
            Err(err) => {
                return Err(PyErr::new::<PyAssertionError, _>(format!(
                    "Error extracting pool config {:?}: {}",
                    pool_config, err
                )));
            }
        };
        let rust_pool_info = match PyPoolInfo::extract(pool_info) {
            Ok(py_pool_info) => py_pool_info.pool_info,
            Err(err) => {
                return Err(PyErr::new::<PyAssertionError, _>(format!(
                    "Error extracting pool info {:?}: {}",
                    pool_info, err
                )));
            }
        };
        let state = State::new(rust_pool_config, rust_pool_info);
        Ok(HyperdriveState::new(state))
    }
}

impl From<State> for HyperdriveState {
    fn from(state: State) -> Self {
        HyperdriveState::new(state)
    }
}

impl TryFrom<(&PyAny, &PyAny)> for HyperdriveState {
    type Error = PyErr;

    fn try_from(args: (&PyAny, &PyAny)) -> PyResult<Self> {
        HyperdriveState::new_from_pool(args.0, args.1)
    }
}
