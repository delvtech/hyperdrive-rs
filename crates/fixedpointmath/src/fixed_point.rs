use std::{
    fmt::{self, Debug},
    ops::Div,
};

use ethers::types::{I256, U256, U512};
use eyre::{bail, eyre, Result};

use crate::{
    sign::FixedPointSign,
    utils::{exp, ln, u256_from_str},
    value::FixedPointValue,
};

/// A fixed point wrapper around the `U256` type from ethers-rs.
///
/// This fixed point type is a heavily based on Solidity's FixedPointMath
/// library with a few changes:
/// - The outward type of the underlying value is generic, allowing the library
///   to be used with any type that implements `FixedPointValue`, including
///   signed integers, and ensuring that the instance is bounded by the generic
///   type's limits.
/// - Support for overflowing intermediate operations in `mul_div_down` and
///  `mul_div_up` via `U512`.
///
/// Each of the functions is fuzz tested against the Solidity implementation to
/// ensure that the behavior is identical given values bounded by the Solidity
/// implementation's limits.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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

    pub const MAX_DECIMALS: u8 = T::MAX_DECIMALS;

    // Constructors //

    // The primary constructor for `FixedPoint`. All instances should be created
    // using this method to ensure consistent behavior.
    pub fn new<V: TryInto<T> + Debug>(value: V) -> Result<Self> {
        // Convert the value to a Debug string before moving it incase the
        // conversion fails.
        let value_debug = format!("{:?}", value);
        let value = value
            .try_into()
            .map_err(|_| eyre!("Failed to convert value to FixedPoint: {value_debug}"))?;

        if (value > T::MAX) || (value < T::MIN) {
            bail!(
                r#"Value {value:?} is out of FixedPoint range:
    min: {min:?}
    max: {max:?}"#,
                min = T::MIN,
                max = T::MAX,
            );
        }

        Ok(Self {
            value,
            // TODO: Add support for variable decimals.
            decimals: Self::MAX_DECIMALS,
        })
    }

    pub fn zero() -> Self {
        Self::new(0).unwrap()
    }

    pub fn from_dec_str(s: &str) -> Result<Self> {
        if s.starts_with('-') {
            let uint = u256_from_str(&s[1..])?;
            let raw = T::from_u256(uint)?.flip_sign();
            return Self::new(raw);
        }
        let uint = u256_from_str(s)?;
        let raw = T::from_u256(uint)?;
        Self::new(raw)
    }

    pub fn saturate_sign(sign: FixedPointSign) -> Self {
        match sign {
            FixedPointSign::Positive => Self::MAX,
            FixedPointSign::Negative => Self::MIN,
        }
    }

    // Getters //

    pub fn raw(&self) -> T {
        self.value
    }

    pub fn decimals(&self) -> u8 {
        self.decimals
    }

    pub fn sign(&self) -> FixedPointSign {
        if self.is_negative() {
            return FixedPointSign::Negative;
        }
        FixedPointSign::Positive
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

    // Math //

    /// One with the same scale as this fixed point number, i.e., `1.0`.
    ///
    /// # Panics
    ///
    /// Panics if `10` to the power of `self.decimals()` overflows `T`.
    pub fn one(&self) -> Self {
        let uint = U256::from(10).pow(self.decimals().into());
        Self::new(T::from_u256(uint).unwrap()).unwrap()
    }

    /// Computes the absolute value of self.
    ///
    /// # Panics
    ///
    /// If the absolute value of self overflows `T`, e.g., if self is the
    /// minimum value of a signed integer.
    pub fn abs(&self) -> Self {
        Self::new(self.raw().abs()).unwrap()
    }

    /// Computes the absolute value of self as a `U256` to avoid overflow.
    pub fn unsigned_abs(&self) -> FixedPoint<U256> {
        FixedPoint::new(self.raw().unsigned_abs()).unwrap()
    }

    pub fn abs_diff(&self, other: Self) -> FixedPoint<U256> {
        if self.sign() != other.sign() {
            return FixedPoint::new(self.unsigned_abs().raw() + other.unsigned_abs().raw())
                .unwrap();
        }
        let diff = if self > &other {
            self.raw() - other.raw()
        } else {
            other.raw() - self.raw()
        };
        FixedPoint::new(diff.to_u256().unwrap()).unwrap()
    }

    pub fn mul_div_down(&self, other: Self, divisor: Self) -> Result<Self> {
        if divisor.is_zero() {
            bail!("Cannot divide by zero.");
        }
        let u256 = U256::try_from(
            self.raw()
                .abs()
                .to_u256()?
                .full_mul(other.raw().abs().to_u256()?.into())
                .div(divisor.raw().abs().to_u256()?),
        )
        .or_else(|_| bail!("FixedPoint operation overflowed: {self} * {other} / {divisor}"))?;
        let sign = self.sign().flip_if(other.sign() != divisor.sign());
        let raw = T::from_u256(u256)?.flip_sign_if(sign.is_negative());
        Self::new(raw)
    }

    pub fn mul_div_up(&self, other: Self, divisor: Self) -> Result<Self> {
        if divisor.is_zero() {
            bail!("Cannot divide by zero.");
        }
        let (u512, remainder) = self
            .raw()
            .abs()
            .to_u256()?
            .full_mul(other.raw().abs().to_u256()?.into())
            .div_mod(divisor.raw().abs().to_u256()?.into());
        let rounded_u256 = U256::try_from(u512 + (remainder.gt(&U512::zero()) as u128))
            .or_else(|_| bail!("FixedPoint operation overflowed: {self} * {other} / {divisor}"))?;
        let sign = self.sign().flip_if(other.sign() != divisor.sign());
        let raw = T::from_u256(rounded_u256)?.flip_sign_if(sign.is_negative());
        Self::new(raw)
    }

    pub fn mul_down(&self, other: Self) -> Result<Self> {
        self.mul_div_down(other, self.one())
    }

    pub fn mul_up(&self, other: Self) -> Result<Self> {
        self.mul_div_up(other, self.one())
    }

    pub fn div_down(self, other: Self) -> Result<Self> {
        self.mul_div_down(self.one(), other)
    }

    pub fn div_up(self, other: Self) -> Result<Self> {
        self.mul_div_up(self.one(), other)
    }

    pub fn pow(self, y: Self) -> Result<Self> {
        let one = self.one();

        // If the exponent is negative, return 1 / x^abs(y).
        if y.is_negative() {
            let abs_result = self.pow(y.abs())?;

            // FIXME: What's the correct return value here?
            if abs_result.is_zero() {
                return Ok(Self::zero());
            }

            return Ok(one.div_down(abs_result)?);
        }

        // If the exponent is 0, return 1.
        if y.is_zero() {
            return Ok(one);
        }

        // If the base is 0, return 0.
        if self.is_zero() {
            return Ok(Self::zero());
        }

        // Using properties of logarithms we calculate x^y: -> ln(x^y) = y *
        // ln(x) -> e^(y * ln(x)) = x^y
        let y_int256 = y.to_i256()?;

        // Compute y*ln(x) Any overflow for x will be caught in _ln() in the
        // initial bounds check
        let lnx = ln(self.to_i256()?)?;
        let mut ylnx = y_int256.wrapping_mul(lnx);
        ylnx = ylnx.wrapping_div(one.to_i256()?);

        // Calculate exp(y * ln(x)) to get x^y
        let (sign, abs) = exp(ylnx)?.into_sign_and_abs();
        let raw = T::from_u256(abs)?.flip_sign_if(sign.is_negative());
        Self::new(raw)
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
        Self::new(T::default()).unwrap()
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
