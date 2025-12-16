// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {TheGuildContributionToken} from "../src/TheGuildContributionToken.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract TheGuildContributionTokenTest is Test {
    TheGuildContributionToken private token;

    address private owner = address(this);
    address private user1 = address(0xBEEF);
    address private user2 = address(0xCAFE);

    function setUp() public {
        // Deploy implementation (cannot be initialized due to _disableInitializers)
        TheGuildContributionToken implementation = new TheGuildContributionToken();

        // Deploy proxy and initialize it
        ERC1967Proxy proxy = new ERC1967Proxy(
            address(implementation),
            abi.encodeWithSelector(
                TheGuildContributionToken.initialize.selector
            )
        );

        token = TheGuildContributionToken(address(proxy));
    }

    function test_Metadata() public view {
        assertEq(token.name(), "The Guild Contribution Token", "name mismatch");
        assertEq(token.symbol(), "TGC", "symbol mismatch");
        assertEq(token.decimals(), 18, "decimals mismatch");
    }

    function test_OwnerIsInitializer() public view {
        assertEq(token.owner(), owner, "owner should be test contract");
    }

    function test_MintByOwner() public {
        token.mint(user1, 1e18);

        assertEq(token.balanceOf(user1), 1e18, "balance mismatch");
        assertEq(token.totalSupply(), 1e18, "totalSupply mismatch");
    }

    function test_RevertMintIfNotOwner() public {
        vm.prank(user1);
        vm.expectRevert();
        token.mint(user1, 1e18);
    }

    function test_MintWithReason_EmitsEvent() public {
        bytes memory reason = bytes("contribution:issue-123");

        vm.expectEmit(true, false, false, true);
        emit TheGuildContributionToken.MintedWithReason(user1, 5e17, reason);

        token.mintWithReason(user1, 5e17, reason);

        assertEq(token.balanceOf(user1), 5e17, "balance mismatch");
        assertEq(token.totalSupply(), 5e17, "totalSupply mismatch");
    }

    function test_BatchMint() public {
        address[] memory recipients = new address[](2);
        recipients[0] = user1;
        recipients[1] = user2;

        uint256[] memory amounts = new uint256[](2);
        amounts[0] = 1e18;
        amounts[1] = 2e18;

        token.batchMint(recipients, amounts);

        assertEq(token.balanceOf(user1), 1e18, "user1 balance mismatch");
        assertEq(token.balanceOf(user2), 2e18, "user2 balance mismatch");
        assertEq(token.totalSupply(), 3e18, "totalSupply mismatch");
    }

    function test_BatchMintWithReason() public {
        address[] memory recipients = new address[](2);
        recipients[0] = user1;
        recipients[1] = user2;

        uint256[] memory amounts = new uint256[](2);
        amounts[0] = 1e18;
        amounts[1] = 2e18;

        bytes[] memory reasons = new bytes[](2);
        reasons[0] = bytes("reason-1");
        reasons[1] = bytes("reason-2");

        // Expect two MintedWithReason events
        vm.expectEmit(true, false, false, true);
        emit TheGuildContributionToken.MintedWithReason(
            recipients[0],
            amounts[0],
            reasons[0]
        );
        vm.expectEmit(true, false, false, true);
        emit TheGuildContributionToken.MintedWithReason(
            recipients[1],
            amounts[1],
            reasons[1]
        );

        token.batchMintWithReason(recipients, amounts, reasons);

        assertEq(token.balanceOf(user1), 1e18, "user1 balance mismatch");
        assertEq(token.balanceOf(user2), 2e18, "user2 balance mismatch");
        assertEq(token.totalSupply(), 3e18, "totalSupply mismatch");
    }
}
