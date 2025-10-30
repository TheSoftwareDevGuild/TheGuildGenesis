// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import {Script} from "forge-std/Script.sol";
import {EAS} from "eas-contracts/EAS.sol";
import {AttestationRequestData, AttestationRequest} from "eas-contracts/IEAS.sol";
import {SchemaRegistry} from "eas-contracts/SchemaRegistry.sol";
import {TheGuildActivityToken} from "../src/TheGuildActivityToken.sol";
import {TheGuildBadgeRegistry} from "../src/TheGuildBadgeRegistry.sol";
import {TheGuildBadgeRanking} from "../src/TheGuildBadgeRanking.sol";
import {EASUtils} from "./utils/EASUtils.s.sol";
import {console} from "forge-std/console.sol";

contract FullDeploymentScript is Script {
    function run() public {
        EAS eas;
        bytes32 salt = bytes32("theguild_v_0.1.1");
        // EAS addresses per https://github.com/ethereum-attestation-service/eas-contracts deployments
        // Base mainnet (8453) and Base Goerli/Sepolia (84531/84532) use the canonical predeploy 0x...21
        // Optimism mainnet (10) and OP Sepolia (11155420) also use canonical 0x...21

        eas = EAS(EASUtils.getEASAddress(vm));

        vm.startBroadcast();

        // Deploy or attach to existing activity token via CREATE2
        TheGuildActivityToken activityToken = new TheGuildActivityToken{salt: salt}(eas);

        // Register TheGuild Schema
        string memory schema = "bytes32 badgeName, bytes justification";
        SchemaRegistry schemaRegistry = SchemaRegistry(
            EASUtils.getSchemaRegistryAddress(vm)
        );
        bytes32 schemaId = schemaRegistry.register(schema, activityToken, true);
        console.logString("Schema ID:");
        console.logBytes32(schemaId);

        // Deploy or attach to existing badge registry via CREATE2
        TheGuildBadgeRegistry badgeRegistry = new TheGuildBadgeRegistry{salt: salt}();

        // Create some badges
        badgeRegistry.createBadge(
            bytes32("Rust"),
            bytes32("Know how to code in Rust")
        );
        badgeRegistry.createBadge(
            bytes32("Solidity"),
            bytes32("Know how to code in Solidity")
        );
        badgeRegistry.createBadge(
            bytes32("TypeScript"),
            bytes32("Know how to code in TypeScript")
        );

        // Deploy or attach to existing badge ranking via CREATE2
        TheGuildBadgeRanking badgeRanking = new TheGuildBadgeRanking{salt: salt}(badgeRegistry);

        // Create some attestations
        AttestationRequestData memory data = AttestationRequestData({
            // TheGuild test account
            recipient: address(0x6cfD0753EC4da15Dcb418E11e921C0665c1d1cBf),
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
        eas.attest(request);
        vm.stopBroadcast();
    }
}
