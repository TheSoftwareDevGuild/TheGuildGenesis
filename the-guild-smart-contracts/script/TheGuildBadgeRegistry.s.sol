// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Script} from "forge-std/Script.sol";
import {TheGuildBadgeRegistry} from "../src/TheGuildBadgeRegistry.sol";

contract TheGuildBadgeRegistryScript is Script {
    function run() public {
        vm.startBroadcast();
        new TheGuildBadgeRegistry();
        vm.stopBroadcast();
    }
}
