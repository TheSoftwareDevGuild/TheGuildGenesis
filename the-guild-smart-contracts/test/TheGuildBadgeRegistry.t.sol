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

        (bytes32 rName, bytes32 rDesc, address creator) = registry.getBadge(
            name
        );
        assertEq(rName, name, "name mismatch");
        assertEq(rDesc, description, "description mismatch");
        assertEq(creator, address(this), "creator mismatch");
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

    function test_CreateBadge_RevertOnEmptyName() public {
        vm.expectRevert(bytes("EMPTY_NAME"));
        registry.createBadge(bytes32(0), bytes32("desc"));
    }

    function test_GetBadgeAt_ByIndex() public {
        bytes32 a = bytes32("BADGE_A");
        bytes32 b = bytes32("BADGE_B");
        registry.createBadge(a, bytes32("descA"));
        registry.createBadge(b, bytes32("descB"));

        (bytes32 n0, bytes32 d0, address c0) = registry.getBadgeAt(0);
        assertEq(n0, a);
        assertEq(d0, bytes32("descA"));
        assertEq(c0, address(this));

        (bytes32 n1, bytes32 d1, address c1) = registry.getBadgeAt(1);
        assertEq(n1, b);
        assertEq(d1, bytes32("descB"));
        assertEq(c1, address(this));
    }
}
