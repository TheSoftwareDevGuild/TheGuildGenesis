// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {TheGuildContributionToken} from "../src/TheGuildContributionToken.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

/// @notice Deploys the upgradable TGC token (implementation + ERC1967/UUPS proxy).
/// @dev Uses CREATE2 with a fixed salt for deterministic deployment, similar to `FullDeploymentScript`.
contract DeployTGC is Script {
    function run() public {
        // Use the same pattern as FullDeploymentScript for deterministic addresses.
        // Update this salt string when you intentionally want a new deployment address.
        bytes32 salt = bytes32("theguild_tgc_v_0.1.2");

        uint256 pk = vm.envUint("PRIVATE_KEY");
        address deployer = vm.addr(pk);
        vm.startBroadcast(pk);

        console.log(
            "Deploying TheGuildContributionToken (TGC) implementation..."
        );
        TheGuildContributionToken implementation = new TheGuildContributionToken{
                salt: salt
            }();

        console.log("Deploying ERC1967 proxy for TGC...");
        // Deploy proxy without initialization data so we can call initialize
        // directly from the deployer EOA. This ensures Ownable's owner is the
        // deployer (and not the CREATE2 factory address).
        ERC1967Proxy proxy = new ERC1967Proxy{salt: salt}(
            address(implementation),
            ""
        );

        console.log("Initializing TGC proxy...");
        TheGuildContributionToken(address(proxy)).initialize();

        console.log("=== TGC Deployment Summary ===");
        console.logBytes32(salt);
        console.log("Proxy (TGC) address:", address(proxy));
        console.log("Deployer (and initial owner):", deployer);
        console.log("Implementation address:", address(implementation));

        vm.stopBroadcast();
    }
}
