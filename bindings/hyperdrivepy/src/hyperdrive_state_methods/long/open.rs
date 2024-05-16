use ethers::core::types::{I256, U256};
use fixed_point::FixedPoint;
use hyperdrive_math::{State, YieldSpace};
use pyo3::{exceptions::PyValueError, prelude::*};

use crate::HyperdriveState;
pub use crate::{utils::*, PyPoolConfig, PyPoolInfo};

#[pymethods]
impl HyperdriveState {
    pub fn calculate_open_long(&self, base_amount: &str) -> PyResult<String> {
        let base_amount_fp = FixedPoint::from(U256::from_dec_str(base_amount).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert base_amount string to U256")
        })?);
        let result_fp = self.state.calculate_open_long(base_amount_fp).unwrap();
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }

    pub fn calculate_pool_deltas_after_open_long(&self, base_amount: &str) -> PyResult<String> {
        let base_amount_fp = FixedPoint::from(U256::from_dec_str(base_amount).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert base_amount string to U256")
        })?);
        let result_fp = self
            .state
            .calculate_pool_deltas_after_open_long(base_amount_fp)
            .map_err(|err| {
                PyErr::new::<PyValueError, _>(format!(
                    "calculate_pool_deltas_after_open_long returned the error: {:?}",
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
        let base_amount_fp = FixedPoint::from(U256::from_dec_str(base_amount).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert base_amount string to U256")
        })?);
        let maybe_bond_amount_fp = if let Some(bond_amount) = maybe_bond_amount {
            Some(FixedPoint::from(U256::from_dec_str(bond_amount).map_err(
                |_| {
                    PyErr::new::<PyValueError, _>(
                        "Failed to convert maybe_bond_amount string to U256",
                    )
                },
            )?))
        } else {
            None
        };
        let result_fp = self
            .state
            .calculate_spot_price_after_long(base_amount_fp, maybe_bond_amount_fp)
            .unwrap();
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }

    pub fn calculate_spot_rate_after_long(
        &self,
        base_amount: &str,
        maybe_bond_amount: Option<&str>,
    ) -> PyResult<String> {
        let base_amount_fp = FixedPoint::from(U256::from_dec_str(base_amount).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert base_amount string to U256")
        })?);
        let maybe_bond_amount_fp = if let Some(bond_amount) = maybe_bond_amount {
            Some(FixedPoint::from(U256::from_dec_str(bond_amount).map_err(
                |_| {
                    PyErr::new::<PyValueError, _>(
                        "Failed to convert maybe_bond_amount string to U256",
                    )
                },
            )?))
        } else {
            None
        };
        let result_fp = self
            .state
            .calculate_spot_rate_after_long(base_amount_fp, maybe_bond_amount_fp)
            .unwrap();
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }
}
