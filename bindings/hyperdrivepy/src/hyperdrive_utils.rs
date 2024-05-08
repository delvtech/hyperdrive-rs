use ethers::core::types::{I256, U256};
use fixed_point::FixedPoint;
use hyperdrive_math::{
    calculate_effective_share_reserves as rs_calculate_effective_share_reserves,
    calculate_time_stretch as rs_calculate_time_stretch,
};
use pyo3::{exceptions::PyValueError, prelude::*};

#[pyfunction]
pub fn calculate_effective_share_reserves(
    share_reserves: &str,
    share_adjustment: &str,
) -> PyResult<String> {
    let share_reserves_fp = FixedPoint::from(U256::from_dec_str(share_reserves).map_err(|_| {
        PyErr::new::<PyValueError, _>("Failed to convert share_reserves string to U256")
    })?);
    let share_adjustment_i = I256::from_dec_str(share_adjustment).map_err(|_| {
        PyErr::new::<PyValueError, _>("Failed to convert share_adjustment string to I256")
    })?;
    let result_fp = rs_calculate_effective_share_reserves(share_reserves_fp, share_adjustment_i);
    let result = U256::from(result_fp).to_string();
    Ok(result)
}

#[pyfunction]
pub fn calculate_time_stretch(rate: &str, position_duration: &str) -> PyResult<String> {
    let rate_fp = FixedPoint::from(
        U256::from_dec_str(rate)
            .map_err(|_| PyErr::new::<PyValueError, _>("Failed to convert rate string to U256"))?,
    );
    let position_duration_fp = FixedPoint::from(
        U256::from_dec_str(position_duration)
            .map_err(|_| PyErr::new::<PyValueError, _>("Failed to convert rate string to U256"))?,
    );
    let result_fp = rs_calculate_time_stretch(rate_fp, position_duration_fp);
    let result = U256::from(result_fp).to_string();
    Ok(result)
}
