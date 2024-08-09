use std::fmt::{self, Debug};

use ethers::types::{I256, U256};
use eyre::{bail, eyre, Result};

use crate::{sign::FixedPointSign, utils::u256_from_str, value::FixedPointValue};

// TODO: Remove decimals for now.

/// A generic fixed point wrapper around the `U256` type from ethers-rs.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct FixedPoint<T: FixedPointValue> {
    value: T,
    decimals: u8,
}

impl<T: FixedPointValue> FixedPoint<T> {
    pub const MIN: Self = Self {
        value: T::MIN,
        decimals: T::MAX_DECIMALS,
    };

    pub const MAX: Self = Self {
        value: T::MAX,
        decimals: T::MAX_DECIMALS,
    };

    // Constructors //

    pub fn from<V: Into<T>>(value: V) -> Self {
        Self {
            value: value.into(),
            // TODO: Add support for variable decimals.
            decimals: T::MAX_DECIMALS,
        }
    }

    pub fn try_from<V: TryInto<T> + Debug>(value: V) -> Result<Self> {
        // Convert the value to a Debug string before moving it incase the
        // conversion fails.
        let value_debug = format!("{:?}", value);
        let value = value.try_into().map_err(|_| {
            eyre!("Failed to convert value to underlying FixedPointValue: {value_debug}")
        })?;

        if (value > T::MAX) || (value < T::MIN) {
            bail!(
                r#"Value {value:?} is out of FixedPoint range:
    min: {min:?}
    max: {max:?}"#,
                min = T::MIN,
                max = T::MAX,
            );
        }

        Ok(Self::from(value))
    }

    pub fn from_dec_str(s: &str) -> Result<Self> {
        let raw = if s.starts_with('-') {
            let uint = u256_from_str(&s[1..])?;
            T::from_u256(uint)?.flip_sign()
        } else {
            let uint = u256_from_str(s)?;
            T::from_u256(uint)?
        };
        Ok(Self::from(raw))
    }

    pub fn saturate_sign(sign: FixedPointSign) -> Self {
        match sign {
            FixedPointSign::Positive => Self::MAX,
            FixedPointSign::Negative => Self::MIN,
        }
    }

    pub fn zero() -> Self {
        Self::from(0)
    }

    /// One with the same scale as this fixed point number, i.e., `1.0`.
    ///
    /// # Panics
    ///
    /// Panics if `10` to the power of `self.decimals()` overflows `T`.
    pub fn one(&self) -> Self {
        Self::try_from(10_u128.pow(self.decimals().into())).unwrap()
    }

    // Getters //

    // TODO: Change this
    pub fn raw(&self) -> T {
        self.value
    }

    pub fn decimals(&self) -> u8 {
        self.decimals
    }

    pub fn sign(&self) -> FixedPointSign {
        match self.is_negative() {
            true => FixedPointSign::Negative,
            false => FixedPointSign::Positive,
        }
    }

    // Predicates //

    pub fn is_negative(&self) -> bool {
        self.raw().is_negative()
    }

    pub fn is_positive(&self) -> bool {
        !self.is_negative()
    }

    pub fn is_zero(&self) -> bool {
        self.raw().is_zero()
    }

    // Conversion to unsigned & signed ethers types //

    pub fn to_u256(&self) -> Result<U256> {
        if self.is_negative() {
            bail!("Cannot convert negative FixedPoint {self} to U256.");
        }
        self.raw().to_u256()
    }

    pub fn to_i256(&self) -> Result<I256> {
        let abs = self.unsigned_abs().raw();
        let abs_max = FixedPoint::<I256>::saturate_sign(self.sign())
            .raw()
            .unsigned_abs();
        if abs > abs_max {
            bail!("FixedPoint {self} is too large to convert to I256.");
        }
        I256::checked_from_sign_and_abs(self.sign().into(), abs)
            .ok_or(eyre!("Failed to convert FixedPoint {self} to I256."))
    }

    // Conversion to unsigned and signed std types //

    pub fn to_u128(&self) -> Result<u128> {
        if self.is_negative() {
            bail!("Cannot convert negative FixedPoint {self} to u128.");
        }
        self.raw().to_u128()
    }

    pub fn to_i128(&self) -> Result<i128> {
        let i256 = self.to_i256()?;
        i128::try_from(i256)
            .or_else(|_| bail!("FixedPoint {self} is too large to convert to i128."))
    }

    // Formatting //

    pub fn to_scaled_string(&self) -> String {
        let decimals = (self.decimals()) as usize;
        let zero = T::from(0);
        let ten = T::from(10);
        let char_code_zero = T::from(48);
        let mut value = self.raw().abs();
        let mut digits = 0;
        let mut result = vec![];
        while value > zero {
            if digits == decimals && decimals > 0 {
                result.push('.');
            }

            let char_code = (value % ten + char_code_zero).to_u128().unwrap();
            result.push(char_code as u8 as char);
            value /= ten;
            digits += 1;
        }

        // Add leading zeros.
        if digits < decimals {
            result.resize(result.len() + decimals - digits, '0');
            digits += decimals - digits;
        }

        // Add the decimal point and leading zero.
        if digits == decimals {
            if decimals > 0 {
                result.push('.');
            }
            result.push('0');
        }

        format!("{}{}", self.sign(), result.iter().rev().collect::<String>())
    }
}

impl<T: FixedPointValue> Default for FixedPoint<T> {
    fn default() -> Self {
        Self::from(T::default())
    }
}

impl<T: FixedPointValue> fmt::Debug for FixedPoint<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FixedPoint({})", self.to_scaled_string())
    }
}

impl<T: FixedPointValue> fmt::Display for FixedPoint<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_scaled_string())
    }
}

// Conversions //

// Raw values can be converted to FixedPoint instances.
impl<T: FixedPointValue> From<T> for FixedPoint<T> {
    fn from(value: T) -> Self {
        Self::from(value)
    }
}

/// Implements conversions to and from `FixedPoint<T>` for a list of types that
/// can be converted to and from `T`.
///
/// "nested raw" types are types that can be converted to a `FixedPointValue`,
/// the "raw" type of a `FixedPoint` instance.
macro_rules! impl_from_nested_raw {
    ($($t:ty),*) => {
        $(
            impl<T: FixedPointValue + From<$t>> From<$t> for FixedPoint<T> {
                fn from(u: $t) -> Self {
                    Self::from(u)
                }
            }

            impl<T: FixedPointValue + Into<$t>> From<FixedPoint<T>> for $t {
                fn from(f: FixedPoint<T>) -> Self {
                    f.raw().into()
                }
            }
        )*
    };
    ($($tt:tt)*) => {};
}

// Any `FixedPointValue` that can be created from these types can be converted
// to a `FixedPoint` instance.
impl_from_nested_raw!(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, [u8; 32], bool);

/// Takes a mapping of raw types to `FixedPoint` methods and implements try
/// conversions to each raw type using the corresponding method.
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

#[cfg(test)]
mod tests {
    use std::u128;

    use crate::fixed_u128;

    #[test]
    fn test_fixed_point_fmt() {
        // fmt::Debug
        assert_eq!(
            format!("{:?}", fixed_u128!(1)),
            "FixedPoint(0.000000000000000001)"
        );
        assert_eq!(
            format!("{:?}", fixed_u128!(1.23456e18)),
            "FixedPoint(1.234560000000000000)"
        );
        assert_eq!(
            format!("{:?}", fixed_u128!(50_000.234_56e18)),
            "FixedPoint(50000.234560000000000000)"
        );

        // fmt::Display
        assert_eq!(format!("{}", fixed_u128!(1)), "0.000000000000000001");
        assert_eq!(
            format!("{}", fixed_u128!(1.23456e18)),
            "1.234560000000000000"
        );
        assert_eq!(
            format!("{}", fixed_u128!(50_000.234_56e18)),
            "50000.234560000000000000"
        );
    }
}
