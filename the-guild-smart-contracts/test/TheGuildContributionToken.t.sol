// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {TheGuildContributionToken} from "../src/TheGuildContributionToken.sol";

contract TheGuildContributionTokenTest is Test {
    TheGuildContributionToken token;
    address owner = address(0xABCD);
    address alice = address(0x1);

    function setUp() public {
        vm.prank(owner);
        token = new TheGuildContributionToken();
    }

    function testMintOnlyOwner() public {
        vm.prank(owner);
        token.mint(alice, 1000);
        assertEq(token.balanceOf(alice), 1000);
    }

    function testMintWithReasonEmits() public {
        bytes32 reason = bytes32("ticket_1");
        vm.prank(owner);
        vm.expectEmit(true, false, false, true);
        emitContributionMinted(alice, 500, reason);
        token.mintWithReason(alice, 500, reason);
        assertEq(token.balanceOf(alice), 500);
    }

    function testBatchMintAndDistributionId() public {
        address bob = address(0x2);
        address carol = address(0x3);

        address[] memory recipients = new address[](2);
        recipients[0] = alice;
        recipients[1] = bob;

        uint256[] memory amounts = new uint256[](2);
        amounts[0] = 100;
        amounts[1] = 200;

        bytes32[] memory reasons = new bytes32[](2);
        reasons[0] = bytes32("t1");
        reasons[1] = bytes32("t2");

        bytes32 distributionId = keccak256(abi.encodePacked(block.timestamp, address(this)));

        vm.prank(owner);
        token.batchMint(distributionId, recipients, amounts, reasons);

        assertEq(token.balanceOf(alice), 100);
        assertEq(token.balanceOf(bob), 200);
        assertTrue(token.isDistributionExecuted(distributionId));

        // Replaying should revert
        vm.prank(owner);
        vm.expectRevert("TGC: distribution already executed");
        token.batchMint(distributionId, recipients, amounts, reasons);
    }

    // helper to satisfy expectEmit signature
    event ContributionTokenMinted(address indexed recipient, uint256 amount, bytes32 reason);

    function emitContributionMinted(address recipient, uint256 amount, bytes32 reason) internal {
        emit ContributionTokenMinted(recipient, amount, reason);
    }
}
