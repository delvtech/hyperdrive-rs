mod long;
mod short;
// mod lp;

use ethers::core::types::{I256, U256};
use fixed_point::FixedPoint;
use hyperdrive_math::{State, YieldSpace};
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

    pub fn calculate_solvency(&self) -> PyResult<String> {
        let result_fp = self.state.calculate_solvency();
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

    pub fn calculate_pool_deltas_after_add_liquidity(
        &self,
        contribution: &str,
    ) -> PyResult<(String, String, String)> {
        let contribution_fp = FixedPoint::from(U256::from_dec_str(contribution).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert contribution string to U256")
        })?);
        let (result_fp1, result_fp2, result_fp3) = self
            .state
            .calculate_pool_deltas_after_add_liquidity(contribution_fp)
            .map_err(|err| {
                PyErr::new::<PyValueError, _>(format!(
                    "calculate_pool_deltas_after_add_liquidity returned the error: {:?}",
                    err
                ))
            })?;
        let result1 = U256::from(result_fp1).to_string();
        let result2 = result_fp2.to_string();
        let result3 = U256::from(result_fp3).to_string();
        Ok((result1, result2, result3))
    }

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

    pub fn calculate_max_spot_price(&self) -> PyResult<String> {
        let result_fp = self.state.calculate_max_spot_price();
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }

    pub fn calculate_max_short(
        &self,
        budget: &str,
        open_vault_share_price: &str,
        checkpoint_exposure: &str,
        maybe_conservative_price: Option<&str>,
        maybe_max_iterations: Option<usize>,
    ) -> PyResult<String> {
        let budget_fp = FixedPoint::from(U256::from_dec_str(budget).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert budget string to U256")
        })?);
        let open_vault_share_price_fp =
            FixedPoint::from(U256::from_dec_str(open_vault_share_price).map_err(|_| {
                PyErr::new::<PyValueError, _>(
                    "Failed to convert open_vault_share_price string to U256",
                )
            })?);
        let checkpoint_exposure_i = I256::from_dec_str(checkpoint_exposure).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert checkpoint_exposure string to I256")
        })?;
        let maybe_conservative_price_fp = if let Some(conservative_price) = maybe_conservative_price
        {
            Some(FixedPoint::from(
                U256::from_dec_str(conservative_price).map_err(|_| {
                    PyErr::new::<PyValueError, _>(
                        "Failed to convert maybe_conservative_price string to U256",
                    )
                })?,
            ))
        } else {
            None
        };
        let result_fp = self.state.calculate_max_short(
            budget_fp,
            open_vault_share_price_fp,
            checkpoint_exposure_i,
            maybe_conservative_price_fp,
            maybe_max_iterations,
        );
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }

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

    pub fn calculate_idle_share_reserves_in_base(&self) -> PyResult<String> {
        let result_fp = self.state.calculate_idle_share_reserves_in_base();
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }

    pub fn calculate_bonds_out_given_shares_in_down(&self, amount_in: &str) -> PyResult<String> {
        let amount_in_fp = FixedPoint::from(U256::from_dec_str(amount_in).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert amount_in string to U256")
        })?);
        let result_fp = self
            .state
            .calculate_bonds_out_given_shares_in_down(amount_in_fp);
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }

    pub fn calculate_shares_in_given_bonds_out_up(&self, amount_in: &str) -> PyResult<String> {
        let amount_in_fp = FixedPoint::from(U256::from_dec_str(amount_in).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert amount_in string to U256")
        })?);
        // We unwrap the error here to throw panic error if this fails
        let result_fp = self
            .state
            .calculate_shares_in_given_bonds_out_up_safe(amount_in_fp)
            .unwrap();
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }

    pub fn calculate_shares_in_given_bonds_out_down(&self, amount_in: &str) -> PyResult<String> {
        let amount_in_fp = FixedPoint::from(U256::from_dec_str(amount_in).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert amount_in string to U256")
        })?);
        let result_fp = self
            .state
            .calculate_shares_in_given_bonds_out_down(amount_in_fp);
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }

    pub fn calculate_shares_out_given_bonds_in_down(&self, amount_in: &str) -> PyResult<String> {
        let amount_in_fp = FixedPoint::from(U256::from_dec_str(amount_in).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert amount_in string to U256")
        })?);
        let result_fp = self
            .state
            .calculate_shares_out_given_bonds_in_down(amount_in_fp);
        let result = U256::from(result_fp).to_string();
        Ok(result)
    }

    pub fn to_checkpoint(&self, time: &str) -> PyResult<String> {
        let time_int = U256::from_dec_str(time)
            .map_err(|_| PyErr::new::<PyValueError, _>("Failed to convert time string to U256"))?;
        let result_int = self.state.to_checkpoint(time_int);
        let result = result_int.to_string();
        Ok(result)
    }
}
