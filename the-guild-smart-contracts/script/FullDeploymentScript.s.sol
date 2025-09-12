// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import {Script} from "forge-std/Script.sol";
import {EAS} from "eas-contracts/EAS.sol";
import {AttestationRequestData, AttestationRequest} from "eas-contracts/IEAS.sol";
import {SchemaRegistry} from "eas-contracts/SchemaRegistry.sol";
import {TheGuildActivityToken} from "../src/TheGuildActivityToken.sol";
import {TheGuildBadgeRegistry} from "../src/TheGuildBadgeRegistry.sol";
import {EASUtils} from "./utils/EASUtils.s.sol";
import {console} from "forge-std/console.sol";

contract FullDeploymentScript is Script {
    function run() public {
        EAS eas;
        bytes32 salt = bytes32("theguild2");
        // EAS addresses per https://github.com/ethereum-attestation-service/eas-contracts deployments
        // Base mainnet (8453) and Base Goerli/Sepolia (84531/84532) use the canonical predeploy 0x...21
        // Optimism mainnet (10) and OP Sepolia (11155420) also use canonical 0x...21

        eas = EAS(EASUtils.getEASAddress(vm));

        vm.startBroadcast();

        // Deploy or attach to existing activity token via CREATE2
        TheGuildActivityToken activityToken;
        try new TheGuildActivityToken{salt: salt}(eas) returns (
            TheGuildActivityToken deployed
        ) {
            activityToken = deployed;
        } catch {
            // If already deployed with same salt + initCode, attach to the predicted address
            activityToken = TheGuildActivityToken(
                payable(0x5a79Dd0F66E2C1203948dD49634E506b3D8723A0)
            );
        }

        // Register TheGuild Schema
        string memory schema = "bytes32 badgeName, bytes32 justification";
        SchemaRegistry schemaRegistry = SchemaRegistry(
            EASUtils.getSchemaRegistryAddress(vm)
        );
        bytes32 schemaId = schemaRegistry.register(schema, activityToken, true);
        console.logString("Schema ID:");
        console.logBytes32(schemaId);

        // Deploy or attach to existing badge registry via CREATE2
        TheGuildBadgeRegistry badgeRegistry;
        try new TheGuildBadgeRegistry{salt: salt}() returns (
            TheGuildBadgeRegistry deployed
        ) {
            badgeRegistry = deployed;

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
        } catch {
            // If already deployed with same salt + initCode, attach to the predicted address
            badgeRegistry = TheGuildBadgeRegistry(
                0x8baA0d5135D241bd22a9eB35915300aCfB286307
            );
        }

        // Create some attestations
        AttestationRequestData memory data = AttestationRequestData({
            recipient: address(0x1234),
            expirationTime: 0,
            revocable: true,
            refUID: bytes32(0),
            data: abi.encode(
                bytes32("Rust"),
                bytes32("Saw them coding in Rust")
            ),
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
