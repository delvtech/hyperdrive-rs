// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.20;

import { IHyperdrive } from "./IHyperdrive.sol";

interface IHyperdriveTargetDeployer {
    /// @notice Deploys a target instance with the given parameters.
    /// @param _config The configuration of the Hyperdrive pool.
    /// @param _extraData The extra data that contains the pool and sweep targets.
    /// @param _salt The create2 salt used in the deployment.
    /// @return The address of the newly deployed target instance.
    function deploy(
        IHyperdrive.PoolConfig memory _config,
        bytes memory _extraData,
        bytes32 _salt
    ) external returns (address);
}
