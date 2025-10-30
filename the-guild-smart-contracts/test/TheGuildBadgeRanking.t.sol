// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {TheGuildBadgeRegistry} from "../src/TheGuildBadgeRegistry.sol";
import {TheGuildBadgeRanking} from "../src/TheGuildBadgeRanking.sol";

contract TheGuildBadgeRankingTest is Test {
    TheGuildBadgeRegistry private registry;
    TheGuildBadgeRanking private ranking;

    address private voter1 = address(0x1);
    address private voter2 = address(0x2);
    bytes32 private badgeName = bytes32("TEST_BADGE");

    function setUp() public {
        registry = new TheGuildBadgeRegistry();
        ranking = new TheGuildBadgeRanking(registry);
        
        // Create a badge for testing
        registry.createBadge(badgeName, bytes32("A test badge"));
    }

    function test_UpvoteBadge_SucceedsAndEmitsEvent() public {
        vm.prank(voter1);
        vm.expectEmit(true, true, false, false);
        emit TheGuildBadgeRanking.BadgeUpvoted(badgeName, voter1);
        
        ranking.upvoteBadge(badgeName);
        assertEq(ranking.getUpvotes(badgeName), 1, "upvotes should be 1");
    }

    function test_UpvoteBadge_MultipleVoters() public {
        vm.prank(voter1);
        ranking.upvoteBadge(badgeName);
        
        vm.prank(voter2);
        ranking.upvoteBadge(badgeName);
        
        assertEq(ranking.getUpvotes(badgeName), 2, "upvotes should be 2");
    }

    function test_UpvoteBadge_RevertOnDuplicateVote() public {
        vm.prank(voter1);
        ranking.upvoteBadge(badgeName);
        
        vm.expectRevert(bytes("ALREADY_VOTED"));
        vm.prank(voter1);
        ranking.upvoteBadge(badgeName);
    }

    function test_UpvoteBadge_RevertOnNonexistentBadge() public {
        bytes32 nonexistentBadge = bytes32("NONEXISTENT");
        
        vm.prank(voter1);
        vm.expectRevert(bytes("BADGE_NOT_FOUND"));
        ranking.upvoteBadge(nonexistentBadge);
    }

    function test_HasVotedForBadge_ReturnsTrueAfterVoting() public {
        vm.prank(voter1);
        ranking.upvoteBadge(badgeName);
        
        assertTrue(ranking.hasVotedForBadge(badgeName, voter1), "should have voted");
        assertFalse(ranking.hasVotedForBadge(badgeName, voter2), "should not have voted");
    }

    function test_Constructor_RevertOnInvalidRegistry() public {
        vm.expectRevert(bytes("INVALID_REGISTRY"));
        new TheGuildBadgeRanking(TheGuildBadgeRegistry(address(0)));
    }
}
