// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {TheGuildContributionToken} from "../src/TheGuildContributionToken.sol";

contract TheGuildContributionTokenBatchTest is Test {
    TheGuildContributionToken token;
    address owner = address(0xABCD);
    address alice = address(0x1);
    address bob = address(0x2);

    function setUp() public {
        vm.prank(owner);
        token = new TheGuildContributionToken();
    }

    function testBatchMintSuccessAndDistributionGuard() public {
        address[] memory recipients = new address[](2);
        recipients[0] = alice;
        recipients[1] = bob;

        uint256[] memory amounts = new uint256[](2);
        amounts[0] = 100;
        amounts[1] = 200;

        bytes32[] memory reasons = new bytes32[](2);
        reasons[0] = bytes32("ticket1");
        reasons[1] = bytes32("ticket2");

        bytes32 distributionId = keccak256(abi.encodePacked("dist-1"));

        vm.prank(owner);
        token.batchMint(distributionId, recipients, amounts, reasons);

        assertEq(token.balanceOf(alice), 100);
        assertEq(token.balanceOf(bob), 200);
        assertTrue(token.isDistributionExecuted(distributionId));

        // Re-running should revert
        vm.prank(owner);
        vm.expectRevert(bytes("TGC: distribution already executed"));
        token.batchMint(distributionId, recipients, amounts, reasons);
    }

    function testBatchMintRevertsOnLengthMismatch() public {
        address[] memory recipients = new address[](1);
        recipients[0] = alice;

        uint256[] memory amounts = new uint256[](2);
        amounts[0] = 100;
        amounts[1] = 200;

        bytes32[] memory reasons = new bytes32[](1);
        reasons[0] = bytes32("ticket1");

        bytes32 distributionId = keccak256(abi.encodePacked("dist-2"));

        vm.prank(owner);
        vm.expectRevert(bytes("TGC: arrays length mismatch"));
        token.batchMint(distributionId, recipients, amounts, reasons);
    }
}
