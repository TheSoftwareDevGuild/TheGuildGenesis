// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {TheGuildInternalResolver} from "../src/TheGuildInternalResolver.sol";


import {EAS} from "eas-contracts/EAS.sol";
import {SchemaRegistry} from "eas-contracts/SchemaRegistry.sol";
import {IEAS, AttestationRequest, AttestationRequestData, Attestation, RevocationRequestData, RevocationRequest} from "eas-contracts/IEAS.sol";

contract TheGuildInternalResolverTest is Test {
    TheGuildInternalResolver private resolver;
    SchemaRegistry private schemaRegistry;
    EAS private eas;

    address private owner = address(this);
    address private authorizedAttester = address(0xA11CE);
    address private unauthorizedAttester = address(0xBEEF);
    address private recipient = address(0xCAFE);

    function setUp() public {
        
        schemaRegistry = new SchemaRegistry();
        eas = new EAS(schemaRegistry);

        
        resolver = new TheGuildInternalResolver(
            IEAS(address(eas)),
            owner
        );
    }

    function _registerSchema() internal returns (bytes32) {
        
        string memory schema = "string skillDescription, bytes32[] linkedBadges";
        
        bytes32 schemaId = schemaRegistry.register(schema, resolver, true);
        return schemaId;
    }

    function test_AttestationByOwnerSucceeds() public {
        bytes32 schemaId = _registerSchema();

        bytes32[] memory linkedBadges = new bytes32[](2);
        linkedBadges[0] = bytes32("Solidity");
        linkedBadges[1] = bytes32("Rust");

        AttestationRequestData memory data = AttestationRequestData({
            recipient: recipient,
            expirationTime: 0,
            revocable: true,
            refUID: bytes32(0),
            data: abi.encode("Master of Smart Contracts", linkedBadges),
            value: 0
        });

        AttestationRequest memory request = AttestationRequest({
            schema: schemaId,
            data: data
        });

        
        eas.attest(request);
        
    }

    function test_AttestationByAuthorizedAttesterSucceeds() public {
        bytes32 schemaId = _registerSchema();
        resolver.setAuthorizedAttester(authorizedAttester, true);

        bytes32[] memory linkedBadges = new bytes32[](1);
        linkedBadges[0] = bytes32("Solidity");

        AttestationRequestData memory data = AttestationRequestData({
            recipient: recipient,
            expirationTime: 0,
            revocable: true,
            refUID: bytes32(0),
            data: abi.encode("Solidity Dev", linkedBadges),
            value: 0
        });

        AttestationRequest memory request = AttestationRequest({
            schema: schemaId,
            data: data
        });

        vm.prank(authorizedAttester);
        eas.attest(request);
        
    }

    function test_AttestationByUnauthorizedAttesterFails() public {
        bytes32 schemaId = _registerSchema();

        bytes32[] memory linkedBadges = new bytes32[](1);
        linkedBadges[0] = bytes32("Solidity");

        AttestationRequestData memory data = AttestationRequestData({
            recipient: recipient,
            expirationTime: 0,
            revocable: true,
            refUID: bytes32(0),
            data: abi.encode("Solidity Dev", linkedBadges),
            value: 0
        });

        AttestationRequest memory request = AttestationRequest({
            schema: schemaId,
            data: data
        });

        vm.prank(unauthorizedAttester);
        vm.expectRevert(); 
        eas.attest(request);
    }

    function test_RevocationByAuthorizedAttesterSucceeds() public {
        bytes32 schemaId = _registerSchema();
        resolver.setAuthorizedAttester(authorizedAttester, true);

        bytes32[] memory linkedBadges = new bytes32[](1);
        linkedBadges[0] = bytes32("Solidity");

        AttestationRequestData memory data = AttestationRequestData({
            recipient: recipient,
            expirationTime: 0,
            revocable: true,
            refUID: bytes32(0),
            data: abi.encode("Solidity Dev", linkedBadges),
            value: 0
        });

        AttestationRequest memory request = AttestationRequest({
            schema: schemaId,
            data: data
        });

        vm.prank(authorizedAttester);
        bytes32 uid = eas.attest(request);

        vm.prank(authorizedAttester);
        eas.revoke(
            RevocationRequest({
                schema: schemaId,
                data: RevocationRequestData({uid: uid, value: 0})
            })
        );
        // success == no revert
    }

    function test_RevocationByUnauthorizedAttesterFails() public {
        bytes32 schemaId = _registerSchema();
        resolver.setAuthorizedAttester(authorizedAttester, true);

        bytes32[] memory linkedBadges = new bytes32[](1);
        linkedBadges[0] = bytes32("Solidity");

        AttestationRequestData memory data = AttestationRequestData({
            recipient: recipient,
            expirationTime: 0,
            revocable: true,
            refUID: bytes32(0),
            data: abi.encode("Solidity Dev", linkedBadges),
            value: 0
        });

        AttestationRequest memory request = AttestationRequest({
            schema: schemaId,
            data: data
        });

        vm.prank(authorizedAttester);
        bytes32 uid = eas.attest(request);

        vm.prank(unauthorizedAttester);
        vm.expectRevert(); 
        eas.revoke(
            RevocationRequest({
                schema: schemaId,
                data: RevocationRequestData({uid: uid, value: 0})
            })
        );
    }

    function test_DeauthorizeAttester() public {
        bytes32 schemaId = _registerSchema();
        resolver.setAuthorizedAttester(authorizedAttester, true);
        assertTrue(resolver.isAuthorizedAttester(authorizedAttester));

        resolver.setAuthorizedAttester(authorizedAttester, false);
        assertFalse(resolver.isAuthorizedAttester(authorizedAttester));

        bytes32[] memory linkedBadges = new bytes32[](1);
        linkedBadges[0] = bytes32("Solidity");

        AttestationRequest memory request = AttestationRequest({
            schema: schemaId,
            data: AttestationRequestData({
                recipient: recipient,
                expirationTime: 0,
                revocable: true,
                refUID: bytes32(0),
                data: abi.encode("Solidity Dev", linkedBadges),
                value: 0
            })
        });

        vm.prank(authorizedAttester);
        vm.expectRevert();
        eas.attest(request);
    }
}
