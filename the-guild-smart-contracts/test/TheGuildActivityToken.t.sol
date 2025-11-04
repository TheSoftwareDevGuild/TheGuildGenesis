// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {TheGuildActivityToken} from "../src/TheGuildActivityToken.sol";

contract TheGuildActivityTokenTest is Test {
    TheGuildActivityToken private token;

    address private owner = address(this);
    address private user = address(0xBEEF);

    function setUp() public {
        token = new TheGuildActivityToken();
    }

    function test_Metadata() public view {
        assertEq(token.name(), "TheGuildActivityToken");
        assertEq(token.symbol(), "TGA");
        assertEq(token.decimals(), 18);
    }

    function test_OwnerIsDeployer() public view {
        assertEq(token.owner(), owner);
    }

    function test_MintByOwner() public {
        token.mint(user, 1e18);
        assertEq(token.balanceOf(user), 1e18);
        assertEq(token.totalSupply(), 1e18);
    }

    function test_RevertMintIfNotOwner() public {
        vm.prank(user);
        vm.expectRevert();
        token.mint(user, 1e18);
    }
}
