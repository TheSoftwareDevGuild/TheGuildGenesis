// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Script} from "forge-std/Script.sol";
import {IEAS} from "eas-contracts/IEAS.sol";
import {SchemaRegistry} from "eas-contracts/SchemaRegistry.sol";
import {TheGuildInternalResolver} from "../src/TheGuildInternalResolver.sol";
import {EASUtils} from "./utils/EASUtils.s.sol";
import {console} from "forge-std/console.sol";

contract TheGuildInternalResolverScript is Script {
    function run() public {
        address eas;
        eas = EASUtils.getEASAddress(vm);

        uint256 pk = vm.envUint("PRIVATE_KEY");
        address deployer = vm.addr(pk);
        vm.startBroadcast(pk);

        TheGuildInternalResolver resolver = new TheGuildInternalResolver(
            IEAS(eas),
            deployer
        );

        // Register Skill Badge Schema
        SchemaRegistry schemaRegistry = SchemaRegistry(
            EASUtils.getSchemaRegistryAddress(vm)
        );
        string memory schema = "string skillDescription, bytes32[] linkedBadges";
        bytes32 schemaId = schemaRegistry.register(schema, resolver, true);

        console.logString("Internal Resolver deployed at:");
        console.logAddress(address(resolver));
        console.logString("Skill Badge Schema ID:");
        console.logBytes32(schemaId);

        vm.stopBroadcast();
    }
}
