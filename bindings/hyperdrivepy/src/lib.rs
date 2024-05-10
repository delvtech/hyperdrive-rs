mod hyperdrive_state;
mod hyperdrive_state_methods;
mod hyperdrive_utils;
mod pool_config;
mod pool_info;
mod utils;

use hyperdrive_state::HyperdriveState;
pub use hyperdrive_state_methods::*;
pub use hyperdrive_utils::{
    calculate_bonds_given_effective_shares_and_rate, calculate_effective_share_reserves,
    calculate_rate_given_fixed_price, calculate_time_stretch,
};
pub use pool_config::PyPoolConfig;
pub use pool_info::PyPoolInfo;
use pyo3::prelude::*;

/// Get the share reserves after subtracting the adjustment used for
/// A pyO3 wrapper for the hyperdrive_math crate.
#[pymodule]
#[pyo3(name = "hyperdrivepy")]
fn hyperdrivepy(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<HyperdriveState>()?;
    m.add_function(wrap_pyfunction!(
        calculate_bonds_given_effective_shares_and_rate,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(calculate_effective_share_reserves, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_rate_given_fixed_price, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_time_stretch, m)?)?;
    Ok(())
}
