use std::ops::Shr;

use ethers::types::{I256, U256};
use eyre::{bail, Result};

use crate::{int256, uint256};

/// Parses a string into a U256 with support for scientific and decimal
/// notation.
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
    // Parse a string into a mantissa and an exponent. The U256 arithmetic will
    // overflow if the mantissa or the exponent are too large.
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
            bail!("Unexpected character in U256: {digit}");
        }
    }

    // Combine the mantissa and the exponent into a single U256. This will
    // overflow if the exponent is too large. We also need to make sure that the
    // final result is an integer.
    let decimals = ethers::types::U256::from(decimals);
    if exponent < decimals {
        bail!("Exponent {exponent} is too small for U256: {s}");
    }

    Ok(mantissa * ethers::types::U256::from(10).pow(exponent - decimals))
}

/// Parse a string into an I256 with support for scientific and decimal
/// notation.
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
    // Parse a string into a mantissa and an exponent. The U256 arithmetic will
    // overflow if the mantissa or the exponent are too large.
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
            bail!("Unexpected character in I256: {digit}");
        }
    }

    // Combine the mantissa and the exponent I256. This will overflow if the
    // exponent is too large. We also need to make sure that the final result is
    // an integer.
    if exponent < decimals {
        bail!("Exponent {exponent} is too small for I256: {s}");
    }

    Ok(sign * mantissa * ethers::types::I256::from(10).pow(exponent - decimals))
}

/// Math

pub fn exp(mut x: I256) -> Result<I256> {
    // When the result is < 0.5 we return zero. This happens when x <=
    // floor(log(0.5e18) * 1e18) ~ -42e18
    if x <= I256::from(-42139678854452767551_i128) {
        return Ok(I256::zero());
    }

    // When the result is > (2**255 - 1) / 1e18 we can not represent it as an
    // int. This happens when x >= floor(log((2**255 - 1) / 1e18) * 1e18) ~ 135.
    if x >= int256!(135305999368893231589) {
        bail!("invalid exponent {x}");
    }

    // x is now in the range (-42, 136) * 1e18. Convert to (-42, 136) * 2**96
    // for more intermediate precision and a binary basis. This base conversion
    // is a multiplication by 1e18 / 2**96 = 5**18 / 2**78.
    x = x.wrapping_shl(78) / int256!(5).pow(18);

    // Reduce range of x to (-½ ln 2, ½ ln 2) * 2**96 by factoring out powers of
    // two such that exp(x) = exp(x') * 2**k, where k is an integer. Solving
    // this gives k = round(x / log(2)) and x' = x - k * log(2).
    let k = ((x.wrapping_shl(96) / int256!(54916777467707473351141471128))
        .wrapping_add(int256!(2).pow(95)))
    .asr(96);
    x = x.wrapping_sub(k.wrapping_mul(54916777467707473351141471128_u128.into()));

    // k is in the range [-61, 195].

    // Evaluate using a (6, 7)-term rational approximation. p is made monic,
    // we'll multiply by a scale factor later.
    let mut y = x.wrapping_add(1346386616545796478920950773328_u128.into());
    y = y
        .wrapping_mul(x)
        .asr(96)
        .wrapping_add(57155421227552351082224309758442_u128.into());
    let mut p = y
        .wrapping_add(x)
        .wrapping_sub(94201549194550492254356042504812_u128.into());
    p = p
        .wrapping_mul(y)
        .asr(96)
        .wrapping_add(28719021644029726153956944680412240_u128.into());
    p = p
        .wrapping_mul(x)
        .wrapping_add(int256!(4385272521454847904659076985693276).wrapping_shl(96));

    // We leave p in 2**192 basis so we don't need to scale it back up for the
    // division.
    let mut q = x.wrapping_sub(2855989394907223263936484059900_u128.into());
    q = q
        .wrapping_mul(x)
        .asr(96)
        .wrapping_add(50020603652535783019961831881945_u128.into());
    q = q
        .wrapping_mul(x)
        .asr(96)
        .wrapping_sub(533845033583426703283633433725380_u128.into());
    q = q
        .wrapping_mul(x)
        .asr(96)
        .wrapping_add(3604857256930695427073651918091429_u128.into());
    q = q
        .wrapping_mul(x)
        .asr(96)
        .wrapping_sub(14423608567350463180887372962807573_u128.into());
    q = q
        .wrapping_mul(x)
        .asr(96)
        .wrapping_add(26449188498355588339934803723976023_u128.into());

    let mut r = p.wrapping_div(q);

    // r should be in the range (0.09, 0.25) * 2**96.

    // We now need to multiply r by:
    // * the scale factor s = ~6.031367120.
    // * the 2**k factor from the range reduction.
    // * the 1e18 / 2**96 factor for base conversion. We do this all at once,
    // with an intermediate result in 2**213 basis, so the final right shift is
    // always by a positive amount.
    r = I256::from_raw(
        (r.into_raw()
            .overflowing_mul(uint256!(3822833074963236453042738258902158003155416615667))
            .0)
            .shr(int256!(195).wrapping_sub(k).low_usize()),
    );

    Ok(r)
}

pub fn ln(mut x: I256) -> Result<I256> {
    if x <= I256::zero() {
        bail!("Cannot calculate ln of of negative number or zero.");
    }

    // We want to convert x from 10**18 fixed point to 2**96 fixed point. We do
    // this by multiplying by 2**96 / 10**18. But since ln(x * C) = ln(x) +
    // ln(C), we can simply do nothing here and add ln(2**96 / 10**18) at the
    // end.

    let mut r = I256::from((x > I256::from(0xffffffffffffffffffffffffffffffff_u128)) as u128)
        .wrapping_shl(7);
    r = r | I256::from((x.asr(r.as_usize()) > I256::from(0xffffffffffffffff_u128)) as u128)
        .wrapping_shl(6);
    r = r | I256::from((x.asr(r.as_usize()) > I256::from(0xffffffff_u128)) as u128).wrapping_shl(5);
    r = r | I256::from((x.asr(r.as_usize()) > I256::from(0xffff_u128)) as u128).wrapping_shl(4);
    r = r | I256::from((x.asr(r.as_usize()) > I256::from(0xff_u128)) as u128).wrapping_shl(3);
    r = r | I256::from((x.asr(r.as_usize()) > I256::from(0xf_u128)) as u128).wrapping_shl(2);
    r = r | I256::from((x.asr(r.as_usize()) > I256::from(0x3_u128)) as u128).wrapping_shl(1);
    r = r | I256::from((x.asr(r.as_usize()) > I256::from(0x1_u128)) as u128);

    // Reduce range of x to (1, 2) * 2**96 ln(2^k * x) = k * ln(2) + ln(x)
    let k = r.wrapping_sub(int256!(96));
    x = x.wrapping_shl(int256!(159).wrapping_sub(k).as_usize());
    x = I256::from_raw(x.into_raw().shr(159));

    // Evaluate using a (8, 8)-term rational approximation. p is made monic, we
    // will multiply by a scale factor later.
    let mut p = x.wrapping_add(int256!(3273285459638523848632254066296));
    p = ((p.wrapping_mul(x)).asr(96)).wrapping_add(int256!(24828157081833163892658089445524));
    p = ((p.wrapping_mul(x)).asr(96)).wrapping_add(int256!(43456485725739037958740375743393));
    p = ((p.wrapping_mul(x)).asr(96)).wrapping_sub(int256!(11111509109440967052023855526967));
    p = ((p.wrapping_mul(x)).asr(96)).wrapping_sub(int256!(45023709667254063763336534515857));
    p = ((p.wrapping_mul(x)).asr(96)).wrapping_sub(int256!(14706773417378608786704636184526));
    p = p
        .wrapping_mul(x)
        .wrapping_sub(int256!(795164235651350426258249787498).wrapping_shl(96));

    // We leave p in 2**192 basis so we don't need to scale it back up for the
    // division. q is monic by convention.
    let mut q = x.wrapping_add(int256!(5573035233440673466300451813936));
    q = (q.wrapping_mul(x).asr(96)).wrapping_add(int256!(71694874799317883764090561454958));
    q = q
        .wrapping_mul(x)
        .asr(96)
        .wrapping_add(int256!(283447036172924575727196451306956));
    q = q
        .wrapping_mul(x)
        .asr(96)
        .wrapping_add(int256!(401686690394027663651624208769553));
    q = q
        .wrapping_mul(x)
        .asr(96)
        .wrapping_add(int256!(204048457590392012362485061816622));
    q = q
        .wrapping_mul(x)
        .asr(96)
        .wrapping_add(int256!(31853899698501571402653359427138));
    q = q
        .wrapping_mul(x)
        .asr(96)
        .wrapping_add(int256!(909429971244387300277376558375));

    r = p.wrapping_div(q);

    // r is in the range (0, 0.125) * 2**96

    // Finalization, we need to:
    // * multiply by the scale factor s = 5.549…
    // * add ln(2**96 / 10**18)
    // * add k * ln(2)
    // * multiply by 10**18 / 2**96 = 5**18 >> 78

    // mul s * 5e18 * 2**96, base is now 5**18 * 2**192
    r = r.wrapping_mul(int256!(1677202110996718588342820967067443963516166));
    // add ln(2) * k * 5e18 * 2**192
    r = r.wrapping_add(
        int256!(16597577552685614221487285958193947469193820559219878177908093499208371)
            .wrapping_mul(k),
    );
    // add ln(2**96 / 10**18) * 5e18 * 2**192
    r = r.wrapping_add(int256!(
        600920179829731861736702779321621459595472258049074101567377883020018308
    ));
    // base conversion: mul 2**18 / 2**192
    r = r.asr(174);

    Ok(r)
}

#[cfg(test)]
mod tests {
    use ethers::signers::Signer;
    use hyperdrive_wrappers::wrappers::mock_fixed_point_math::MockFixedPointMath;
    use rand::{thread_rng, Rng};
    use test_utils::{chain::Chain, constants::DEPLOYER};

    use super::*;
    use crate::{fixed, uint256, FixedPoint};

    #[tokio::test]
    async fn fuzz_exp_narrow() -> Result<()> {
        let chain = Chain::connect(None, None).await?;
        chain.deal(DEPLOYER.address(), uint256!(100_000e18)).await?;
        let client = chain.client(DEPLOYER.clone()).await?;
        let mock_fixed_point_math = MockFixedPointMath::deploy(client, ())?.send().await?;

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..10_000 {
            let x: FixedPoint<I256> = rng.gen_range(fixed!(0)..=fixed!(1e18));
            let actual = ln(x.raw());
            match mock_fixed_point_math.ln(x.raw()).call().await {
                Ok(expected) => assert_eq!(actual.unwrap(), expected),
                Err(_) => assert!(actual.is_err()),
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn fuzz_exp() -> Result<()> {
        let chain = Chain::connect(None, None).await?;
        chain.deal(DEPLOYER.address(), uint256!(100_000e18)).await?;
        let client = chain.client(DEPLOYER.clone()).await?;
        let mock_fixed_point_math = MockFixedPointMath::deploy(client, ())?.send().await?;

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..10_000 {
            let x: FixedPoint<I256> = rng.gen_range(fixed!(0)..FixedPoint::<I256>::MAX);
            let actual = exp(x.raw());
            match mock_fixed_point_math.exp(x.raw()).call().await {
                Ok(expected) => assert_eq!(actual.unwrap(), expected),
                Err(_) => assert!(actual.is_err()),
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn fuzz_ln_narrow() -> Result<()> {
        let chain = Chain::connect(None, None).await?;
        chain.deal(DEPLOYER.address(), uint256!(100_000e18)).await?;
        let client = chain.client(DEPLOYER.clone()).await?;
        let mock_fixed_point_math = MockFixedPointMath::deploy(client, ())?.send().await?;

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..10_000 {
            let x: FixedPoint<I256> = rng.gen_range(fixed!(0)..=fixed!(1e18));
            let actual = ln(x.raw());
            match mock_fixed_point_math.ln(x.raw()).call().await {
                Ok(expected) => assert_eq!(actual.unwrap(), expected),
                Err(_) => assert!(actual.is_err()),
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn fuzz_ln() -> Result<()> {
        let chain = Chain::connect(None, None).await?;
        chain.deal(DEPLOYER.address(), uint256!(100_000e18)).await?;
        let client = chain.client(DEPLOYER.clone()).await?;
        let mock_fixed_point_math = MockFixedPointMath::deploy(client, ())?.send().await?;

        // Fuzz the rust and solidity implementations against each other.
        let mut rng = thread_rng();
        for _ in 0..10_000 {
            let x: FixedPoint<I256> = rng.gen_range(fixed!(0)..FixedPoint::<I256>::MAX);
            let actual = ln(x.raw());
            match mock_fixed_point_math.ln(x.raw()).call().await {
                Ok(expected) => assert_eq!(actual.unwrap(), expected),
                Err(_) => assert!(actual.is_err()),
            }
        }

        Ok(())
    }
}
