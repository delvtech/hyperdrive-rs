// TODO: These macros can be improved in two important ways:
//
// 1. Support non-decimal number formats. It would be convenient to support
//    hexadecimal and binary numbers since Rust has similar problems with those
//    number types.
// 2. Support expressions. Ideally, we would execute any expressions at compile
//    time, so we could instantiate a fixed point number to represent
//    60 * 60 * 24 seconds (for example).

#[macro_export]
macro_rules! uint256 {
    ($number:expr) => {{
        let str = stringify!($number);

        // Parse a string into a mantissa and an exponent. The U256 arithmetic
        // will overflow if the mantissa or the exponent are too large.
        let mut found_dot = false;
        let mut found_e = false;
        let mut mantissa = ethers::types::U256::zero();
        let mut exponent = ethers::types::U256::zero();
        let mut decimals = 0;

        for digit in str.chars() {
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
                panic!("Unexpected character: {digit}");
            }
        }

        // Combine the mantissa and the exponent into a single U256. This will
        // overflow if the exponent is too large. We also need to make sure that
        // the final result is an integer.
        let decimals = ethers::types::U256::from(decimals);
        if exponent < decimals {
            panic!("Exponent is too small: {exponent}");
        }

        mantissa * ethers::types::U256::from(10).pow(exponent - decimals)
    }};
}

#[macro_export]
macro_rules! int256 {
    ($number:expr) => {{
        let str = stringify!($number);

        // Parse a string into a mantissa and an exponent. The U256 arithmetic
        // will overflow if the mantissa or the exponent are too large.
        let mut sign = ethers::types::I256::one();
        let mut found_dot = false;
        let mut found_e = false;
        let mut mantissa = ethers::types::I256::zero();
        let mut exponent = 0;
        let mut decimals = 0;

        for digit in str.chars() {
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
                panic!("Unexpected character: {digit}");
            }
        }

        // Combine the mantissa and the exponent I256. This will
        // overflow if the exponent is too large. We also need to make sure that
        // the final result is an integer.
        if exponent < decimals {
            panic!("Exponent is too small: {exponent}");
        }

        sign * mantissa * ethers::types::I256::from(10).pow(exponent - decimals)
    }};
}

#[macro_export]
macro_rules! fixed {
    ($number:expr) => {{
        $crate::FixedPoint::from($crate::uint256!($number))
    }};
}

#[cfg(test)]
mod tests {
    use ethers::types::{I256, U256};

    use crate::FixedPoint;

    #[test]
    fn test_int256() {
        // simple cases
        assert_eq!(int256!(1), I256::from(1));
        assert_eq!(int256!(1_000), I256::from(1_000));
        assert_eq!(int256!(-1_000), I256::from(-1_000));
        assert_eq!(
            int256!(-42139678854452767551),
            I256::from(-42139678854452767551_i128)
        );

        // scientific notation
        assert_eq!(int256!(1e0), I256::from(1));
        assert_eq!(int256!(1e3), I256::from(1_000));
        assert_eq!(int256!(-1e18), -I256::from(10).pow(18));
        assert_eq!(
            int256!(-50_000e18),
            -I256::from(50_000) * I256::from(10).pow(18)
        );

        // decimal notation
        assert_eq!(int256!(1.0e1), I256::from(10));
        assert_eq!(int256!(1.1e18), I256::from(11) * I256::from(10).pow(17));
        assert_eq!(
            int256!(-333_333.555_555e18),
            -I256::from(333_333_555_555_u128) * I256::from(10).pow(12)
        );
    }

    #[test]
    fn test_uint256() {
        // simple cases
        assert_eq!(uint256!(1), U256::from(1));
        assert_eq!(uint256!(1_000), U256::from(1_000));
        assert_eq!(uint256!(5_500_000_000), U256::from(5_500_000_000_u128));

        // scientific notation
        assert_eq!(uint256!(1e0), U256::from(1));
        assert_eq!(uint256!(1e3), U256::from(1_000));
        assert_eq!(
            uint256!(50_000e18),
            U256::from(50_000) * U256::from(10).pow(18.into())
        );

        // decimal notation
        assert_eq!(uint256!(1.0e1), U256::from(10));
        assert_eq!(
            uint256!(1.1e18),
            U256::from(11) * U256::from(10).pow(17.into())
        );
        assert_eq!(
            uint256!(333_333.555_555e18),
            U256::from(333_333_555_555_u128) * U256::from(10).pow(12.into())
        );
    }

    #[test]
    fn test_fixed() {
        // simple cases
        assert_eq!(fixed!(1), FixedPoint::from(1));
        assert_eq!(fixed!(1_000), FixedPoint::from(1_000));
        assert_eq!(fixed!(5_500_000_000), FixedPoint::from(5_500_000_000_u128));

        // scientific notation
        assert_eq!(fixed!(1e0), FixedPoint::from(1));
        assert_eq!(fixed!(1e3), FixedPoint::from(1_000));
        assert_eq!(
            fixed!(50_000e18),
            FixedPoint::from(U256::from(50_000) * U256::from(10).pow(18.into()))
        );

        // decimal notation
        assert_eq!(fixed!(1.0e1), FixedPoint::from(10));
        assert_eq!(
            fixed!(1.1e18),
            FixedPoint::from(U256::from(11) * U256::from(10).pow(17.into())),
        );
        assert_eq!(
            fixed!(333_333.555_555e18),
            FixedPoint::from(U256::from(333_333_555_555_u128) * U256::from(10).pow(12.into()))
        );
    }
}
