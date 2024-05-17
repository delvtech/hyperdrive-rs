use ethers::core::types::U256;
use fixed_point::FixedPoint;
use pyo3::{exceptions::PyValueError, prelude::*};

use crate::HyperdriveState;

#[pymethods]
impl HyperdriveState {
    pub fn calculate_open_short(
        &self,
        bond_amount: &str,
        open_vault_share_price: &str,
    ) -> PyResult<String> {
        let bond_amount_fp = FixedPoint::from(U256::from_dec_str(bond_amount).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert bond_amount string to U256")
        })?);
        let open_vault_share_price_fp =
            FixedPoint::from(U256::from_dec_str(open_vault_share_price).map_err(|_| {
                PyErr::new::<PyValueError, _>(
                    "Failed to convert open_vault_share_price string to U256",
                )
            })?);
        let result_fp = self
            .state
            .calculate_open_short(bond_amount_fp, open_vault_share_price_fp)
            .unwrap();
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }

    pub fn calculate_pool_deltas_after_open_short(&self, bond_amount: &str) -> PyResult<String> {
        let bond_amount_fp = FixedPoint::from(U256::from_dec_str(bond_amount).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert bond_amount string to U256")
        })?);
        let result_fp = self
            .state
            .calculate_pool_deltas_after_open_short(bond_amount_fp)
            .unwrap();
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }

    pub fn calculate_spot_price_after_short(
        &self,
        bond_amount: &str,
        maybe_base_amount: Option<&str>,
    ) -> PyResult<String> {
        let bond_amount_fp = FixedPoint::from(U256::from_dec_str(bond_amount).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert bond_amount string to U256")
        })?);
        let maybe_base_amount_fp = if let Some(base_amount) = maybe_base_amount {
            Some(FixedPoint::from(U256::from_dec_str(base_amount).map_err(
                |_| {
                    PyErr::new::<PyValueError, _>(
                        "Failed to convert maybe_base_amount string to U256",
                    )
                },
            )?))
        } else {
            None
        };
        let result_fp = self
            .state
            .calculate_spot_price_after_short(bond_amount_fp, maybe_base_amount_fp)
            .map_err(|_| {
                PyErr::new::<PyValueError, _>("Failed to calculate spot price after short.")
            })?;
        Ok(U256::from(result_fp).to_string())
    }
}
