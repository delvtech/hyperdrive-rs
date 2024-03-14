// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.20;

import { ERC4626HyperdriveCoreDeployer } from "contracts/src/deployers/erc4626/ERC4626HyperdriveCoreDeployer.sol";
import { ERC4626HyperdriveDeployerCoordinator } from "contracts/src/deployers/erc4626/ERC4626HyperdriveDeployerCoordinator.sol";
import { ERC4626Target0Deployer } from "contracts/src/deployers/erc4626/ERC4626Target0Deployer.sol";
import { ERC4626Target1Deployer } from "contracts/src/deployers/erc4626/ERC4626Target1Deployer.sol";
import { ERC4626Target2Deployer } from "contracts/src/deployers/erc4626/ERC4626Target2Deployer.sol";
import { ERC4626Target3Deployer } from "contracts/src/deployers/erc4626/ERC4626Target3Deployer.sol";
import { ERC4626Target4Deployer } from "contracts/src/deployers/erc4626/ERC4626Target4Deployer.sol";
import { HyperdriveFactory } from "contracts/src/factory/HyperdriveFactory.sol";
import { IERC20 } from "contracts/src/interfaces/IERC20.sol";
import { IERC4626 } from "contracts/src/interfaces/IERC4626.sol";
import { IHyperdrive } from "contracts/src/interfaces/IHyperdrive.sol";
import { IHyperdriveDeployerCoordinator } from "contracts/src/interfaces/IHyperdriveDeployerCoordinator.sol";
import { ILido } from "contracts/src/interfaces/ILido.sol";
import { AssetId } from "contracts/src/libraries/AssetId.sol";
import { FixedPointMath, ONE } from "contracts/src/libraries/FixedPointMath.sol";
import { HyperdriveMath } from "contracts/src/libraries/HyperdriveMath.sol";
import { ERC20ForwarderFactory } from "contracts/src/token/ERC20ForwarderFactory.sol";
import { HyperdriveTest } from "test/utils/HyperdriveTest.sol";
import { MockERC4626Hyperdrive } from "contracts/test/MockERC4626Hyperdrive.sol";
import { ERC20Mintable } from "contracts/test/ERC20Mintable.sol";
import { MockERC4626 } from "contracts/test/MockERC4626.sol";
import { HyperdriveUtils } from "test/utils/HyperdriveUtils.sol";
import { Lib } from "test/utils/Lib.sol";
import { ERC4626ValidationTest } from "./ERC4626Validation.t.sol";

contract UsdcERC4626 is ERC4626ValidationTest {
    using FixedPointMath for *;
    using Lib for *;

    function setUp() public override {
        super.setUp();
        vm.startPrank(deployer);
        decimals = 6;
        underlyingToken = IERC20(
            address(new ERC20Mintable("usdc", "USDC", 6, address(this), false))
        );
        token = IERC4626(
            address(
                new MockERC4626(
                    ERC20Mintable(address(underlyingToken)),
                    "yearn usdc",
                    "yUSDC",
                    0,
                    address(0),
                    false
                )
            )
        );
        uint256 monies = 1_000_000_000e6;
        ERC20Mintable(address(underlyingToken)).mint(deployer, monies);
        ERC20Mintable(address(underlyingToken)).mint(alice, monies);
        ERC20Mintable(address(underlyingToken)).mint(bob, monies);

        // Initialize deployer contracts and forwarder.
        coreDeployer = address(new ERC4626HyperdriveCoreDeployer());
        target0Deployer = address(new ERC4626Target0Deployer());
        target1Deployer = address(new ERC4626Target1Deployer());
        target2Deployer = address(new ERC4626Target2Deployer());
        target3Deployer = address(new ERC4626Target3Deployer());
        target4Deployer = address(new ERC4626Target4Deployer());
        deployerCoordinator = address(
            new ERC4626HyperdriveDeployerCoordinator(
                coreDeployer,
                target0Deployer,
                target1Deployer,
                target2Deployer,
                target3Deployer,
                target4Deployer
            )
        );

        address[] memory defaults = new address[](1);
        defaults[0] = bob;
        forwarderFactory = new ERC20ForwarderFactory();

        // Hyperdrive factory to produce ERC4626 instances for UsdcERC4626.
        factory = new HyperdriveFactory(
            HyperdriveFactory.FactoryConfig({
                governance: alice,
                hyperdriveGovernance: bob,
                feeCollector: feeCollector,
                sweepCollector: sweepCollector,
                defaultPausers: defaults,
                checkpointDurationResolution: 1 hours,
                minCheckpointDuration: 8 hours,
                maxCheckpointDuration: 1 days,
                minPositionDuration: 7 days,
                maxPositionDuration: 10 * 365 days,
                minFixedAPR: 0.001e18,
                maxFixedAPR: 0.5e18,
                minTimeStretchAPR: 0.005e18,
                maxTimeStretchAPR: 0.5e18,
                minFees: IHyperdrive.Fees({
                    curve: 0,
                    flat: 0,
                    governanceLP: 0,
                    governanceZombie: 0
                }),
                maxFees: IHyperdrive.Fees({
                    curve: ONE,
                    flat: ONE,
                    governanceLP: ONE,
                    governanceZombie: ONE
                }),
                linkerFactory: address(0xdeadbeef),
                linkerCodeHash: bytes32(uint256(0xdeadbabe))
            })
        );

        // Config changes required to support ERC4626 with the correct initial vault share price.
        IHyperdrive.PoolDeployConfig memory config = testDeployConfig(
            FIXED_RATE,
            POSITION_DURATION
        );
        config.governance = factory.hyperdriveGovernance();
        config.feeCollector = factory.feeCollector();
        config.linkerFactory = factory.linkerFactory();
        config.linkerCodeHash = factory.linkerCodeHash();
        config.timeStretch = 0;
        config.baseToken = underlyingToken;
        config.minimumTransactionAmount = 1e6;
        config.minimumShareReserves = 1e6;
        uint256 contribution = 7_500e6;
        vm.stopPrank();
        vm.startPrank(alice);

        factory.addDeployerCoordinator(deployerCoordinator);

        // Set approval to allow initial contribution to factory.
        underlyingToken.approve(
            address(deployerCoordinator),
            type(uint256).max
        );

        // Deploy and set hyperdrive instance.
        bytes memory extraData = abi.encode(address(token));
        factory.deployTarget(
            bytes32(uint256(0xbeefbabe)),
            deployerCoordinator,
            config,
            extraData,
            FIXED_RATE,
            FIXED_RATE,
            0,
            bytes32(uint256(0xdeadfade))
        );
        factory.deployTarget(
            bytes32(uint256(0xbeefbabe)),
            deployerCoordinator,
            config,
            extraData,
            FIXED_RATE,
            FIXED_RATE,
            1,
            bytes32(uint256(0xdeadfade))
        );
        factory.deployTarget(
            bytes32(uint256(0xbeefbabe)),
            deployerCoordinator,
            config,
            extraData,
            FIXED_RATE,
            FIXED_RATE,
            2,
            bytes32(uint256(0xdeadfade))
        );
        factory.deployTarget(
            bytes32(uint256(0xbeefbabe)),
            deployerCoordinator,
            config,
            extraData,
            FIXED_RATE,
            FIXED_RATE,
            3,
            bytes32(uint256(0xdeadfade))
        );
        factory.deployTarget(
            bytes32(uint256(0xbeefbabe)),
            deployerCoordinator,
            config,
            extraData,
            FIXED_RATE,
            FIXED_RATE,
            4,
            bytes32(uint256(0xdeadfade))
        );
        hyperdrive = factory.deployAndInitialize(
            bytes32(uint256(0xbeefbabe)),
            deployerCoordinator,
            config,
            extraData,
            contribution,
            FIXED_RATE,
            FIXED_RATE,
            IHyperdrive.Options({
                asBase: true,
                destination: alice,
                extraData: new bytes(0)
            }),
            bytes32(uint256(0xdeadfade))
        );

        // Setup maximum approvals so transfers don't require further approval.
        underlyingToken.approve(address(hyperdrive), type(uint256).max);
        underlyingToken.approve(address(token), type(uint256).max);
        token.approve(address(hyperdrive), type(uint256).max);
        vm.stopPrank();

        // Start recording events.
        vm.recordLogs();
    }

    function advanceTimeWithYield(
        uint256 timeDelta,
        int256 variableRate
    ) public override {
        vm.warp(block.timestamp + timeDelta);
        (, int256 interest) = HyperdriveUtils.calculateCompoundInterest(
            underlyingToken.balanceOf(address(token)),
            variableRate,
            timeDelta
        );
        if (interest > 0) {
            ERC20Mintable(address(underlyingToken)).mint(
                address(token),
                uint256(interest)
            );
        } else if (interest < 0) {
            ERC20Mintable(address(underlyingToken)).burn(
                address(token),
                uint256(-interest)
            );
        }
    }
}
