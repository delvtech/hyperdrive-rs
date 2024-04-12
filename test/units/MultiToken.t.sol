// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.20;

import { IHyperdrive } from "contracts/src/interfaces/IHyperdrive.sol";
import { AssetId } from "contracts/src/libraries/AssetId.sol";
import { ERC20ForwarderFactory } from "contracts/src/token/ERC20ForwarderFactory.sol";
import { MockAssetId } from "contracts/test/MockAssetId.sol";
import { IMockHyperdrive } from "contracts/test/MockHyperdrive.sol";
import { HyperdriveTest } from "test/utils/HyperdriveTest.sol";
import { Lib } from "test/utils/Lib.sol";

contract MultiTokenTest is HyperdriveTest {
    using Lib for *;

    function testFactory() public {
        assertEq(
            hyperdrive.getPoolConfig().linkerFactory,
            address(forwarderFactory)
        );
    }

    function testLinkerCodeHash() public {
        assertEq(
            hyperdrive.getPoolConfig().linkerCodeHash,
            forwarderFactory.ERC20LINK_HASH()
        );
    }

    function test__metadata() public {
        // Create a real tokenId.
        MockAssetId assetId = new MockAssetId();
        uint256 maturityTime = 126144000;
        uint256 id = assetId.encodeAssetId(
            AssetId.AssetIdPrefix.Long,
            maturityTime
        );

        // Generate expected token name and symbol.
        string memory expectedName = "Hyperdrive Long: 126144000";
        string memory expectedSymbol = "HYPERDRIVE-LONG:126144000";

        // Test that the name and symbol are correct.
        assertEq(hyperdrive.name(id), expectedName);
        assertEq(hyperdrive.symbol(id), expectedSymbol);
    }

    function testPermitForAll() public {
        uint256 privateKey = 0xBEEF;
        address owner = vm.addr(privateKey);

        uint256 deadline = block.timestamp + 1000;

        uint256 nonce = hyperdrive.nonces(owner);

        bytes32 structHash = keccak256(
            abi.encodePacked(
                "\x19\x01",
                hyperdrive.domainSeparator(),
                keccak256(
                    abi.encode(
                        hyperdrive.PERMIT_TYPEHASH(),
                        owner,
                        address(0xCAFE),
                        true,
                        nonce,
                        deadline
                    )
                )
            )
        );

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(privateKey, structHash);

        hyperdrive.permitForAll(
            owner,
            address(0xCAFE),
            true,
            deadline,
            v,
            r,
            s
        );

        assertEq(hyperdrive.isApprovedForAll(owner, address(0xCAFE)), true);

        // Check that nonce increments
        assertEq(hyperdrive.nonces(owner), nonce + 1);
    }

    function testNegativePermitBadNonce() public {
        uint256 privateKey = 0xBEEF;
        address owner = vm.addr(privateKey);

        uint256 deadline = block.timestamp + 1000;

        uint256 nonce = hyperdrive.nonces(owner);

        bytes32 structHash = keccak256(
            abi.encodePacked(
                "\x19\x01",
                hyperdrive.domainSeparator(),
                keccak256(
                    abi.encode(
                        hyperdrive.PERMIT_TYPEHASH(),
                        owner,
                        address(0xCAFE),
                        true,
                        nonce + 5,
                        deadline
                    )
                )
            )
        );

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(privateKey, structHash);

        vm.expectRevert();
        hyperdrive.permitForAll(
            owner,
            address(0xCAFE),
            true,
            deadline,
            v,
            r,
            s
        );

        assertEq(hyperdrive.isApprovedForAll(owner, address(0xCAFE)), false);
    }

    function testNegativePermitExpired() public {
        uint256 privateKey = 0xBEEF;
        address owner = vm.addr(privateKey);

        uint256 deadline = block.timestamp - 1;

        uint256 nonce = hyperdrive.nonces(owner);

        bytes32 structHash = keccak256(
            abi.encodePacked(
                "\x19\x01",
                hyperdrive.domainSeparator(),
                keccak256(
                    abi.encode(
                        hyperdrive.PERMIT_TYPEHASH(),
                        owner,
                        address(0xCAFE),
                        true,
                        nonce + 5,
                        deadline
                    )
                )
            )
        );

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(privateKey, structHash);

        vm.expectRevert();
        hyperdrive.permitForAll(
            owner,
            address(0xCAFE),
            true,
            deadline,
            v,
            r,
            s
        );

        assertEq(hyperdrive.isApprovedForAll(owner, address(0xCAFE)), false);
    }

    function testNegativePermitBadSignature() public {
        uint256 privateKey = 0xBEEF;
        address owner = vm.addr(privateKey);

        uint256 deadline = block.timestamp + 1000;

        uint256 nonce = hyperdrive.nonces(owner);

        bytes32 structHash = keccak256(
            abi.encodePacked(
                "\x19\x01",
                hyperdrive.domainSeparator(),
                keccak256(
                    abi.encode(
                        hyperdrive.PERMIT_TYPEHASH(),
                        owner,
                        address(0xCAFE),
                        true,
                        nonce,
                        deadline
                    )
                )
            )
        );

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(privateKey, structHash);

        vm.expectRevert();
        hyperdrive.permitForAll(
            owner,
            address(0xF00DBABE),
            true,
            deadline,
            v,
            r,
            s
        );

        assertEq(
            hyperdrive.isApprovedForAll(owner, address(0xF00DBABE)),
            false
        );
    }

    function testCannotTransferZeroAddrBatchTransferFrom() public {
        vm.expectRevert();
        hyperdrive.batchTransferFrom(
            alice,
            address(0),
            new uint256[](0),
            new uint256[](0)
        );

        vm.expectRevert();
        hyperdrive.batchTransferFrom(
            address(0),
            alice,
            new uint256[](0),
            new uint256[](0)
        );
    }

    function testCannotSendInconsistentLengths() public {
        vm.expectRevert();
        hyperdrive.batchTransferFrom(
            alice,
            bob,
            new uint256[](0),
            new uint256[](1)
        );

        vm.expectRevert();
        hyperdrive.batchTransferFrom(
            alice,
            bob,
            new uint256[](1),
            new uint256[](0)
        );
    }

    function testBatchTransferFrom() public {
        uint256 privateKey = 0xBEEF;
        address owner = vm.addr(privateKey);

        uint256 deadline = block.timestamp + 1000;

        uint256 nonce = hyperdrive.nonces(owner);

        bytes32 structHash = keccak256(
            abi.encodePacked(
                "\x19\x01",
                hyperdrive.domainSeparator(),
                keccak256(
                    abi.encode(
                        hyperdrive.PERMIT_TYPEHASH(),
                        owner,
                        address(0xCAFE),
                        true,
                        nonce,
                        deadline
                    )
                )
            )
        );

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(privateKey, structHash);

        hyperdrive.permitForAll(
            owner,
            address(0xCAFE),
            true,
            deadline,
            v,
            r,
            s
        );

        IMockHyperdrive(address(hyperdrive)).mint(1, owner, 100 ether);
        IMockHyperdrive(address(hyperdrive)).mint(2, owner, 50 ether);
        IMockHyperdrive(address(hyperdrive)).mint(3, owner, 10 ether);

        uint256[] memory ids = new uint256[](3);
        ids[0] = 1;
        ids[1] = 2;
        ids[2] = 3;

        uint256[] memory amounts = new uint256[](3);
        amounts[0] = 100 ether;
        amounts[1] = 50 ether;
        amounts[2] = 10 ether;

        vm.startPrank(address(0xCAFE));
        hyperdrive.batchTransferFrom(owner, bob, ids, amounts);
    }

    function testBatchTransferFromFailsWithoutApproval() public {
        uint256 privateKey = 0xBEEF;
        address owner = vm.addr(privateKey);

        IMockHyperdrive(address(hyperdrive)).mint(1, owner, 100 ether);
        IMockHyperdrive(address(hyperdrive)).mint(2, owner, 50 ether);
        IMockHyperdrive(address(hyperdrive)).mint(3, owner, 10 ether);

        uint256[] memory ids = new uint256[](3);
        ids[0] = 1;
        ids[1] = 2;
        ids[2] = 3;

        uint256[] memory amounts = new uint256[](3);
        amounts[0] = 100 ether;
        amounts[1] = 50 ether;
        amounts[2] = 10 ether;

        vm.expectRevert();
        hyperdrive.batchTransferFrom(owner, bob, ids, amounts);
    }
}
