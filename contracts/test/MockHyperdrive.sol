// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.20;

import { Hyperdrive } from "contracts/src/external/Hyperdrive.sol";
import { HyperdriveTarget0 } from "contracts/src/external/HyperdriveTarget0.sol";
import { HyperdriveTarget1 } from "contracts/src/external/HyperdriveTarget1.sol";
import { HyperdriveTarget2 } from "contracts/src/external/HyperdriveTarget2.sol";
import { HyperdriveTarget3 } from "contracts/src/external/HyperdriveTarget3.sol";
import { HyperdriveTarget4 } from "contracts/src/external/HyperdriveTarget4.sol";
import { HyperdriveBase } from "contracts/src/internal/HyperdriveBase.sol";
import { IHyperdrive } from "contracts/src/interfaces/IHyperdrive.sol";
import { IHyperdrive } from "contracts/src/interfaces/IHyperdrive.sol";
import { ETH } from "contracts/src/libraries/Constants.sol";
import { FixedPointMath } from "contracts/src/libraries/FixedPointMath.sol";
import { ERC20Mintable } from "contracts/test/ERC20Mintable.sol";
import { HyperdriveUtils } from "test/utils/HyperdriveUtils.sol";

interface IMockHyperdrive {
    function accrue(uint256 time, int256 apr) external;

    function calculateTimeRemaining(
        uint256 _maturityTime
    ) external view returns (uint256);

    function calculateTimeRemainingScaled(
        uint256 _maturityTime
    ) external view returns (uint256);

    function latestCheckpoint() external view returns (uint256);

    function updateLiquidity(uint256 shareReservesDelta) external;

    function setReserves(uint256 shareReserves, uint256 bondReserves) external;

    function getGovernanceFeesAccrued() external view returns (uint256);
}

abstract contract MockHyperdriveBase is HyperdriveBase {
    using FixedPointMath for uint256;

    uint256 internal totalShares;

    /// @dev Accepts a deposit from the user in base.
    /// @param _baseAmount The base amount to deposit.
    /// @return The shares that were minted in the deposit.
    /// @return The amount of ETH to refund. Since this yield source isn't
    ///         payable, this is always zero.
    function _depositWithBase(
        uint256 _baseAmount,
        bytes calldata // unused
    ) internal override returns (uint256, uint256) {
        // Calculate the total amount of assets.
        uint256 assets;
        if (address(_baseToken) == ETH) {
            assets = address(this).balance;
        } else {
            assets = _baseToken.balanceOf(address(this));
        }

        // Transfer the specified amount of funds from the trader. If the trader
        // overpaid, we return the excess amount.
        bool success = true;
        uint256 refund;
        if (address(_baseToken) == ETH) {
            if (msg.value < _baseAmount) {
                revert IHyperdrive.TransferFailed();
            }
            refund = msg.value - _baseAmount;
        } else {
            success = _baseToken.transferFrom(
                msg.sender,
                address(this),
                _baseAmount
            );
        }
        if (!success) {
            revert IHyperdrive.TransferFailed();
        }

        // Increase the total shares and return with the amount of shares minted
        // and the current share price.
        if (totalShares == 0) {
            totalShares = _baseAmount.divDown(_initialVaultSharePrice);
            return (totalShares, refund);
        } else {
            uint256 newShares = _baseAmount.mulDivDown(totalShares, assets);
            totalShares += newShares;
            return (newShares, refund);
        }
    }

    /// @dev Process a deposit in vault shares.
    /// @param _shareAmount The vault shares amount to deposit.
    function _depositWithShares(
        uint256 _shareAmount,
        bytes calldata // unused
    ) internal override {
        // Calculate the base amount of the deposit.
        uint256 baseAmount = _convertToBase(_shareAmount);

        // Increase the total shares and return with the amount of shares minted
        // and the current share price.
        if (totalShares == 0) {
            totalShares = baseAmount.divDown(_initialVaultSharePrice);
        } else {
            uint256 newShares = _convertToShares(baseAmount);
            totalShares += newShares;
        }

        // Transfer the specified amount of funds from the trader. If the trader
        // overpaid, we return the excess amount.
        bool success = true;
        uint256 refund;
        if (address(_baseToken) == ETH) {
            if (msg.value < baseAmount) {
                revert IHyperdrive.TransferFailed();
            }
            refund = msg.value - baseAmount;
        } else {
            success = _baseToken.transferFrom(
                msg.sender,
                address(this),
                baseAmount
            );
        }
        if (!success) {
            revert IHyperdrive.TransferFailed();
        }
    }

    /// @dev Process a withdrawal in base and send the proceeds to the
    ///      destination.
    /// @param _shareAmount The amount of vault shares to withdraw.
    /// @param _destination The destination of the withdrawal.
    /// @return amountWithdrawn The amount of base withdrawn.
    function _withdrawWithBase(
        uint256 _shareAmount,
        address _destination,
        bytes calldata // unused
    ) internal override returns (uint256 amountWithdrawn) {
        // If the shares to withdraw is greater than the total shares, we clamp
        // to the total shares.
        _shareAmount = _shareAmount > totalShares ? totalShares : _shareAmount;

        // Calculate the base proceeds.
        uint256 withdrawValue = _convertToBase(_shareAmount);

        // Transfer the base proceeds to the destination and burn the shares.
        totalShares -= _shareAmount;
        bool success;
        if (address(_baseToken) == ETH) {
            (success, ) = payable(_destination).call{ value: withdrawValue }(
                ""
            );
        } else {
            success = _baseToken.transfer(_destination, withdrawValue);
        }
        if (!success) {
            revert IHyperdrive.TransferFailed();
        }

        return withdrawValue;
    }

    /// @dev Process a withdrawal in vault shares and send the proceeds to the
    ///      destination.
    /// @param _shareAmount The amount of vault shares to withdraw.
    /// @param _destination The destination of the withdrawal.
    function _withdrawWithShares(
        uint256 _shareAmount,
        address _destination,
        bytes calldata // unused
    ) internal override {
        // If the shares to withdraw is greater than the total shares, we clamp
        // to the total shares.
        _shareAmount = _shareAmount > totalShares ? totalShares : _shareAmount;

        // Calculate the base proceeds.
        uint256 withdrawValue = _convertToBase(_shareAmount);

        // Transfer the base proceeds to the destination and burn the shares.
        totalShares -= _shareAmount;
        bool success;
        if (address(_baseToken) == ETH) {
            (success, ) = payable(_destination).call{ value: withdrawValue }(
                ""
            );
        } else {
            success = _baseToken.transfer(_destination, withdrawValue);
        }
        if (!success) {
            revert IHyperdrive.TransferFailed();
        }
    }

    // This overrides checkMessageValue to serve the dual purpose of making
    // ETH yield source instances to be payable and non-ETH yield
    // source instances non-payable.
    function _checkMessageValue() internal view override {
        if (address(_baseToken) != ETH && msg.value > 0) {
            revert IHyperdrive.NotPayable();
        }
    }

    /// @dev Convert an amount of vault shares to an amount of base.
    /// @param _shareAmount The vault shares amount.
    /// @return The base amount.
    function _convertToBase(
        uint256 _shareAmount
    ) internal view override returns (uint256) {
        // Get the total amount of base held in Hyperdrive.
        uint256 assets;
        if (address(_baseToken) == ETH) {
            assets = address(this).balance;
        } else {
            assets = _baseToken.balanceOf(address(this));
        }

        return
            totalShares != 0 ? _shareAmount.mulDivDown(assets, totalShares) : 0;
    }

    /// @dev Convert an amount of base to an amount of vault shares.
    /// @param _baseAmount The base amount.
    /// @return The vault shares amount.
    function _convertToShares(
        uint256 _baseAmount
    ) internal view override returns (uint256) {
        // Get the total amount of base held in Hyperdrive.
        uint256 assets;
        if (address(_baseToken) == ETH) {
            assets = address(this).balance;
        } else {
            assets = _baseToken.balanceOf(address(this));
        }

        return _baseAmount.mulDivDown(totalShares, assets);
    }
}

contract MockHyperdrive is Hyperdrive, MockHyperdriveBase {
    using FixedPointMath for uint256;

    constructor(
        IHyperdrive.PoolConfig memory _config
    )
        Hyperdrive(
            _config,
            address(new MockHyperdriveTarget0(_config)),
            address(new MockHyperdriveTarget1(_config)),
            address(new MockHyperdriveTarget2(_config)),
            address(new MockHyperdriveTarget3(_config)),
            address(new MockHyperdriveTarget4(_config))
        )
    {}

    /// Mocks ///

    function setMarketState(
        IHyperdrive.MarketState memory _marketState_
    ) external {
        _marketState = _marketState_;
    }

    function setTotalShares(uint256 _totalShares) external {
        totalShares = _totalShares;
    }

    // Accrues compounded interest for a given number of seconds and readjusts
    // share price to reflect such compounding
    function accrue(uint256 time, int256 apr) external {
        (, int256 interest) = HyperdriveUtils.calculateCompoundInterest(
            _baseToken.balanceOf(address(this)),
            apr,
            time
        );

        if (interest > 0) {
            ERC20Mintable(address(_baseToken)).mint(
                address(this),
                uint256(interest)
            );
        } else if (interest < 0) {
            ERC20Mintable(address(_baseToken)).burn(
                address(this),
                uint256(-interest)
            );
        }
    }

    function calculateFeesGivenShares(
        uint256 _shareAmount,
        uint256 _spotPrice,
        uint256 vaultSharePrice
    ) external view returns (uint256 curveFee, uint256 governanceCurveFee) {
        (curveFee, governanceCurveFee) = _calculateFeesGivenShares(
            _shareAmount,
            _spotPrice,
            vaultSharePrice
        );
        return (curveFee, governanceCurveFee);
    }

    function calculateFeesGivenBonds(
        uint256 _bondAmount,
        uint256 _normalizedTimeRemaining,
        uint256 _spotPrice,
        uint256 vaultSharePrice
    )
        external
        view
        returns (
            uint256 totalCurveFee,
            uint256 totalFlatFee,
            uint256 governanceCurveFee,
            uint256 totalGovernanceFee
        )
    {
        (
            totalCurveFee,
            totalFlatFee,
            governanceCurveFee,
            totalGovernanceFee
        ) = _calculateFeesGivenBonds(
            _bondAmount,
            _normalizedTimeRemaining,
            _spotPrice,
            vaultSharePrice
        );
        return (
            totalCurveFee,
            totalFlatFee,
            governanceCurveFee,
            totalGovernanceFee
        );
    }

    // Calls Hyperdrive._calculateOpenLong
    function calculateOpenLong(
        uint256 _shareAmount,
        uint256 _vaultSharePrice
    )
        external
        view
        returns (
            uint256 shareReservesDelta,
            uint256 bondReservesDelta,
            uint256 totalGovernanceFee
        )
    {
        return _calculateOpenLong(_shareAmount, _vaultSharePrice);
    }

    function calculateTimeRemaining(
        uint256 _maturityTime
    ) external view returns (uint256 timeRemaining) {
        return _calculateTimeRemaining(_maturityTime);
    }

    function calculateTimeRemainingScaled(
        uint256 _maturityTime
    ) external view returns (uint256 timeRemaining) {
        return _calculateTimeRemainingScaled(_maturityTime);
    }

    function latestCheckpoint() external view returns (uint256 checkpointTime) {
        return _latestCheckpoint();
    }

    function updateLiquidity(int256 _shareReservesDelta) external {
        _updateLiquidity(_shareReservesDelta);
    }

    function calculateIdleShareReserves(
        uint256 _vaultSharePrice
    ) external view returns (uint256) {
        return _calculateIdleShareReserves(_vaultSharePrice);
    }

    function getTotalShares() external view returns (uint256) {
        return totalShares;
    }

    function setReserves(uint128 shareReserves, uint128 bondReserves) external {
        _marketState.shareReserves = shareReserves;
        _marketState.bondReserves = bondReserves;
    }

    function setLongExposure(uint128 longExposure) external {
        _marketState.longExposure = longExposure;
    }
}

contract MockHyperdriveTarget0 is HyperdriveTarget0, MockHyperdriveBase {
    constructor(
        IHyperdrive.PoolConfig memory _config
    ) HyperdriveTarget0(_config) {}

    /// Mocks ///

    function getGovernanceFeesAccrued() external view returns (uint256) {
        _revert(abi.encode(_governanceFeesAccrued));
    }
}

contract MockHyperdriveTarget1 is HyperdriveTarget1, MockHyperdriveBase {
    constructor(
        IHyperdrive.PoolConfig memory _config
    ) HyperdriveTarget1(_config) {}
}

contract MockHyperdriveTarget2 is HyperdriveTarget2, MockHyperdriveBase {
    constructor(
        IHyperdrive.PoolConfig memory _config
    ) HyperdriveTarget2(_config) {}
}

contract MockHyperdriveTarget3 is HyperdriveTarget3, MockHyperdriveBase {
    constructor(
        IHyperdrive.PoolConfig memory _config
    ) HyperdriveTarget3(_config) {}
}

contract MockHyperdriveTarget4 is HyperdriveTarget4, MockHyperdriveBase {
    constructor(
        IHyperdrive.PoolConfig memory _config
    ) HyperdriveTarget4(_config) {}
}
