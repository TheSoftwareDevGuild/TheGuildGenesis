// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {TheGuildContributionToken} from "../src/TheGuildContributionToken.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

/// @notice Deploys the upgradable TGC token (implementation + ERC1967/UUPS proxy).
contract DeployTGC is Script {
    function run() public {
        uint256 pk = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(pk);

        console.log(
            "Deploying TheGuildContributionToken (TGC) implementation..."
        );
        TheGuildContributionToken implementation = new TheGuildContributionToken();

        console.log("Deploying ERC1967 proxy for TGC...");
        ERC1967Proxy proxy = new ERC1967Proxy(
            address(implementation),
            abi.encodeWithSelector(
                TheGuildContributionToken.initialize.selector
            )
        );

        console.log("=== TGC Deployment Summary ===");
        console.log("Proxy (TGC) address:", address(proxy));
        console.log("Implementation address:", address(implementation));

        vm.stopBroadcast();
    }
}


