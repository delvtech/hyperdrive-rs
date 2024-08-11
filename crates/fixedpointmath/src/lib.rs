//! A generic fixed point wrapper around the `U256` type from ethers-rs.
//!
//! The math in this library is a heavily based on Solidity's FixedPointMath
//! library with a few changes:
//! - The outward type of the underlying value is generic, allowing the library
//!   to be used with any type that implements `FixedPointValue`, including
//!   signed integers, and ensuring that the instance is bounded by the generic
//!   type's limits.
//! - Support for overflowing intermediate operations in `mul_div_down` and
//!  `mul_div_up` via `U512`.
//!
//! Each of the functions is fuzz tested against the Solidity implementation to
//! ensure that the behavior is identical given values bounded by the Solidity
//! implementation's limits.

mod fixed_point;
mod macros;
mod math;
mod rng;
mod sign;
mod utils;
mod value;
mod value_impls;

pub use fixed_point::*;
pub use rng::*;
pub use sign::*;
pub use utils::*;
pub use value::*;

pub mod prelude {
    pub use super::{
        fixed, fixed_i128, fixed_i256,
        fixed_point::{Fixed, FixedPoint, ToFixed},
        fixed_u128, fixed_u256, int256, uint256,
        value::FixedPointValue,
    };
}
