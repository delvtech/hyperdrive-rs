use ethers::types::{I256, U256};
use eyre::{eyre, Result};

/// Parse a string into a U256 with support for scientific and decimal notation.
///
/// ## Example
///
/// ```
/// use ethers::types::U256;
/// use fixedpointmath::u256_from_str;
///
/// let u = u256_from_str("1.1e18").unwrap();
/// assert_eq!(u, U256::from(11) * U256::from(10).pow(U256::from(17)));
/// ```
pub fn u256_from_str(s: &str) -> Result<U256> {
    // Parse a string into a mantissa and an exponent. The U256 arithmetic
    // will overflow if the mantissa or the exponent are too large.
    let mut found_dot = false;
    let mut found_e = false;
    let mut mantissa = ethers::types::U256::zero();
    let mut exponent = ethers::types::U256::zero();
    let mut decimals = 0;

    for digit in s.chars() {
        if digit.is_ascii_digit() {
            let d = digit.to_digit(10).unwrap();
            if !found_e {
                mantissa = mantissa * 10 + d;
            } else {
                exponent = exponent * 10 + d;
            }
            if found_dot && !found_e {
                decimals += 1;
            }
        } else if digit == 'e' && !found_e {
            found_e = true;
        } else if digit == '.' && !found_dot {
            found_dot = true;
        } else if digit != '_' {
            return Err(eyre!("Unexpected character: {digit}"));
        }
    }

    // Combine the mantissa and the exponent into a single U256. This will
    // overflow if the exponent is too large. We also need to make sure that
    // the final result is an integer.
    let decimals = ethers::types::U256::from(decimals);
    if exponent < decimals {
        return Err(eyre!("Exponent is too small: {exponent}"));
    }

    Ok(mantissa * ethers::types::U256::from(10).pow(exponent - decimals))
}

/// Parse a string into an I256 with support for scientific and decimal notation.
///
/// ## Example
///
/// ```
/// use ethers::types::I256;
/// use fixedpointmath::i256_from_str;
///
/// let i = i256_from_str("-1.1e18").unwrap();
/// assert_eq!(i, -I256::from(11) * I256::from(10).pow(17));
/// ```
pub fn i256_from_str(s: &str) -> Result<I256> {
    // Parse a string into a mantissa and an exponent. The U256 arithmetic
    // will overflow if the mantissa or the exponent are too large.
    let mut sign = ethers::types::I256::one();
    let mut found_dot = false;
    let mut found_e = false;
    let mut mantissa = ethers::types::I256::zero();
    let mut exponent = 0;
    let mut decimals = 0;

    for digit in s.chars() {
        if digit.is_ascii_digit() {
            let d = digit.to_digit(10).unwrap();
            if !found_e {
                mantissa = mantissa * 10 + d;
            } else {
                exponent = exponent * 10 + d;
            }
            if found_dot && !found_e {
                decimals += 1;
            }
        } else if digit == '-' {
            sign = -ethers::types::I256::one();
        } else if digit == 'e' && !found_e {
            found_e = true;
        } else if digit == '.' && !found_dot {
            found_dot = true;
        } else if digit != '_' {
            return Err(eyre!("Unexpected character: {digit}"));
        }
    }

    // Combine the mantissa and the exponent I256. This will
    // overflow if the exponent is too large. We also need to make sure that
    // the final result is an integer.
    if exponent < decimals {
        return Err(eyre!("Exponent is too small: {exponent}"));
    }

    Ok(sign * mantissa * ethers::types::I256::from(10).pow(exponent - decimals))
}
