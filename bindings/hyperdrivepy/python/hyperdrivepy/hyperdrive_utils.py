"""Python wrapper for the rust hyperdrive_math::utils module."""

from __future__ import annotations

# pylint: disable=no-name-in-module
from . import hyperdrivepy as rust_module  # type: ignore


def calculate_time_stretch(rate: str, position_duration: str) -> str:
    """Calculate the time stretch parameter given a pool's spot rate.

    ..math::
        tau = 5.24592 / (0.4665 * r * 100)

    Arguments
    ---------
    rate: str (FixedPoint)
        The pool's spot rate (aka apr, or fixed rate).
    position_duration: str(FixedPoint)
        The amount of time before a trade matures.

    Returns
    -------
    time_stretch: str (FixedPoint)
        The time stretch parameter (tau).
    """
    return rust_module.calculate_time_stretch(rate, position_duration)


def calculate_effective_share_reserves(
    share_reserves: str,
    share_adjustment: str,
) -> str:
    r"""Calculate the effective share reserves given the share reserves and share adjustment.

    ..math::
        z_effective = z - \zeta

    Arguments
    ---------
    share_reserves: str (FixedPoint)
        The pool's share reserves.
    share_adjustment: str (I256)
        The zeta factor for adjusting share reserves.

    Returns
    -------
    effective_share_reserves: str (FixedPoint)
        The adjusted share reserves, accounting for the zeta factor.
    """
    return rust_module.calculate_effective_share_reserves(share_reserves, share_adjustment)


def calculate_bonds_given_effective_shares_and_rate(
    effective_share_reserves: str,
    target_rate: str,
    initial_vault_share_price: str,
    position_duration: str,
    time_stretch: str,
) -> str:
    """Calculates the bond reserves assuming that the pool has a given
    effective share reserves and fixed rate APR.

    ..note::
        This function should not be used for computing reserve levels when
        initializing a pool. Instead use `lp::calculate_initial_reserves`.

    ..math::
        r = ((1/p)-1)/t = (1-p)/(pt)
        p = ((u * z) / y) ** t
        y = mu * (z - zeta) * (1 + apr * t) ** (1/tau)

    Arguments
    ---------
    effective_share_reserves: str (FixedPoint)
        The pool's effective share reserves. The effective share
        reserves are a modified version of the share reserves
        used when pricing trades.
    target_rate: str (FixedPoint)
        The target pool's fixed APR.
    initial_vault_share_price: str (FixedPoint)
        The pool's initial share price.
    position_duration: str (FixedPoint)
        The amount of time until maturity in seconds.
    time_stretch: str (FixedPoint)
        The time stretch parameter (tau).

    Returns
    -------
    bond_reserves: str (FixedPoint)
        The bond reserves that make the pool have a specified APR.
    """
    return rust_module.calculate_bonds_given_effective_shares_and_rate(
        effective_share_reserves, target_rate, initial_vault_share_price, position_duration, time_stretch
    )


def calculate_rate_given_fixed_price(
    price: str,
    position_duration: str,
) -> str:
    """Calculate the rate assuming a given price is constant for some annualized duration.

    We calculate the rate for a fixed length of time as:

    ..math::
        r = (1 - p) / (p t)

    where $p$ is the price and $t$ is the length of time that this price is
    assumed to be constant, in units of years. For example, if the price is
    constant for 6 months, then $t=0.5$.
    In our case, $t = \text{position_duration} / (60*60*24*365)$.

    Arguments
    ---------
    price: str (FixedPoint)
        The spot price of bonds fora given pool.
    position_duration: str (FixedPoint)
        The pool bond position duration, over which the price is assumed to be constant.
    """
    return rust_module.calculate_rate_given_fixed_price(price, position_duration)
