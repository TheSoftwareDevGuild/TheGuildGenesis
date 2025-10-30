// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {TheGuildBadgeRegistry} from "./TheGuildBadgeRegistry.sol";

/// @title TheGuildBadgeRanking
/// @notice Allows voting on badge relevancy to filter spam and rank badges.
contract TheGuildBadgeRanking {
    TheGuildBadgeRegistry public badgeRegistry;

    /// @notice Emitted when a badge receives an upvote.
    event BadgeUpvoted(
        bytes32 indexed badgeName,
        address indexed voter
    );

    // badgeName => upvote count
    mapping(bytes32 => uint256) public upvotes;
    // badgeName => voter => hasVoted
    mapping(bytes32 => mapping(address => bool)) private hasVoted;

    constructor(TheGuildBadgeRegistry _badgeRegistry) {
        require(address(_badgeRegistry) != address(0), "INVALID_REGISTRY");
        badgeRegistry = _badgeRegistry;
    }

    /// @notice Upvote a badge for relevancy.
    /// @param badgeName The name of the badge to upvote.
    /// @dev Reverts if badge doesn't exist or address has already voted for this badge.
    function upvoteBadge(bytes32 badgeName) external {
        require(badgeRegistry.exists(badgeName), "BADGE_NOT_FOUND");
        require(!hasVoted[badgeName][msg.sender], "ALREADY_VOTED");

        hasVoted[badgeName][msg.sender] = true;
        upvotes[badgeName]++;

        emit BadgeUpvoted(badgeName, msg.sender);
    }

    /// @notice Check if an address has voted for a specific badge.
    /// @param badgeName The name of the badge.
    /// @param voter The address to check.
    function hasVotedForBadge(bytes32 badgeName, address voter) external view returns (bool) {
        return hasVoted[badgeName][voter];
    }

    /// @notice Get the upvote count for a badge.
    /// @param badgeName The name of the badge.
    function getUpvotes(bytes32 badgeName) external view returns (uint256) {
        return upvotes[badgeName];
    }
}
