// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.19;

import { IERC20 } from "../interfaces/IERC20.sol";
import { IHyperdrive } from "../interfaces/IHyperdrive.sol";
import { IHyperdriveDeployer } from "../interfaces/IHyperdriveDeployer.sol";
import { ERC4626DataProvider } from "../instances/ERC4626DataProvider.sol";
import { HyperdriveFactory } from "./HyperdriveFactory.sol";
import { IERC4626 } from "../interfaces/IERC4626.sol";

/// @author DELV
/// @title ERC4626HyperdriveFactory
/// @notice Deploys hyperdrive instances and initializes them. It also holds a
///         registry of all deployed hyperdrive instances.
/// @custom:disclaimer The language used in this code is for coding convenience
///                    only, and is not intended to, and does not, have any
///                    particular legal or regulatory significance.
contract ERC4626HyperdriveFactory is HyperdriveFactory {
    IERC4626 internal immutable pool;

    /// @notice Deploys the contract
    /// @param _factoryConfig The variables that configure the factory;
    /// @param _deployer The contract which holds the bytecode and deploys new versions.
    /// @param _linkerFactory The address of the linker factory.
    /// @param _linkerCodeHash The hash of the linker contract's constructor code.
    /// @param _pool The Maker ERC4626 manger contract address
    constructor(
        FactoryConfig memory _factoryConfig,
        IHyperdriveDeployer _deployer,
        address _linkerFactory,
        bytes32 _linkerCodeHash,
        IERC4626 _pool
    )
        HyperdriveFactory(
            _factoryConfig,
            _deployer,
            _linkerFactory,
            _linkerCodeHash
        )
    {
        pool = _pool;
    }

    /// @notice This deploys a data provider for the ERC4626 hyperdrive instance
    /// @param _config The configuration of the pool we are deploying
    /// @param _linkerCodeHash The code hash from the multitoken deployer
    /// @param _linkerFactory The factory of the multitoken deployer
    function deployDataProvider(
        IHyperdrive.PoolConfig memory _config,
        bytes32[] memory,
        bytes32 _linkerCodeHash,
        address _linkerFactory
    ) internal override returns (address) {
        return (
            address(
                new ERC4626DataProvider(
                    _config,
                    _linkerCodeHash,
                    _linkerFactory,
                    pool
                )
            )
        );
    }
}
