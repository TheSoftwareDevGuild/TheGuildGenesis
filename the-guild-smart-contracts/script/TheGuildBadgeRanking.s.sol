// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Script} from "forge-std/Script.sol";
import {TheGuildBadgeRegistry} from "../src/TheGuildBadgeRegistry.sol";
import {TheGuildBadgeRanking} from "../src/TheGuildBadgeRanking.sol";

contract TheGuildBadgeRankingScript is Script {
    function run() public {
        // Badge registry address - can be provided via env var or defaults to Amoy dev address
        address badgeRegistryAddress = vm.envOr(
            "BADGE_REGISTRY_ADDRESS",
            address(0x8aC95734e778322684f1D318fb7633777baa8427) // Amoy dev registry
        );

        // Salt can be provided via env var or defaults to zero salt
        bytes32 salt = bytes32(vm.envOr("CREATE2_SALT", uint256(0)));

    vm.startBroadcast();
    // Pass the address (cast to TheGuildBadgeRegistry)
    new TheGuildBadgeRanking{salt: salt}(TheGuildBadgeRegistry(badgeRegistryAddress));
        vm.stopBroadcast();
    }
}