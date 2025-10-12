// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

/// @title TheGuildContributionToken (TGC)
/// @notice ERC20 token for rewarding contributions to The Guild
/// @dev Standard ERC20 with owner-controlled minting capabilities
contract TheGuildContributionToken is ERC20, Ownable {
    /// @notice Maximum total supply to prevent infinite inflation
    uint256 public constant MAX_SUPPLY = 1_000_000_000 * 10**18; // 1 billion tokens
    
    /// @notice Event emitted when tokens are minted to multiple recipients
    event BatchMint(address indexed owner, uint256 totalAmount, uint256 recipientCount);
    
    /// @notice Event emitted when tokens are minted to a single recipient
    event Mint(address indexed to, uint256 amount);
    
    /// @notice Event emitted when tokens are minted with a reason (e.g., GitHub ticket reference)
    event ContributionTokenMinted(address indexed recipient, uint256 amount, bytes32 indexed reason);

    /// @dev Constructor sets the token name, symbol, and initial owner
    /// @param initialOwner The address that will own the contract
    constructor(address initialOwner) 
        ERC20("TheGuild Contribution Token", "TGC") 
        Ownable(initialOwner) 
    {
        // Contract starts with zero supply - all tokens must be explicitly minted
    }

    /// @notice Mint tokens to a single recipient
    /// @dev Only the owner can mint tokens
    /// @param to The address to receive the tokens
    /// @param amount The amount of tokens to mint (in wei, including decimals)
    function mint(address to, uint256 amount) external onlyOwner {
        require(to != address(0), "TGC: cannot mint to zero address");
        require(amount > 0, "TGC: amount must be greater than zero");
        require(totalSupply() + amount <= MAX_SUPPLY, "TGC: would exceed max supply");
        
        _mint(to, amount);
        emit Mint(to, amount);
    }

    /// @notice Mint tokens to a recipient with a reason (e.g., GitHub ticket reference)
    /// @dev Only the owner can mint tokens. Use this for tracking specific contributions.
    /// @param to The address to receive the tokens
    /// @param amount The amount of tokens to mint (in wei, including decimals)
    /// @param reason The reason for minting (e.g., GitHub issue hash, ticket ID)
    function mintWithReason(address to, uint256 amount, bytes32 reason) external onlyOwner {
        require(to != address(0), "TGC: cannot mint to zero address");
        require(amount > 0, "TGC: amount must be greater than zero");
        require(reason != bytes32(0), "TGC: reason cannot be empty");
        require(totalSupply() + amount <= MAX_SUPPLY, "TGC: would exceed max supply");
        
        _mint(to, amount);
        emit ContributionTokenMinted(to, amount, reason);
    }

    /// @notice Mint tokens to multiple recipients
    /// @dev Only the owner can mint tokens. Gas-efficient batch minting.
    /// @param recipients Array of addresses to receive tokens
    /// @param amounts Array of amounts to mint to each recipient
    function batchMint(address[] calldata recipients, uint256[] calldata amounts) external onlyOwner {
        require(recipients.length == amounts.length, "TGC: arrays length mismatch");
        require(recipients.length > 0, "TGC: empty arrays");
        
        uint256 totalAmount = 0;
        
        for (uint256 i = 0; i < recipients.length; i++) {
            require(recipients[i] != address(0), "TGC: cannot mint to zero address");
            require(amounts[i] > 0, "TGC: amount must be greater than zero");
            
            totalAmount += amounts[i];
            _mint(recipients[i], amounts[i]);
        }
        
        require(totalSupply() <= MAX_SUPPLY, "TGC: would exceed max supply");
        
        emit BatchMint(msg.sender, totalAmount, recipients.length);
    }

    /// @notice Mint tokens to multiple recipients with reasons
    /// @dev Only the owner can mint tokens. Gas-efficient batch minting with contribution tracking.
    /// @param recipients Array of addresses to receive tokens
    /// @param amounts Array of amounts to mint to each recipient
    /// @param reasons Array of reasons for each mint (e.g., GitHub issue hashes)
    function batchMintWithReasons(
        address[] calldata recipients, 
        uint256[] calldata amounts, 
        bytes32[] calldata reasons
    ) external onlyOwner {
        require(recipients.length == amounts.length, "TGC: recipients/amounts length mismatch");
        require(recipients.length == reasons.length, "TGC: recipients/reasons length mismatch");
        require(recipients.length > 0, "TGC: empty arrays");
        
        uint256 totalAmount = 0;
        
        for (uint256 i = 0; i < recipients.length; i++) {
            require(recipients[i] != address(0), "TGC: cannot mint to zero address");
            require(amounts[i] > 0, "TGC: amount must be greater than zero");
            require(reasons[i] != bytes32(0), "TGC: reason cannot be empty");
            
            totalAmount += amounts[i];
            _mint(recipients[i], amounts[i]);
            emit ContributionTokenMinted(recipients[i], amounts[i], reasons[i]);
        }
        
        require(totalSupply() <= MAX_SUPPLY, "TGC: would exceed max supply");
        
        emit BatchMint(msg.sender, totalAmount, recipients.length);
    }

    /// @notice Get the maximum supply of tokens
    /// @return The maximum total supply
    function maxSupply() external pure returns (uint256) {
        return MAX_SUPPLY;
    }

    /// @notice Get the remaining mintable supply
    /// @return The amount of tokens that can still be minted
    function remainingSupply() external view returns (uint256) {
        return MAX_SUPPLY - totalSupply();
    }
}