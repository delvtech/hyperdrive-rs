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
