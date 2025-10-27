// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {TheGuildBadgeRegistry} from "../src/TheGuildBadgeRegistry.sol";

contract TheGuildBadgeRegistryTest is Test {
    TheGuildBadgeRegistry private registry;

    function setUp() public {
        registry = new TheGuildBadgeRegistry();
    }

    function test_CreateBadge_SucceedsAndEmitsEvent() public {
        bytes32 name = bytes32("BADGE_ALPHA");
        bytes32 description = bytes32("First badge");

        vm.expectEmit(true, false, false, true);
        emit TheGuildBadgeRegistry.BadgeCreated(
            name,
            description,
            address(this)
        );

        registry.createBadge(name, description);

        (bytes32 rName, bytes32 rDesc, address creator, int256 voteScore) = registry.getBadge(
            name
        );
        assertEq(rName, name, "name mismatch");
        assertEq(rDesc, description, "description mismatch");
        assertEq(creator, address(this), "creator mismatch");
        assertEq(voteScore, 0, "initial vote score should be 0");
        assertTrue(registry.exists(name), "should exist");
        assertEq(registry.totalBadges(), 1);
        assertEq(registry.badgeNameAt(0), name);
    }

    function test_CreateBadge_RevertOnDuplicate() public {
        bytes32 name = bytes32("BADGE_DUP");
        registry.createBadge(name, bytes32("desc"));

        vm.expectRevert(bytes("DUPLICATE_NAME"));
        registry.createBadge(name, bytes32("another"));
    }

    function test_GetBadge_RevertOnMissing() public {
        vm.expectRevert(bytes("NOT_FOUND"));
        registry.getBadge(bytes32("MISSING"));
    }

    function test_Vote_Upvote_IncreasesScore() public {
        bytes32 name = bytes32("VOTE_TEST");
        registry.createBadge(name, bytes32("test"));

        registry.vote(name, true);

        (, , , int256 score) = registry.getBadge(name);
        assertEq(score, 1);
    }

    function test_Vote_Downvote_DecreasesScore() public {
        bytes32 name = bytes32("VOTE_TEST_DOWN");
        registry.createBadge(name, bytes32("test"));

        registry.vote(name, false);

        (, , , int256 score) = registry.getBadge(name);
        assertEq(score, -1);
    }

    function test_Vote_CannotVoteTwice() public {
        bytes32 name = bytes32("VOTE_TWICE");
        registry.createBadge(name, bytes32("test"));

        registry.vote(name, true);
        vm.expectRevert(bytes("ALREADY_VOTED"));
        registry.vote(name, true);
    }

    function test_AddPointers_Succeeds() public {
        bytes32 fromName = bytes32("FROM_BADGE");
        bytes32 toName1 = bytes32("TO_BADGE_1");
        bytes32 toName2 = bytes32("TO_BADGE_2");

        registry.createBadge(fromName, bytes32("from"));
        registry.createBadge(toName1, bytes32("to1"));
        registry.createBadge(toName2, bytes32("to2"));

        bytes32[] memory pointers = new bytes32[](2);
        pointers[0] = toName1;
        pointers[1] = toName2;

        registry.addPointers(fromName, pointers);

        bytes32[] memory retrieved = registry.getPointers(fromName);
        assertEq(retrieved.length, 2);
        assertEq(retrieved[0], toName1);
        assertEq(retrieved[1], toName2);
    }

    function test_AddPointers_OnlyCreator() public {
        bytes32 fromName = bytes32("FROM_BADGE_CREATOR");
        bytes32 toName = bytes32("TO_BADGE_CREATOR");

        registry.createBadge(fromName, bytes32("from"));
        registry.createBadge(toName, bytes32("to"));

        // Try to add pointers as different user
        vm.prank(address(1));
        bytes32[] memory pointers = new bytes32[](1);
        pointers[0] = toName;

        vm.expectRevert(bytes("ONLY_CREATOR_CAN_ADD_POINTERS"));
        registry.addPointers(fromName, pointers);
    }
}
