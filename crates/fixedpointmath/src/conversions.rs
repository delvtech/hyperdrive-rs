use std::fmt::Debug;

use ethers::types::{I256, U256};

use crate::{FixedPoint, FixedPointValue};

impl<T: FixedPointValue> From<T> for FixedPoint<T> {
    fn from(value: T) -> Self {
        Self::new(value).unwrap()
    }
}

/// Implements the `Fixed` trait and conversions to `FixedPoint<T>` for a list
/// of values that can be converted to `T`.
macro_rules! impl_from_nested_raw {
    ($($t:ty),*) => {
        $(
            impl<T: FixedPointValue + From<$t>> From<$t> for FixedPoint<T> {
                fn from(u: $t) -> Self {
                    Self::new(u).unwrap()
                }
            }

            impl Fixed for $t {}
        )*
    };
    ($($tt:tt)*) => {};
}

// Any `FixedPointValue`` that can be created from these types can be converted
// to a `FixedPoint` instance.
impl_from_nested_raw!(usize, isize, u64, i64, u32, i32, u16, i16, u8, i8, [u8; 32], bool);

/// Implements the `Fixed` trait and try conversions to `FixedPoint<T>` for a
/// list of values that can be converted to `T`.
macro_rules! impl_mapped_try_into {
    ($($t:ty => $fn:ident),*) => {
        $(
            impl<T: FixedPointValue> TryFrom<FixedPoint<T>> for $t {
                type Error = eyre::ErrReport;

                fn try_from(f: FixedPoint<T>) -> eyre::Result<Self> {
                    f.$fn()
                }
            }
        )*
    };
    ($($tt:tt)*) => {};
}

impl_mapped_try_into!(
    U256 => to_u256,
    I256 => to_i256,
    u128 => to_u128,
    i128 => to_i128
);

// Any value that implements Into<FixedPointValue> can be converted to a
// FixedPoint<FixedPointValue> via `fixed()`.
pub trait Fixed: Sized + Debug {
    fn fixed<T: FixedPointValue + From<Self>>(self) -> FixedPoint<T> {
        let value: T = self.into();
        FixedPoint::from(value)
    }
}

/// Convert a aw value to a `FixedPoint` instance.
pub fn fixed<T: FixedPointValue, R: Into<T>>(raw: R) -> FixedPoint<T> {
    let raw: T = raw.into();
    FixedPoint::from(raw)
}

#[cfg(test)]
mod tests {
    use crate::{fixed, FixedPoint};

    #[test]
    fn test_fixed() -> eyre::Result<()> {
        assert_eq!(fixed(1), FixedPoint::<i128>::from(1));
        assert_eq!(fixed(1_000), FixedPoint::<i128>::from(1_000));
        assert_eq!(fixed(-1_000), FixedPoint::<i128>::from(-1_000));
        assert_eq!(
            fixed(5_500_000_000_i128),
            FixedPoint::<i128>::from(5_500_000_000_i128)
        );

        Ok(())
    }
}
