use ethers::types::{I256, U256};
use eyre::eyre;

use crate::{FixedPoint, FixedPointValue};

/// Implements [`FixedPointValue`] and conversion traits for the given type.
///
/// # Example
///
/// ```rs
/// fixed_point_value_impl!(
///     type = U256,
///     MAX = U256::MAX,
///     MIN = U256::zero(),
///     from = u128,
///     try_from = i128 | I256,
/// );
/// ```
///
/// # Parameters
/// - `type`: The type to implement `FixedPointValue` for.
/// - `MAX`: The maximum value of the type. `-(2^256-1)..=(2^256-1)`.
/// - `MIN`: The minimum value of the type. `0..=(2^256-1)`.
/// - `MAX_DECIMALS`: *(Optional)* The maximum number of decimal places the value can support.
/// - `from`: *(Optional)* Other types that can convert to the given type.
/// - `try_from`: *(Optional)* Other types that can try to convert to the given type.
#[macro_export]
macro_rules! fixed_point_value_impl {
  (
      type = $t:ty,
      MAX = $max:expr,
      MIN = $min:expr$(,
      MAX_DECIMALS = $decimals:expr)?$(,
      from = $($from:ty)|+)?$(,
      try_from = $($try_from:ty)|+)?$(,)?
  ) => {
    impl FixedPointValue for $t {
        const MIN: Self = $min;
        const MAX: Self = $max;
        $(
            const MAX_DECIMALS: u8 = $decimals;
        )?
    }


    impl From<FixedPoint<$t>> for $t {
        fn from(f: FixedPoint<$t>) -> Self {
            f.raw()
        }
    }

    $(
      $(
        impl From<$from> for FixedPoint<$t> {
            fn from(f: $from) -> Self {
                FixedPoint::new(f)
            }
        }

        impl TryFrom<FixedPoint<$t>> for $from {
            type Error = eyre::ErrReport;

            fn try_from(value: FixedPoint<$t>) -> eyre::Result<Self> {
                value.raw().try_into().map_err(|_| {
                    eyre!(
                        "Failed to convert underlying FixedPointValue of type {type} to {to_type}: {value:?}",
                        type = stringify!($t),
                        to_type = stringify!($from),
                        value = value.raw()
                    )
                })
            }
          }
      )+
    )*
    $(
      $(
        impl TryFrom<$try_from> for FixedPoint<$t> {
            type Error = eyre::ErrReport;

            fn try_from(value: $try_from) -> eyre::Result<Self> {
                FixedPoint::try_from(value)
            }
        }

        impl TryFrom<FixedPoint<$t>> for $try_from {
            type Error = eyre::ErrReport;

            fn try_from(value: FixedPoint<$t>) -> eyre::Result<Self> {
                value.raw().try_into().map_err(|_| {
                    eyre!(
                        "Failed to convert underlying FixedPointValue of type {type} to {to_type}: {value:?}",
                        type = stringify!($t),
                        to_type = stringify!($try_from),
                        value = value.raw()
                    )
                })
            }
          }
      )+
    )*
  }
}

fixed_point_value_impl!(
    type = i128,
    MAX = i128::MAX,
    MIN = i128::MIN,
    try_from = u128 | I256 | U256,
);

fixed_point_value_impl!(
    type = u128,
    MAX = u128::MAX,
    MIN = u128::MIN,
    try_from = i128 | I256 | U256,
);

fixed_point_value_impl!(
    type = I256,
    MAX = I256::MAX,
    MIN = I256::MIN,
    from = i128 | u128,
    try_from = U256,
);

fixed_point_value_impl!(
    type = U256,
    MAX = U256::MAX,
    MIN = U256::zero(),
    from = u128,
    try_from = i128 | I256,
);
