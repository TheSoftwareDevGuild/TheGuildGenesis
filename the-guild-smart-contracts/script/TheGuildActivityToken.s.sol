// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Script} from "forge-std/Script.sol";
import {IEAS} from "eas-contracts/IEAS.sol";
import {TheGuildActivityToken} from "../src/TheGuildActivityToken.sol";
import {TheGuildAttestationResolver} from "../src/TheGuildAttestationResolver.sol";
import {EASUtils} from "./utils/EASUtils.s.sol";

contract TheGuildActivityTokenScript is Script {
    function run() public {
        address eas;
        // EAS addresses per https://github.com/ethereum-attestation-service/eas-contracts deployments
        // Base mainnet (8453) and Base Goerli/Sepolia (84531/84532) use the canonical predeploy 0x...21
        // Optimism mainnet (10) and OP Sepolia (11155420) also use canonical 0x...21

        eas = EASUtils.getEASAddress(vm);

        vm.startBroadcast();
        TheGuildActivityToken token = new TheGuildActivityToken();
        TheGuildAttestationResolver resolver = new TheGuildAttestationResolver(
            IEAS(eas),
            token
        );
        token.transferOwnership(address(resolver));
        vm.stopBroadcast();
    }
}
