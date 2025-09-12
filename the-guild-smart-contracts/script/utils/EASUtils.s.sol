// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Vm.sol";

library EASUtils {
    function getEASAddress(Vm vm) internal view returns (address) {
        // Base and Optimism chains use canonical predeploy
        if (
            block.chainid == 8453 ||
            block.chainid == 84531 ||
            block.chainid == 84532 ||
            block.chainid == 10 ||
            block.chainid == 11155420
        ) {
            return 0x4200000000000000000000000000000000000021;
        }
        // Arbitrum Sepolia
        if (block.chainid == 421614) {
            return 0x2521021fc8BF070473E1e1801D3c7B4aB701E1dE;
        }
        // Polygon Amoy
        if (block.chainid == 80002) {
            return 0xb101275a60d8bfb14529C421899aD7CA1Ae5B5Fc;
        }
        // Linea Goerli
        if (block.chainid == 59140) {
            return 0xaEF4103A04090071165F78D45D83A0C0782c2B2a;
        }
        //Mainnet
        if (block.chainid == 1) {
            return 0xA1207F3BBa224E2c9c3c6D5aF63D0eb1582Ce587;
        }
        //Sepolia
        if (block.chainid == 11155111) {
            return 0xC2679fBD37d54388Ce493F1DB75320D236e1815e;
        }
        //Arbitrum One
        if (block.chainid == 42161) {
            return 0xbD75f629A22Dc1ceD33dDA0b68c546A1c035c458;
        }
        //Polygon
        if (block.chainid == 137) {
            return 0x5E634ef5355f45A855d02D66eCD687b1502AF790;
        }
        // Fallback to env var for other networks
        address fallbackAddr = vm.envOr("EAS_ADDRESS", address(0));
        require(
            fallbackAddr != address(0),
            "EAS_ADDRESS not set for this network"
        );
        return fallbackAddr;
    }

    function getSchemaRegistryAddress(Vm vm) internal view returns (address) {
        // Base and Optimism chains use canonical predeploy for SchemaRegistry
        if (
            block.chainid == 8453 ||
            block.chainid == 84531 ||
            block.chainid == 84532 ||
            block.chainid == 10 ||
            block.chainid == 11155420
        ) {
            return 0x4200000000000000000000000000000000000020;
        }
        // Arbitrum Sepolia
        if (block.chainid == 421614) {
            return 0x45CB6Fa0870a8Af06796Ac15915619a0f22cd475;
        }
        // Polygon Amoy
        if (block.chainid == 80002) {
            return 0x23c5701A1BDa89C61d181BD79E5203c730708AE7;
        }
        // Linea Goerli
        if (block.chainid == 59140) {
            return 0x55D26f9ae0203EF95494AE4C170eD35f4Cf77797;
        }
        // Fallback to env var for other networks
        address fallbackAddr = vm.envOr("SCHEMA_REGISTRY_ADDRESS", address(0));
        require(
            fallbackAddr != address(0),
            "SCHEMA_REGISTRY_ADDRESS not set for this network"
        );
        return fallbackAddr;
    }
}
