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
        bytes memory description = bytes("First badge");

        vm.expectEmit(true, false, false, true);
        emit TheGuildBadgeRegistry.BadgeCreated(
            name,
            description,
            address(this)
        );

        registry.createBadge(name, description);

        (bytes32 rName, bytes memory rDesc, address creator) = registry.getBadge(
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
        registry.createBadge(name, bytes("desc"));

        vm.expectRevert(bytes("DUPLICATE_NAME"));
        registry.createBadge(name, bytes("another"));
    }

    function test_GetBadge_RevertOnMissing() public {
        vm.expectRevert(bytes("NOT_FOUND"));
        registry.getBadge(bytes32("MISSING"));
    }

    function test_CreateBadge_RevertOnEmptyName() public {
        vm.expectRevert(bytes("EMPTY_NAME"));
        registry.createBadge(bytes32(0), bytes("desc"));
    }

    function test_GetBadgeAt_ByIndex() public {
        bytes32 a = bytes32("BADGE_A");
        bytes32 b = bytes32("BADGE_B");
        registry.createBadge(a, bytes("descA"));
        registry.createBadge(b, bytes("descB"));

        (bytes32 n0, bytes memory d0, address c0) = registry.getBadgeAt(0);
        assertEq(n0, a);
        assertEq(keccak256(d0), keccak256(bytes("descA")));
        assertEq(c0, address(this));

        (bytes32 n1, bytes memory d1, address c1) = registry.getBadgeAt(1);
        assertEq(n1, b);
        assertEq(keccak256(d1), keccak256(bytes("descB")));
        assertEq(c1, address(this));
    }
    function test_CreateBadge_AllowsLongDescription() public {
    bytes32 name = bytes32("BADGE_LONG");
    bytes memory longDesc = bytes(
        "Long descriptions matter. Short buffers do not. This test proves it."
    );

    registry.createBadge(name, longDesc);

    (, bytes memory rDesc,) = registry.getBadge(name);
    assertEq(keccak256(rDesc), keccak256(longDesc), "long description mismatch");
    }
}
