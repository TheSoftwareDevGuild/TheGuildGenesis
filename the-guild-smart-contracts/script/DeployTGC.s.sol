// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import {Script, console} from "forge-std/Script.sol";
import {TheGuildContributionToken} from "../src/TheGuildContributionToken.sol";

/// @title TGC Deployment Script
/// @notice Script to deploy TheGuildContributionToken and optionally mint initial balances
contract DeployTGC is Script {
    
    /// @notice Deploy the TGC token contract
    /// @param initialOwner The address that will own the contract
    /// @return token The deployed TGC token contract
    function deployToken(address initialOwner) public returns (TheGuildContributionToken) {
        vm.startBroadcast();
        
        TheGuildContributionToken token = new TheGuildContributionToken(initialOwner);
        
        vm.stopBroadcast();
        
        console.log("TheGuildContributionToken deployed at:", address(token));
        console.log("Owner:", token.owner());
        console.log("Name:", token.name());
        console.log("Symbol:", token.symbol());
        console.log("Decimals:", token.decimals());
        console.log("Max Supply:", token.maxSupply());
        
        return token;
    }
    
    /// @notice Deploy token and mint initial balances from CSV data
    /// @param initialOwner The address that will own the contract
    /// @param recipients Array of recipient addresses
    /// @param amounts Array of amounts to mint (in tokens, will be converted to wei)
    /// @return token The deployed TGC token contract
    function deployTokenWithInitialMint(
        address initialOwner,
        address[] memory recipients,
        uint256[] memory amounts
    ) public returns (TheGuildContributionToken) {
        TheGuildContributionToken token = deployToken(initialOwner);
        
        if (recipients.length > 0) {
            vm.startBroadcast();
            
            // Convert amounts to wei (multiply by 10^18)
            uint256[] memory amountsInWei = new uint256[](amounts.length);
            for (uint256 i = 0; i < amounts.length; i++) {
                amountsInWei[i] = amounts[i] * 10**18;
            }
            
            token.batchMint(recipients, amountsInWei);
            
            vm.stopBroadcast();
            
            console.log("Initial minting completed:");
            console.log("Recipients:", recipients.length);
            console.log("Total Supply:", token.totalSupply());
        }
        
        return token;
    }
    
    /// @notice Main deployment function
    function run() external {
        // Get deployer address from private key
        address deployer = vm.addr(vm.envUint("PRIVATE_KEY"));
        
        // Example recipients and amounts (replace with actual CSV data)
        address[] memory recipients = new address[](3);
        uint256[] memory amounts = new uint256[](3);
        
        // Example data - replace with actual addresses and amounts
        recipients[0] = 0x1234567890123456789012345678901234567890;
        recipients[1] = 0x2345678901234567890123456789012345678901;
        recipients[2] = 0x3456789012345678901234567890123456789012;
        
        amounts[0] = 1000; // 1000 TGC tokens
        amounts[1] = 2000; // 2000 TGC tokens
        amounts[2] = 500;  // 500 TGC tokens
        
        // Deploy with initial minting
        deployTokenWithInitialMint(deployer, recipients, amounts);
    }
}