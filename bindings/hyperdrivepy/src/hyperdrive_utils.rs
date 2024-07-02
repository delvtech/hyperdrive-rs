use ethers::core::types::{I256, U256};
use fixedpointmath::FixedPoint;
use hyperdrive_math::{
    calculate_bonds_given_effective_shares_and_rate as rs_calculate_bonds_given_effective_shares_and_rate,
    calculate_effective_share_reserves as rs_calculate_effective_share_reserves,
    calculate_rate_given_fixed_price as rs_calculate_rate_given_fixed_price,
    calculate_time_stretch as rs_calculate_time_stretch,
};
use pyo3::{exceptions::PyValueError, prelude::*};

#[pyfunction]
pub fn calculate_bonds_given_effective_shares_and_rate(
    effective_share_reserves: &str,
    target_rate: &str,
    initial_vault_share_price: &str,
    position_duration: &str,
    time_stretch: &str,
) -> PyResult<String> {
    let effective_share_reserves_fp =
        FixedPoint::from(U256::from_dec_str(effective_share_reserves).map_err(|err| {
            PyErr::new::<PyValueError, _>(format!(
                "Failed to convert effective_share_reserves string {} to U256: {}",
                effective_share_reserves, err
            ))
        })?);
    let target_rate_fp = FixedPoint::from(U256::from_dec_str(target_rate).map_err(|err| {
        PyErr::new::<PyValueError, _>(format!(
            "Failed to convert target_rate string {} to U256: {}",
            target_rate, err
        ))
    })?);
    let initial_vault_share_price_fp = FixedPoint::from(
        U256::from_dec_str(initial_vault_share_price).map_err(|err| {
            PyErr::new::<PyValueError, _>(format!(
                "Failed to convert initial_vault_share_price string {} to U256: {}",
                initial_vault_share_price, err
            ))
        })?,
    );
    let position_duration_fp =
        FixedPoint::from(U256::from_dec_str(position_duration).map_err(|err| {
            PyErr::new::<PyValueError, _>(format!(
                "Failed to convert position_duration string {} to U256: {}",
                position_duration, err
            ))
        })?);
    let time_stretch_fp = FixedPoint::from(U256::from_dec_str(time_stretch).map_err(|err| {
        PyErr::new::<PyValueError, _>(format!(
            "Failed to convert time_stretch string {} to U256: {}",
            time_stretch, err
        ))
    })?);
    let result_fp = rs_calculate_bonds_given_effective_shares_and_rate(
        effective_share_reserves_fp,
        target_rate_fp,
        initial_vault_share_price_fp,
        position_duration_fp,
        time_stretch_fp,
    )
    .map_err(|err| {
        PyErr::new::<PyValueError, _>(format!(
            "calculate_bonds_given_effective_shares_and_rate: {}",
            err
        ))
    })?;
    let result = U256::from(result_fp).to_string();
    Ok(result)
}

#[pyfunction]
pub fn calculate_effective_share_reserves(
    share_reserves: &str,
    share_adjustment: &str,
) -> PyResult<String> {
    let share_reserves_fp =
        FixedPoint::from(U256::from_dec_str(share_reserves).map_err(|err| {
            PyErr::new::<PyValueError, _>(format!(
                "Failed to convert share_reserves string {} to U256: {}",
                share_reserves, err
            ))
        })?);
    let share_adjustment_i = I256::from_dec_str(share_adjustment).map_err(|err| {
        PyErr::new::<PyValueError, _>(format!(
            "Failed to convert share_adjustment string {} to I256: {}",
            share_adjustment, err
        ))
    })?;
    let result_fp = rs_calculate_effective_share_reserves(share_reserves_fp, share_adjustment_i)
        .map_err(|err| {
            PyErr::new::<PyValueError, _>(format!("calculate_effective_share_reserves: {}", err))
        })?;
    let result = U256::from(result_fp).to_string();
    Ok(result)
}

#[pyfunction]
pub fn calculate_rate_given_fixed_price(price: &str, position_duration: &str) -> PyResult<String> {
    let price_fp = FixedPoint::from(U256::from_dec_str(price).map_err(|err| {
        PyErr::new::<PyValueError, _>(format!(
            "Failed to convert price string {} to U256: {}",
            price, err
        ))
    })?);
    let position_duration_fp =
        FixedPoint::from(U256::from_dec_str(position_duration).map_err(|err| {
            PyErr::new::<PyValueError, _>(format!(
                "Failed to convert position_duration string {} to U256: {}",
                position_duration, err
            ))
        })?);
    let result_fp = rs_calculate_rate_given_fixed_price(price_fp, position_duration_fp);
    let result = U256::from(result_fp).to_string();
    Ok(result)
}

#[pyfunction]
pub fn calculate_time_stretch(rate: &str, position_duration: &str) -> PyResult<String> {
    let rate_fp = FixedPoint::from(U256::from_dec_str(rate).map_err(|err| {
        PyErr::new::<PyValueError, _>(format!(
            "Failed to convert rate string {} to U256: {}",
            rate, err
        ))
    })?);
    let position_duration_fp =
        FixedPoint::from(U256::from_dec_str(position_duration).map_err(|err| {
            PyErr::new::<PyValueError, _>(format!(
                "Failed to convert position_duration string {} to U256: {}",
                position_duration, err
            ))
        })?);
    let result_fp = rs_calculate_time_stretch(rate_fp, position_duration_fp)
        .map_err(|err| PyErr::new::<PyValueError, _>(format!("calculate_time_stretch: {}", err)))?;
    let result = U256::from(result_fp).to_string();
    Ok(result)
}
