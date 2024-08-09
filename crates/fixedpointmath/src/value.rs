use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, Sub, SubAssign},
};

use ethers::types::{I256, U256};
use eyre::{eyre, Result};
use paste::paste;

use crate::FixedPoint;

/// Implements the `from_<type>` and `to_<type>` conversion functions for a list
/// of types.
macro_rules! conversion_fns {
    ($($type_name:ident),*) => {
        $(
        paste! {
            fn [<from_ $type_name:snake>](value: $type_name) -> Result<Self> {
                Self::try_from(value).map_err(|_| {
                    eyre!(
        r#"Failed to convert {type} to underlying FixedPointValue:
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
    Sized
    + Copy
    + Debug
    + Default

    // Required comparisons
    + PartialEq
    + Eq
    + PartialOrd
    + Ord

    // Required math
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Rem<Output = Self>
    
    // Required std conversions
    + TryFrom<u128>
    + TryInto<u128>
    + From<u64>

    // Required ethers conversions
    + TryFrom<U256>
    + TryInto<U256>
{
    /// The minimum value that can be represented by the type. Can be negative.
    const MIN: Self;

    /// The maximum value that can be represented by the type. Must be `>= 0`.
    const MAX: Self;

    const MAX_DECIMALS: u8 = 18;

    fn is_zero(self) -> bool {
        self == 0.into()
    }

    fn is_negative(self) -> bool {
        self < 0.into()
    }

    fn is_positive(self) -> bool {
        !self.is_negative()
    }

    /// Whether the value supports negation.
    fn is_signed() -> bool {
        Self::MIN.is_negative()
    }

    /// Flips the sign of the value.
    ///
    /// # Panics
    ///
    /// If the value doesn't support negation or the value overflows, i.e., the
    /// value is equal to `MIN`.
    fn flip_sign(self) -> Self {
        if Self::is_signed() {
            return Self::from(0) - self;
        } else {
            panic!("Cannot flip sign of unsigned value: {self:?}");
        }
    }

    /// Flips the sign of the value if the condition is true.
    ///
    /// # Panics
    ///
    /// If the value doesn't support negation.
    fn flip_sign_if(self, condition: bool) -> Self {
        if condition {
            return self.flip_sign();
        }
        self
    }

    /// Computes the absolute value of self.
    ///
    /// # Panics
    ///
    /// If the absolute value of self overflows `T`, e.g., if self is the
    /// minimum value of a signed integer.
    fn abs(self) -> Self {
        if self.is_negative() {
             self.flip_sign()
        } else {
            self
        }
    }

    /// Computes the absolute value of self as a `U256` to avoid overflow.
    fn unsigned_abs(self) -> U256 {
        if self.is_negative() {
            // Add 1 before flipping the sign to avoid overflow
            let abs = (self + 1.into()).flip_sign();
            abs.to_u256().unwrap() + U256::from(1)
        } else {
            self.to_u256().unwrap()
        }
    }

    conversion_fns!(U256, u128);
    
    /// Converts the value to a `FixedPoint` instance.
    ///
    /// # Example
    ///
    /// ```rs
    /// let U256 = U256::from(100).as_fixed(); // -> FixedPoint<U256>
    /// let I256 = I256::from(100).as_fixed(); // -> FixedPoint<I256>
    /// let u128 = 100u128.as_fixed();         // -> FixedPoint<u128>
    /// let i128 = 100i128.as_fixed();         // -> FixedPoint<i128>
    /// ```
    fn as_fixed(self) -> FixedPoint<Self> {
        FixedPoint::from(self)
    }
}

impl FixedPointValue for U256 {
    const MIN: Self = U256([0, 0, 0, 0]);
    const MAX: Self = U256::MAX;
}

impl FixedPointValue for I256 {
    const MIN: Self = I256::MIN;
    const MAX: Self = I256::MAX;
}

impl FixedPointValue for u128 {
    const MIN: Self = u128::MIN;
    const MAX: Self = u128::MAX;
}

impl FixedPointValue for i128 {
    const MIN: Self = i128::MIN;
    const MAX: Self = i128::MAX;
}