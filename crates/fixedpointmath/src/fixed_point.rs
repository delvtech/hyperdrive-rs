use std::fmt::{self, Debug};

use ethers::types::{I256, U256};
use eyre::{bail, eyre, Result};

use crate::{sign::FixedPointSign, utils::u256_from_str, value::FixedPointValue};

/// A generic fixed point type built on ethers-rs.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct FixedPoint<T: FixedPointValue> {
    raw: T,
    decimals: u8,
}

impl<T: FixedPointValue> FixedPoint<T> {
    pub const MIN: Self = Self {
        raw: T::MIN,
        decimals: T::MAX_DECIMALS,
    };

    pub const MAX: Self = Self {
        raw: T::MAX,
        decimals: T::MAX_DECIMALS,
    };

    // Constructors //

    pub fn new<V: Into<T>>(value: V) -> Self {
        Self {
            raw: value.into(),
            // TODO: Add support for variable decimals.
            decimals: T::MAX_DECIMALS,
        }
    }

    pub fn try_from<V: TryInto<T> + Debug>(value: V) -> Result<Self> {
        // Convert the value to a Debug string before moving it incase the
        // conversion fails.
        let value_debug = format!("{:?}", value);
        let value = value.try_into().map_err(|_| {
            eyre!(
                r#"Failed to convert value to underlying FixedPointValue type:
    value: {value_debug}
    Underlying range: {min:?} to {max:?}
"#,
                min = Self::MIN,
                max = Self::MAX,
            )
        })?;

        Ok(Self::new(value))
    }

    pub fn from_sign_and_abs(sign: FixedPointSign, abs: U256) -> Result<Self> {
        Ok(match sign {
            FixedPointSign::Positive => Self::new(T::from_u256(abs)?),
            FixedPointSign::Negative => {
                if abs == T::MIN.unsigned_abs() {
                    // NOTE: The absolute MIN value of a two's-complement
                    // integer is 1 greater than its MAX. Attempting to create a
                    // positive `T` instance with this value then flipping the
                    // sign after the fact will overflow, so we just return the
                    // MIN value directly in this case.
                    Self::MIN
                } else {
                    let raw = T::from_u256(abs)?.flip_sign();
                    Self::new(raw)
                }
            }
        })
    }

    pub fn from_dec_str(s: &str) -> Result<Self> {
        if s.starts_with('-') {
            Self::from_sign_and_abs(FixedPointSign::Negative, u256_from_str(&s[1..])?)
        } else {
            Self::from_sign_and_abs(FixedPointSign::Positive, u256_from_str(s)?)
        }
    }

    pub fn saturate_sign(sign: FixedPointSign) -> Self {
        match sign {
            FixedPointSign::Positive => Self::MAX,
            FixedPointSign::Negative => Self::MIN,
        }
    }

    pub fn zero() -> Self {
        Self::new(0)
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

    /// Returns the underlying raw value of the fixed point number, e.g., `U256`
    /// for `FixedPoint<U256>`.
    pub fn raw(&self) -> T {
        self.raw
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

    // Conversion to other FixedPoint types //

    /// Creates a `FixedPoint` instance with the same value as this instance but
    /// with a different underlying `FixedPointValue` type.
    ///
    /// # Example
    ///
    /// ```rs
    /// let fp_i128 = fixed_i128!(1);
    ///
    /// let fp_u128 = fp_i128.change_type::<u128>()?;
    /// assert_eq!(fp_u128, fixed_u128!(1));
    ///
    /// let fp_u128: FixedPoint<u128> = fp_i128.change_type()?;
    /// assert_eq!(fp_u128, fixed_u128!(1));
    /// ```
    pub fn change_type<U: FixedPointValue + TryFrom<T>>(self) -> Result<FixedPoint<U>> {
        self.raw().try_to_fixed()
    }

    // Conversion to unsigned & signed ethers types //

    pub fn to_u256(self) -> Result<U256> {
        if self.is_negative() {
            bail!("Cannot convert negative FixedPoint {self} to U256.");
        }
        self.raw().to_u256()
    }

    pub fn to_i256(self) -> Result<I256> {
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

    pub fn to_u128(self) -> Result<u128> {
        if self.is_negative() {
            bail!("Cannot convert negative FixedPoint {self} to u128.");
        }
        self.raw().to_u128()
    }

    pub fn to_i128(self) -> Result<i128> {
        let i256 = self.to_i256()?;
        i128::try_from(i256)
            .or_else(|_| bail!("FixedPoint {self} is too large to convert to i128."))
    }

    // Formatting //

    pub fn to_scaled_string(&self) -> String {
        let decimals = (self.decimals()) as usize;
        let zero = U256::from(0);
        let ten = U256::from(10);
        let char_code_zero = U256::from(48);
        let mut value = self.raw().unsigned_abs();
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

// Trait implementations //

impl<T: FixedPointValue> Default for FixedPoint<T> {
    fn default() -> Self {
        Self::new(T::default())
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

// Basic raw to FixedPoint conversion.
impl<T: FixedPointValue> From<T> for FixedPoint<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

/// A `FixedPointValue` that can convert to `FixedPoint<Self>` via `.fixed()`.
pub trait Fixed: FixedPointValue {
    /// Converts the value to a `FixedPoint<Self>` instance.
    ///
    /// # Example
    ///
    /// ```rs
    /// let a = U256::from(100).fixed(); // -> FixedPoint<U256>
    /// let b = I256::from(100).fixed(); // -> FixedPoint<I256>
    /// let c = 100_u128.fixed();        // -> FixedPoint<u128>
    /// let d = 100_i128.fixed();        // -> FixedPoint<i128>
    /// ```
    fn fixed(self) -> FixedPoint<Self> {
        FixedPoint::new(self)
    }
}

// Add `.fixed()` to all types that implement `FixedPointValue`.
impl<T: FixedPointValue> Fixed for T {}

/// A type that can convert to `FixedPoint<T>` via `.to_fixed()`, and attempt
/// conversion to `FixedPoint<T>` via `.try_to_fixed()`.
pub trait ToFixed: Sized + Debug {
    /// Converts the value to a `FixedPoint<T>` instance, first converting the
    /// value to the underlying `T` type if necessary.
    ///
    /// # Example
    ///
    /// ```rs
    /// let a: FixedPoint<U256> = 100_u128.to_fixed();
    /// let b: FixedPoint<I256> = 100_i128.to_fixed();
    /// ```
    ///
    /// Using ['turbofish'](https://turbo.fish/) syntax:
    ///
    /// ```rs
    /// let a = 100_u128.to_fixed::<U256>();
    /// let b = 100_i128.to_fixed::<I256>();
    /// ```
    fn to_fixed<T: FixedPointValue + From<Self>>(self) -> FixedPoint<T> {
        FixedPoint::<T>::new(self)
    }

    /// Attempts to convert the value to a `FixedPoint<T>` instance, first
    /// converting the value to the underlying `T` type if necessary.
    ///
    /// # Example
    ///
    /// ```rs
    /// let b: FixedPoint<I256> = U256::from(100).try_to_fixed().unwrap();
    /// ```
    ///
    /// Using ['turbofish'](https://turbo.fish/) syntax:
    ///
    /// ```rs
    /// let a = 100.try_to_fixed::<U256>();  // -> Ok(FixedPoint<U256>)
    /// let b = -100.try_to_fixed::<U256>(); // -> Err(...)
    /// ```
    fn try_to_fixed<T: FixedPointValue + TryFrom<Self>>(self) -> Result<FixedPoint<T>> {
        FixedPoint::<T>::try_from(self)
    }
}

// Add `.to_fixed()` & `.try_to_fixed()` to all sized types that implement
// `Debug`.
impl<T: Sized + Debug> ToFixed for T {}

/// Implements conversions to and from `FixedPoint<T>` for a list of types that
/// can be converted to and from `T`.
macro_rules! conversion_impls {
    ($($t:ty),*) => {
        $(
            impl<T: FixedPointValue + From<$t>> From<$t> for FixedPoint<T> {
                fn from(u: $t) -> Self {
                    Self::new(u)
                }
            }

            impl<T: FixedPointValue + TryInto<$t>> TryFrom<FixedPoint<T>> for $t {
                type Error = eyre::ErrReport;

                fn try_from(f: FixedPoint<T>) -> eyre::Result<Self> {
                    f.raw().try_into().map_err(|_| {
                        eyre!(
                            "Failed to convert underlying FixedPointValue to {type}: {f:?}",
                            type = stringify!($t)
                        )
                    })
                }
            }
        )*
    };
    ($($tt:tt)*) => {};
}

// Direct conversions between primitive types and FixedPoint for any
// `FixedPointValue` that can be converted to and from the primitive type.
conversion_impls!(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, [u8; 32], bool);

#[cfg(test)]
mod tests {
    use crate::fixed_i128;

    #[test]
    fn test_change_type_failure() {
        let fixed = fixed_i128!(-1);
        let fixed_u128 = fixed.change_type::<u128>();
        assert!(fixed_u128.is_err());
    }

    #[test]
    fn test_fmt() {
        // fmt::Debug
        assert_eq!(
            format!("{:?}", fixed_i128!(1)),
            "FixedPoint(0.000000000000000001)"
        );
        assert_eq!(
            format!("{:?}", fixed_i128!(1.23456e18)),
            "FixedPoint(1.234560000000000000)"
        );
        assert_eq!(
            format!("{:?}", fixed_i128!(50_000.234_56e18)),
            "FixedPoint(50000.234560000000000000)"
        );
        assert_eq!(
            format!("{:?}", fixed_i128!(-50_000.234_56e18)),
            "FixedPoint(-50000.234560000000000000)"
        );

        // fmt::Display
        assert_eq!(format!("{}", fixed_i128!(1)), "0.000000000000000001");
        assert_eq!(
            format!("{}", fixed_i128!(1.23456e18)),
            "1.234560000000000000"
        );
        assert_eq!(
            format!("{}", fixed_i128!(50_000.234_56e18)),
            "50000.234560000000000000"
        );
        assert_eq!(
            format!("{}", fixed_i128!(-50_000.234_56e18)),
            "-50000.234560000000000000"
        );
    }
}
