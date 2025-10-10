// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import {ERC20} from "openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";
import {Ownable} from "openzeppelin-contracts/contracts/access/Ownable.sol";

/// @title TheGuildContributionToken (TGC)
/// @notice Simple ERC20 Ownable token used for rewarding contributions.
contract TheGuildContributionToken is ERC20, Ownable {
    /// @notice Emitted when tokens are minted with a reason reference
    event ContributionTokenMinted(address indexed recipient, uint256 amount, bytes32 reason);

    // Track distribution IDs to prevent double distributions
    mapping(bytes32 => bool) private distributionExecuted;

    constructor() ERC20("TheGuild Contribution Token", "TGC") Ownable(msg.sender) {}

    /// @notice Mint tokens to a recipient. Only owner can mint.
    function mint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }

    /// @notice Mint tokens and associate a bytes32 reason (e.g. ticket id).
    function mintWithReason(address to, uint256 amount, bytes32 reason) public onlyOwner {
        _mint(to, amount);
        emit ContributionTokenMinted(to, amount, reason);
    }

    /// @notice Batch mint with a distribution id to avoid double execution.
    /// @param distributionId Unique id for this distribution (e.g., keccak256 of csv file content or timestamp)
    /// @param recipients Array of recipients
    /// @param amounts Array of amounts
    /// @param reasons Array of bytes32 reasons (one per recipient)
    function batchMint(
        bytes32 distributionId,
        address[] calldata recipients,
        uint256[] calldata amounts,
        bytes32[] calldata reasons
    ) external onlyOwner {
        require(!distributionExecuted[distributionId], "TGC: distribution already executed");
        require(recipients.length == amounts.length, "TGC: arrays length mismatch");
        require(recipients.length == reasons.length, "TGC: recipients/reasons length mismatch");
        require(recipients.length > 0, "TGC: empty arrays");

        distributionExecuted[distributionId] = true;

        for (uint256 i = 0; i < recipients.length; i++) {
            require(recipients[i] != address(0), "TGC: cannot mint to zero address");
            // reuse mintWithReason which already emits the event
            mintWithReason(recipients[i], amounts[i], reasons[i]);
        }
    }

    /// @notice Check if a distribution id has been executed
    function isDistributionExecuted(bytes32 distributionId) external view returns (bool) {
        return distributionExecuted[distributionId];
    }
}
