// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {TheGuildContributionToken} from "../src/TheGuildContributionToken.sol";

/// @notice Deploys a new TGC implementation and upgrades an existing UUPS proxy to it.
/// @dev Requires PRIVATE_KEY to be the current owner of the proxy.
contract UpgradeTGCImplementation is Script {
    function run() public {
        uint256 pk = vm.envUint("PRIVATE_KEY");
        address proxyAddress = vm.envAddress("TGC_PROXY_ADDRESS");

        require(proxyAddress != address(0), "TGC_PROXY_ADDRESS not set");

        vm.startBroadcast(pk);

        console.log(
            "Deploying new TheGuildContributionToken implementation..."
        );
        TheGuildContributionToken newImplementation = new TheGuildContributionToken();
        console.log("New implementation address:", address(newImplementation));

        console.log("Upgrading proxy to new implementation...");
        // OZ UUPS v5 only exposes upgradeToAndCall; use it with empty data.
        TheGuildContributionToken(proxyAddress).upgradeToAndCall(
            address(newImplementation),
            bytes("")
        );

        console.log("=== TGC Upgrade Summary ===");
        console.log("Proxy (TGC) address:", proxyAddress);
        console.log("New implementation address:", address(newImplementation));

        vm.stopBroadcast();
    }
}
