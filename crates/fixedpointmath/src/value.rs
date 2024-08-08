use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign},
};

use ethers::{
    core::k256::sha2::digest::typenum::PartialDiv,
    types::{I256, U256},
};
use eyre::{eyre, Result};
use paste::paste;

/// Implements the `from_<type>` and `to_<type>` conversion functions for the
/// given type.
macro_rules! try_from_into {
    ($($type_name:ident),*) => {
        $(
        paste! {
            fn [<from_ $type_name:snake>](value: $type_name) -> Result<Self> {
                Self::try_from(value).map_err(|_| {
                    eyre!(
        r#"Failed to convert {type} to underlying FixedPointValue:
    {type} value: {value}
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
                        "Failed to convert underlying FixedPointValue to {}: {self:?}",
                        stringify!($type_name)
                   )
                })
            }
        })*
    };
}

pub trait FixedPointValue:
    Copy
    + Default
    + Debug
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Rem<Output = Self>
    + TryFrom<U256>
    + TryFrom<u128>
    + TryFrom<i128>
    + TryInto<U256>
    + TryInto<u128>
    + TryInto<i128>
    + From<u64>
{
    const MAX: Self;
    const MIN: Self;
    const MAX_DECIMALS: u8;

    try_from_into!(U256, u128, i128);

    fn abs(self) -> Self;
    fn is_negative(self) -> bool;
    fn flip_sign(self) -> Self;

    fn flip_sign_if(self, condition: bool) -> Self {
        if condition {
            return self.flip_sign();
        }
        self
    }
}

// TODO: On hold until varying decimal precision is implemented
// impl FixedPointValue for U512 {
//     const MAX: Self = U512::MAX;
//     const MIN: Self = U512([0, 0, 0, 0, 0, 0, 0, 0]);
//     const MAX_DECIMALS: u8 = 36;

//     fn abs(self) -> Self {
//         self
//     }

//     fn is_negative(self) -> bool {
//         false
//     }

//     fn flip_sign(self) -> Self {
//         self
//     }
// }

impl FixedPointValue for U256 {
    const MAX: Self = U256::MAX;
    const MIN: Self = U256::MAX;
    const MAX_DECIMALS: u8 = 18;

    fn abs(self) -> Self {
        self
    }

    fn is_negative(self) -> bool {
        false
    }

    fn flip_sign(self) -> Self {
        self
    }
}

impl FixedPointValue for I256 {
    const MAX: Self = I256::MAX;
    const MIN: Self = I256::MIN;
    const MAX_DECIMALS: u8 = 18;

    fn abs(self) -> Self {
        I256::abs(self)
    }

    fn is_negative(self) -> bool {
        self.is_negative()
    }

    fn flip_sign(self) -> Self {
        -self
    }
}

impl FixedPointValue for u128 {
    const MAX: Self = u128::MAX;
    const MIN: Self = u128::MIN;
    const MAX_DECIMALS: u8 = 18;

    fn abs(self) -> Self {
        self
    }

    fn is_negative(self) -> bool {
        false
    }

    fn flip_sign(self) -> Self {
        self
    }
}

impl FixedPointValue for i128 {
    const MAX: Self = i128::MAX;
    const MIN: Self = i128::MIN;
    const MAX_DECIMALS: u8 = 18;

    fn abs(self) -> Self {
        i128::abs(self)
    }

    fn is_negative(self) -> bool {
        self.is_negative()
    }

    fn flip_sign(self) -> Self {
        -self
    }
}
