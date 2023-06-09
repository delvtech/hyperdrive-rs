// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.19;

import { IHyperdrive } from "./IHyperdrive.sol";
import { IERC20 } from "./IERC20.sol";

interface IHyperdriveDeployer {
    function deploy(
        IHyperdrive.PoolConfig memory _config,
        address _dataProvider,
        bytes32 _linkerCodeHash,
        address _linkerFactory,
        bytes32[] memory _extraData
    ) external returns (address);
}
