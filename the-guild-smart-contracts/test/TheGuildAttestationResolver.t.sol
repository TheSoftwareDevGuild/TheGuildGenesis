// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {TheGuildActivityToken} from "../src/TheGuildActivityToken.sol";
import {TheGuildAttestationResolver} from "../src/TheGuildAttestationResolver.sol";
import {TheGuildBadgeRegistry} from "../src/TheGuildBadgeRegistry.sol";

// EAS contracts
import {EAS} from "eas-contracts/EAS.sol";
import {SchemaRegistry} from "eas-contracts/SchemaRegistry.sol";
import {IEAS, AttestationRequest, AttestationRequestData, Attestation, RevocationRequestData, RevocationRequest} from "eas-contracts/IEAS.sol";

contract TheGuildAttestationResolverTest is Test {
    TheGuildActivityToken private token;
    TheGuildAttestationResolver private resolver;
    TheGuildBadgeRegistry private badgeRegistry;
    SchemaRegistry private schemaRegistry;
    EAS private eas;

    address private attester = address(0xA11CE);
    address private recipient = address(0xBEEF);

    function setUp() public {
        // Deploy EAS stack locally
        schemaRegistry = new SchemaRegistry();
        eas = new EAS(schemaRegistry);

        // Deploy token, registry & resolver and transfer ownership to resolver
        token = new TheGuildActivityToken(address(this));
        badgeRegistry = new TheGuildBadgeRegistry();
        resolver = new TheGuildAttestationResolver(
            IEAS(address(eas)),
            token,
            badgeRegistry
        );
        token.transferOwnership(address(resolver));
    }

    function _registerSchema() internal returns (bytes32) {
        // Minimal schema aligned with README and scripts
        string memory schema = "bytes32 badgeName, bytes justification";
        // Register with our resolver so attestations invoke onAttest
        bytes32 schemaId = schemaRegistry.register(schema, resolver, true);
        return schemaId;
    }

    function test_AttestationMintsTokensToAttester() public {
        bytes32 schemaId = _registerSchema();

        // Ensure badge exists
        badgeRegistry.createBadge(
            bytes32("Rust"),
            bytes("Know how to code in Rust")
        );

        // Build attestation request
        AttestationRequestData memory data = AttestationRequestData({
            recipient: recipient,
            expirationTime: 0,
            revocable: true,
            refUID: bytes32(0),
            data: abi.encode(bytes32("Rust"), bytes("Saw them coding in Rust")),
            value: 0
        });

        AttestationRequest memory request = AttestationRequest({
            schema: schemaId,
            data: data
        });

        // Simulate attester calling EAS
        vm.prank(attester);
        eas.attest(request);

        // Resolver should mint 10 * 10^18 to attester
        assertEq(token.balanceOf(attester), 10 * (10 ** token.decimals()));
    }

    function test_AttestWithoutOwnershipTransferWouldFail() public {
        // Fresh token not owned by resolver
        TheGuildActivityToken tempToken = new TheGuildActivityToken(
            address(this)
        );
        TheGuildBadgeRegistry tempRegistry = new TheGuildBadgeRegistry();
        TheGuildAttestationResolver tempResolver = new TheGuildAttestationResolver(
                IEAS(address(eas)),
                tempToken,
                tempRegistry
            );

        bytes32 schemaId = schemaRegistry.register(
            "bytes32 badgeName, bytes justification",
            tempResolver,
            true
        );

        AttestationRequest memory request = AttestationRequest({
            schema: schemaId,
            data: AttestationRequestData({
                recipient: recipient,
                expirationTime: 0,
                revocable: true,
                refUID: bytes32(0),
                data: abi.encode(
                    bytes32("Solidity"),
                    bytes("Saw them coding in Solidity")
                ),
                value: 0
            })
        });

        vm.prank(attester);
        vm.expectRevert(); // tempResolver will try to mint but isn't owner
        eas.attest(request);
    }

    function test_AttestationFailsIfBadgeDoesNotExist() public {
        bytes32 schemaId = _registerSchema();

        AttestationRequest memory request = AttestationRequest({
            schema: schemaId,
            data: AttestationRequestData({
                recipient: recipient,
                expirationTime: 0,
                revocable: true,
                refUID: bytes32(0),
                data: abi.encode(
                    bytes32("NonExistent"),
                    bytes("Justification")
                ),
                value: 0
            })
        });

        vm.prank(attester);
        vm.expectRevert(); // resolver returns false -> EAS reverts
        eas.attest(request);
    }

    function test_AttestationIndexerGetters() public {
        bytes32 schemaId = _registerSchema();
        badgeRegistry.createBadge(
            bytes32("Rust"),
            bytes("Know how to code in Rust")
        );

        vm.prank(attester);
        bytes32 uid = eas.attest(
            AttestationRequest({
                schema: schemaId,
                data: AttestationRequestData({
                    recipient: recipient,
                    expirationTime: 0,
                    revocable: true,
                    refUID: bytes32(0),
                    data: abi.encode(bytes32("Rust"), bytes("any")),
                    value: 0
                })
            })
        );

        assertEq(resolver.getAttestationCount(), 1);
        Attestation memory a = resolver.getAttestationAtIndex(0);
        assertEq(a.uid, uid);
        assertEq(a.attester, attester);
        assertEq(a.recipient, recipient);
    }

    function test_Revoke_NoRevert() public {
        bytes32 schemaId = _registerSchema();
        badgeRegistry.createBadge(
            bytes32("Rust"),
            bytes("Know how to code in Rust")
        );

        vm.prank(attester);
        bytes32 uid = eas.attest(
            AttestationRequest({
                schema: schemaId,
                data: AttestationRequestData({
                    recipient: recipient,
                    expirationTime: 0,
                    revocable: true,
                    refUID: bytes32(0),
                    data: abi.encode(bytes32("Rust"), bytes("any")),
                    value: 0
                })
            })
        );

        vm.prank(attester);
        eas.revoke(
            RevocationRequest({
                schema: schemaId,
                data: RevocationRequestData({uid: uid, value: 0})
            })
        );
        // success == no revert
    }

    function test_DuplicateAttestationIsRejected() public {
        bytes32 schemaId = _registerSchema();
        // Ensure badge exists
        badgeRegistry.createBadge(
            bytes32("Rust"),
            bytes("Know how to code in Rust")
        );

        AttestationRequest memory request = AttestationRequest({
            schema: schemaId,
            data: AttestationRequestData({
                recipient: recipient,
                expirationTime: 0,
                revocable: true,
                refUID: bytes32(0),
                data: abi.encode(bytes32("Rust"), bytes("First")),
                value: 0
            })
        });

        // First attest succeeds
        vm.prank(attester);
        eas.attest(request);

        // Second identical attestation should revert (resolver returns false)
        vm.prank(attester);
        vm.expectRevert();
        eas.attest(request);
    }
}
