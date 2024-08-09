// TODO: These macros can be improved in two important ways:
//
// 1. Support non-decimal number formats. It would be convenient to support
//    hexadecimal and binary numbers since Rust has similar problems with those
//    number types.
// 2. Support expressions. Ideally, we would execute any expressions at compile
//    time, so we could instantiate a fixed point number to represent 60 * 60 *
//    24 seconds (for example).

#[macro_export]
macro_rules! uint256 {
    ($number:expr) => {{
        let str = stringify!($number);
        $crate::u256_from_str(str).unwrap()
    }};
}

#[macro_export]
macro_rules! int256 {
    ($number:expr) => {{
        let str = stringify!($number);
        $crate::i256_from_str(str).unwrap()
    }};
}

/// Creates a `FixedPoint<T>` from a decimal number. Infers the type of `T` from
/// the context. If the context is ambiguous, use a typed alternative such as
/// [`fixed_u256!`] or [`fixed_i256!`].
#[macro_export]
macro_rules! fixed {
    ($number:expr) => {{
        let str = stringify!($number);
        $crate::FixedPoint::from_dec_str(str).unwrap()
    }};
}

/// Creates a `FixedPoint<U256>` from a decimal number.
#[macro_export]
macro_rules! fixed_u256 {
    ($number:expr) => {{
        let str = stringify!($number);
        $crate::FixedPoint::<ethers::types::U256>::from_dec_str(str).unwrap()
    }};
}

/// Creates a `FixedPoint<I256>` from a decimal number.
#[macro_export]
macro_rules! fixed_i256 {
    ($number:expr) => {{
        let str = stringify!($number);
        $crate::FixedPoint::<ethers::types::I256>::from_dec_str(str).unwrap()
    }};
}

/// Creates a `FixedPoint<u128>` from a decimal number.
#[macro_export]
macro_rules! fixed_u128 {
    ($number:expr) => {{
        let str = stringify!($number);
        $crate::FixedPoint::<u128>::from_dec_str(str).unwrap()
    }};
}

/// Creates a `FixedPoint<i128>` from a decimal number.
#[macro_export]
macro_rules! fixed_i128 {
    ($number:expr) => {{
        let str = stringify!($number);
        $crate::FixedPoint::<i128>::from_dec_str(str).unwrap()
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
        assert_eq!(fixed!(1), FixedPoint::<i128>::from(1));
        assert_eq!(fixed!(1_000), FixedPoint::<i128>::from(1_000));
        assert_eq!(fixed!(-1_000), FixedPoint::<i128>::from(-1_000));
        assert_eq!(
            fixed!(5_500_000_000),
            FixedPoint::<i128>::from(5_500_000_000_i128)
        );

        // scientific notation
        assert_eq!(fixed!(1e0), FixedPoint::<i128>::from(1));
        assert_eq!(fixed!(1e3), FixedPoint::<i128>::from(1_000));
        assert_eq!(fixed!(1e18), FixedPoint::<i128>::from(10_i128.pow(18)));
        assert_eq!(fixed!(-1e18), FixedPoint::<i128>::from(-10_i128.pow(18)));
        assert_eq!(
            fixed!(50_000e18),
            FixedPoint::<i128>::from(50_000 * 10_i128.pow(18))
        );

        // decimal notation
        assert_eq!(fixed!(1.0e1), FixedPoint::<i128>::from(10));
        assert_eq!(
            fixed!(1.1e18),
            FixedPoint::<i128>::from(11_i128 * 10_i128.pow(17)),
        );
        assert_eq!(
            fixed!(333_333.555_555e18),
            FixedPoint::<i128>::from(333_333_555_555_i128 * 10_i128.pow(12))
        );

        assert_eq!(
            fixed!(-333_333.555_555e18),
            -FixedPoint::<i128>::from(333_333_555_555_i128 * 10_i128.pow(12))
        );
    }
}
