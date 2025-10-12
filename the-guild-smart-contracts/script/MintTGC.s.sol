// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import {Script, console} from "forge-std/Script.sol";
import {TheGuildContributionToken} from "../src/TheGuildContributionToken.sol";

/// @title TGC Mint Script
/// @notice Script to mint tokens on an already deployed TGC contract
contract MintTGC is Script {
    
    /// @notice Mint tokens to recipients from CSV data on existing contract
    /// @param tokenAddress The address of the deployed TGC contract
    /// @param recipients Array of recipient addresses
    /// @param amounts Array of amounts to mint (in tokens, will be converted to wei)
    function mintTokens(
        address tokenAddress,
        address[] memory recipients,
        uint256[] memory amounts
    ) public {
        require(tokenAddress != address(0), "Invalid token address");
        require(recipients.length == amounts.length, "Arrays length mismatch");
        require(recipients.length > 0, "Empty arrays");
        
        TheGuildContributionToken token = TheGuildContributionToken(tokenAddress);
        
        vm.startBroadcast();
        
        // Convert amounts to wei (multiply by 10^18)
        uint256[] memory amountsInWei = new uint256[](amounts.length);
        for (uint256 i = 0; i < amounts.length; i++) {
            amountsInWei[i] = amounts[i] * 10**18;
        }
        
        // Use batch mint for gas efficiency
        token.batchMint(recipients, amountsInWei);
        
        vm.stopBroadcast();
        
        console.log("Minting completed:");
        console.log("Contract Address:", tokenAddress);
        console.log("Recipients:", recipients.length);
        console.log("New Total Supply:", token.totalSupply());
        console.log("Remaining Supply:", token.remainingSupply());
    }
    
    /// @notice Mint tokens to a single recipient with reason
    /// @param tokenAddress The address of the deployed TGC contract
    /// @param recipient The recipient address
    /// @param amount Amount to mint (in tokens, will be converted to wei)
    /// @param reason The reason for minting (e.g., GitHub issue hash)
    function mintToSingleWithReason(
        address tokenAddress,
        address recipient,
        uint256 amount,
        bytes32 reason
    ) public {
        require(tokenAddress != address(0), "Invalid token address");
        require(recipient != address(0), "Invalid recipient address");
        require(amount > 0, "Amount must be greater than zero");
        require(reason != bytes32(0), "Reason cannot be empty");
        
        TheGuildContributionToken token = TheGuildContributionToken(tokenAddress);
        
        vm.startBroadcast();
        
        uint256 amountInWei = amount * 10**18;
        token.mintWithReason(recipient, amountInWei, reason);
        
        vm.stopBroadcast();
        
        console.log("Single mint with reason completed:");
        console.log("Contract Address:", tokenAddress);
        console.log("Recipient:", recipient);
        console.log("Amount:", amount, "TGC");
        console.log("Reason:", vm.toString(reason));
        console.log("New Total Supply:", token.totalSupply());
    }
    
    /// @notice Mint tokens to recipients with reasons from CSV data
    /// @param tokenAddress The address of the deployed TGC contract
    /// @param recipients Array of recipient addresses
    /// @param amounts Array of amounts to mint (in tokens, will be converted to wei)
    /// @param reasons Array of reasons for each mint
    function mintTokensWithReasons(
        address tokenAddress,
        address[] memory recipients,
        uint256[] memory amounts,
        bytes32[] memory reasons
    ) public {
        require(tokenAddress != address(0), "Invalid token address");
        require(recipients.length == amounts.length, "Recipients/amounts length mismatch");
        require(recipients.length == reasons.length, "Recipients/reasons length mismatch");
        require(recipients.length > 0, "Empty arrays");
        
        TheGuildContributionToken token = TheGuildContributionToken(tokenAddress);
        
        vm.startBroadcast();
        
        // Convert amounts to wei (multiply by 10^18)
        uint256[] memory amountsInWei = new uint256[](amounts.length);
        for (uint256 i = 0; i < amounts.length; i++) {
            amountsInWei[i] = amounts[i] * 10**18;
        }
        
        // Use batch mint with reasons for gas efficiency
        token.batchMintWithReasons(recipients, amountsInWei, reasons);
        
        vm.stopBroadcast();
        
        console.log("Minting with reasons completed:");
        console.log("Contract Address:", tokenAddress);
        console.log("Recipients:", recipients.length);
        console.log("New Total Supply:", token.totalSupply());
        console.log("Remaining Supply:", token.remainingSupply());
    }
    
    /// @notice Main function for minting from CSV data with reasons
    function run() external {
        // Get the deployed contract address from environment variable or hardcode
        address tokenAddress = vm.envAddress("TGC_TOKEN_ADDRESS");
        
        // Example recipients, amounts, and reasons (replace with actual CSV data)
        address[] memory recipients = new address[](2);
        uint256[] memory amounts = new uint256[](2);
        bytes32[] memory reasons = new bytes32[](2);
        
        // Example data - replace with actual addresses, amounts, and reasons from CSV
        recipients[0] = 0x4567890123456789012345678901234567890123;
        recipients[1] = 0x5678901234567890123456789012345678901234;
        
        amounts[0] = 500;  // 500 TGC tokens
        amounts[1] = 1500; // 1500 TGC tokens
        
        // GitHub issue hashes or ticket IDs
        reasons[0] = keccak256("GitHub-Issue-123");
        reasons[1] = keccak256("GitHub-Issue-456");
        
        // Mint tokens to recipients with reasons
        mintTokensWithReasons(tokenAddress, recipients, amounts, reasons);
    }
}