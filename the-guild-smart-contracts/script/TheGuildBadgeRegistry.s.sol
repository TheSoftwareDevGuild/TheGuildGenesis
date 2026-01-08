// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Script} from "forge-std/Script.sol";
import {TheGuildBadgeRegistry} from "../src/TheGuildBadgeRegistry.sol";

contract TheGuildBadgeRegistryScript is Script {
    function run() public {
        // Salt can be provided via env var or defaults to zero salt
        bytes32 salt = bytes32(vm.envOr("CREATE2_SALT", uint256(0)));

        vm.startBroadcast();
        TheGuildBadgeRegistry registry = new TheGuildBadgeRegistry{
            salt: salt
        }();
        registry.createBadge(
            bytes32("Rust"),
            bytes("Know how to code in Rust")
        );
        registry.createBadge(
            bytes32("Solidity"),
            bytes("Know how to code in Solidity")
        );
        registry.createBadge(
            bytes32("Python"),
            bytes("Know how to code in Python")
        );
        registry.createBadge(
            bytes32("JavaScript"),
            bytes("Know how to code in JavaScript")
        );
        registry.createBadge(
            bytes32("TypeScript"),
            bytes("Know how to code in TypeScript")
        );
        registry.createBadge(
            bytes32("React"),
            bytes("Know how to code in React")
        );
        registry.createBadge(
            bytes32("Next.js"),
            bytes("Know how to code in Next.js")
        );
        vm.stopBroadcast();
    }
}
