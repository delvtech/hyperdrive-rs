use ethers::core::types::U256;
use fixedpointmath::FixedPoint;
use pyo3::{exceptions::PyValueError, prelude::*};

use crate::HyperdriveState;

#[pymethods]
impl HyperdriveState {
    pub fn calculate_open_long(&self, base_amount: &str) -> PyResult<String> {
        let base_amount_fp = FixedPoint::from(U256::from_dec_str(base_amount).map_err(|err| {
            PyErr::new::<PyValueError, _>(format!(
                "Failed to convert base_amount string {} to U256: {}",
                base_amount, err
            ))
        })?);
        let result_fp = self
            .state
            .calculate_open_long(base_amount_fp)
            .map_err(|err| {
                PyErr::new::<PyValueError, _>(format!("calculate_open_long: {}", err))
            })?;
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }

    pub fn calculate_pool_deltas_after_open_long(&self, base_amount: &str) -> PyResult<String> {
        let base_amount_fp = FixedPoint::from(U256::from_dec_str(base_amount).map_err(|err| {
            PyErr::new::<PyValueError, _>(format!(
                "Failed to convert base_amount string {} to U256: {}",
                base_amount, err
            ))
        })?);
        let result_fp = self
            .state
            .calculate_pool_deltas_after_open_long(base_amount_fp)
            .map_err(|err| {
                PyErr::new::<PyValueError, _>(format!(
                    "calculate_pool_deltas_after_open_long: {:?}",
                    err
                ))
            })?;
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }

    pub fn calculate_spot_price_after_long(
        &self,
        base_amount: &str,
        maybe_bond_amount: Option<&str>,
    ) -> PyResult<String> {
        let base_amount_fp = FixedPoint::from(U256::from_dec_str(base_amount).map_err(|err| {
            PyErr::new::<PyValueError, _>(format!(
                "Failed to convert base_amount string {} to U256: {}",
                base_amount, err
            ))
        })?);
        let maybe_bond_amount_fp = if let Some(bond_amount) = maybe_bond_amount {
            Some(FixedPoint::from(U256::from_dec_str(bond_amount).map_err(
                |err| {
                    PyErr::new::<PyValueError, _>(format!(
                        "Failed to convert maybe_bond_amount string {} to U256: {}",
                        bond_amount, err
                    ))
                },
            )?))
        } else {
            None
        };
        let result_fp = self
            .state
            .calculate_spot_price_after_long(base_amount_fp, maybe_bond_amount_fp)
            .map_err(|err| {
                PyErr::new::<PyValueError, _>(format!("calculate_spot_price_after_long: {}", err))
            })?;
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }

    pub fn calculate_spot_rate_after_long(
        &self,
        base_amount: &str,
        maybe_bond_amount: Option<&str>,
    ) -> PyResult<String> {
        let base_amount_fp = FixedPoint::from(U256::from_dec_str(base_amount).map_err(|err| {
            PyErr::new::<PyValueError, _>(format!(
                "Failed to convert base_amount string {} to U256: {}",
                base_amount, err
            ))
        })?);
        let maybe_bond_amount_fp = if let Some(bond_amount) = maybe_bond_amount {
            Some(FixedPoint::from(U256::from_dec_str(bond_amount).map_err(
                |err| {
                    PyErr::new::<PyValueError, _>(format!(
                        "Failed to convert maybe_bond_amount string {} to U256: {}",
                        bond_amount, err
                    ))
                },
            )?))
        } else {
            None
        };
        let result_fp = self
            .state
            .calculate_spot_rate_after_long(base_amount_fp, maybe_bond_amount_fp)
            .map_err(|err| PyErr::new::<PyValueError, _>(format!("{}", err)))?;
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }
}
