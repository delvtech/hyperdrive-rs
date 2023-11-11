// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.18;

import { Test } from "forge-std/Test.sol";
import { Hyperdrive } from "contracts/src/external/Hyperdrive.sol";
import { IERC20 } from "contracts/src/interfaces/IERC20.sol";
import { IHyperdrive } from "contracts/src/interfaces/IHyperdrive.sol";
import { HyperdriveStorage } from "contracts/src/internal/HyperdriveStorage.sol";
import { MockHyperdriveBase } from "contracts/test/MockHyperdrive.sol";
import { HyperdriveUtils } from "test/utils/HyperdriveUtils.sol";

contract DummyProvider {
    function get() external pure returns (uint256) {
        _revert(abi.encode(42));
    }

    fallback() external {}

    /// @dev Reverts with the provided bytes. This is useful in getters used
    ///      with the force-revert delegatecall pattern.
    /// @param _bytes The bytes to revert with.
    function _revert(bytes memory _bytes) internal pure {
        revert IHyperdrive.ReturnData(_bytes);
    }
}

contract DummyHyperdrive is Hyperdrive, MockHyperdriveBase {
    constructor()
        Hyperdrive(
            IHyperdrive.PoolConfig({
                baseToken: IERC20(address(0)),
                linkerFactory: address(0),
                linkerCodeHash: bytes32(0),
                initialSharePrice: 1e18,
                minimumShareReserves: 1e18,
                minimumTransactionAmount: 1e15,
                precisionThreshold: 1e14,
                positionDuration: 365 days,
                checkpointDuration: 1 days,
                timeStretch: HyperdriveUtils.calculateTimeStretch(0.05e18),
                governance: address(0),
                feeCollector: address(0),
                fees: IHyperdrive.Fees({ curve: 0, flat: 0, governance: 0 })
            }),
            address(new DummyProvider()),
            address(0)
        )
    {}
}

// This test verifies that the Hyperdrive contract's read-only delegation logic
// works as expected. Hyperdrive uses a "force-revert delegatecall" pattern that
// makes a delegatecall to a getter and expects it to revert with a specific
// error message.
contract ForceRevertDelegatecallTest is Test {
    DummyHyperdrive hyperdrive;

    function setUp() public {
        hyperdrive = new DummyHyperdrive();
    }

    function testRevertsOnUnderlyingSuccess() public {
        (bool success, bytes memory data) = address(hyperdrive).call{
            value: 0
        }("");

        if (success) {
            revert("Expected revert");
        }

        assert(data.length == 4);
        assert(bytes4(data) == bytes4(keccak256("UnexpectedSuccess()")));
    }

    function testCanFetchData() public {
        (bool success, bytes memory data) = address(hyperdrive).call{
            value: 0
        }(abi.encodeWithSignature("get()"));

        assert(success);

        assert(data.length == 32);
        assert(uint256(bytes32(data)) == 42);
    }
}