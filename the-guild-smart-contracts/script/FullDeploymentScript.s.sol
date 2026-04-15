// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import {Script} from "forge-std/Script.sol";
import {EAS} from "eas-contracts/EAS.sol";
import {AttestationRequestData, AttestationRequest} from "eas-contracts/IEAS.sol";
import {SchemaRegistry} from "eas-contracts/SchemaRegistry.sol";
import {TheGuildActivityToken} from "../src/TheGuildActivityToken.sol";
import {TheGuildAttestationResolver} from "../src/TheGuildAttestationResolver.sol";
import {TheGuildInternalResolver} from "../src/TheGuildInternalResolver.sol";
import {TheGuildBadgeRegistry} from "../src/TheGuildBadgeRegistry.sol";
import {TheGuildBadgeRanking} from "../src/TheGuildBadgeRanking.sol";
import {EASUtils} from "./utils/EASUtils.s.sol";
import {console} from "forge-std/console.sol";

contract FullDeploymentScript is Script {
    function run() public {
        bytes32 salt = bytes32("theguild_v_0.1.3");
        EAS eas = EAS(EASUtils.getEASAddress(vm));

        
        uint256 pk = vm.envUint("PRIVATE_KEY");
        address deployer = vm.addr(pk);
        
        vm.startBroadcast(pk);

       
        TheGuildActivityToken activityToken = new TheGuildActivityToken{
            salt: salt
        }(deployer);

        TheGuildBadgeRegistry badgeRegistry = new TheGuildBadgeRegistry{
            salt: salt
        }();

        
        TheGuildAttestationResolver resolver = new TheGuildAttestationResolver{
            salt: salt
        }(eas, activityToken, badgeRegistry);

      
        activityToken.transferOwnership(address(resolver));

       
        SchemaRegistry schemaRegistry = SchemaRegistry(
            EASUtils.getSchemaRegistryAddress(vm)
        );
        string memory schema = "bytes32 badgeName, bytes justification";
        bytes32 schemaId = schemaRegistry.register(schema, resolver, true);
        
        console.log("Badge Attestation Schema ID:", vm.toString(schemaId));

        // This deployment sets 'deployer' as the initial owner and authorized attester
        TheGuildInternalResolver internalResolver = new TheGuildInternalResolver{
            salt: salt
        }(eas, deployer);

        
        string memory skillSchema = "string skillDescription, bytes32[] linkedBadges";
        bytes32 skillSchemaId = schemaRegistry.register(
            skillSchema,
            internalResolver,
            true
        );
        
        console.log("Skill Badge Schema ID:", vm.toString(skillSchemaId));

        
        new TheGuildBadgeRanking{salt: salt}(badgeRegistry);

        
        AttestationRequestData memory data = AttestationRequestData({
            recipient: address(0x6cfD0753EC4da15Dcb418E11e921C0665c1d1cBf),
            expirationTime: 0,
            revocable: true,
            refUID: bytes32(0),
            data: abi.encode(bytes32("Rust"), bytes("Saw them coding in Rust")),
            value: 0
        });

        eas.attest(AttestationRequest({
            schema: schemaId,
            data: data
        }));

        vm.stopBroadcast();
    }
}