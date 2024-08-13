use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, Sub, SubAssign},
};

use ethers::types::U256;
use eyre::{eyre, Result};
use paste::paste;

/// Adds `from_<type>` and `to_<type>` conversion functions for a list of types.
macro_rules! conversion_fns {
    ($($type_name:ident),*) => {
        $(
        paste! {
            fn [<from_ $type_name:snake>](value: $type_name) -> Result<Self> {
                Self::try_from(value).map_err(|_| {
                    eyre!(
                        r#"Failed to convert {type} to underlying FixedPointValue type:
    {type} value: {value:?}
    Underlying range: {min:?} to {max:?}
"#,
                        type = stringify!($type_name),
                        min = Self::MIN,
                        max = Self::MAX,
                    )
                })
            }

            fn [<to_ $type_name:snake>](self) -> Result<$type_name> {
                self.try_into().map_err(|_| {
                    eyre!(
                        "Failed to convert underlying FixedPointValue to {type}: {self:?}",
                        type = stringify!($type_name)
                   )
                })
            }
        })*
    };
}

/// A value that can be used to perform fixed-point math.
///
/// All methods have default implementations based on comparisons to `0`, but
/// can be overridden to provide more efficient alternatives.
pub trait FixedPointValue:
    Copy
    + Debug
    + Default
    + Sized
    + Eq
    + PartialEq
    + Ord
    + PartialOrd
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Rem<Output = Self>
    + TryFrom<u128>
    + TryInto<u128>
    + From<u64>
    + TryFrom<U256>
    + TryInto<U256>
{
    /// The minimum value that can be represented by the type.
    /// Must be `-2^256-1..=2^256 - 1`.
    const MIN: Self;

    /// The maximum value that can be represented by the type.
    /// Must be `0..=2^256 - 1`.
    const MAX: Self;

    /// The maximum number of decimal places the value can support.
    const MAX_DECIMALS: u8 = 18;

    /// Whether the value supports negation.
    fn is_signed() -> bool {
        Self::MIN.is_negative()
    }

    fn is_negative(&self) -> bool {
        self < &Self::from(0)
    }

    fn is_positive(&self) -> bool {
        !self.is_negative()
    }

    fn is_zero(&self) -> bool {
        self == &Self::from(0)
    }

    /// Flips the sign of the value.
    ///
    /// # Panics
    ///
    /// If the value doesn't support negation or the value overflows, i.e., the
    /// value is equal to `MIN`.
    fn flip_sign(self) -> Self {
        if !Self::is_signed() {
            panic!("Cannot flip sign of unsigned type: {self:?}");
        }
        Self::from(0) - self
    }

    /// Flips the sign of the value if the condition is true.
    ///
    /// # Panics
    ///
    /// If the value doesn't support negation or the value overflows, i.e., the
    /// value is equal to `MIN`.
    fn flip_sign_if(self, condition: bool) -> Self {
        if condition {
            self.flip_sign()
        } else {
            self
        }
    }

    /// Computes the absolute value of self.
    ///
    /// # Panics
    ///
    /// If the absolute value of self overflows `T`, e.g., if self is the
    /// minimum value of a signed integer.
    fn abs(self) -> Self {
        self.flip_sign_if(self.is_negative())
    }

    /// Computes the absolute value of self as a `U256` to avoid overflow.
    fn unsigned_abs(self) -> U256 {
        if self.is_negative() && self == Self::MIN {
            // Add 1 before flipping the sign to avoid overflow
            let abs = (self + 1.into()).flip_sign();
            return abs.to_u256().unwrap() + U256::from(1);
        }
        self.abs().to_u256().unwrap()
    }

    conversion_fns!(u128, U256);
}
