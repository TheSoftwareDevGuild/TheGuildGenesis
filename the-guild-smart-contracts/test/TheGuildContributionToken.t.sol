// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

<<<<<<< HEAD
import {Test, console} from "forge-std/Test.sol";
import {TheGuildContributionToken} from "../src/TheGuildContributionToken.sol";

/// @title TheGuildContributionToken Tests
/// @notice Comprehensive test suite for TGC token contract
contract TheGuildContributionTokenTest is Test {
    TheGuildContributionToken public token;
    address public owner;
    address public user1;
    address public user2;
    address public user3;
    
    /// @notice Set up test environment
    function setUp() public {
        owner = makeAddr("owner");
        user1 = makeAddr("user1");
        user2 = makeAddr("user2");
        user3 = makeAddr("user3");
        
        vm.prank(owner);
        token = new TheGuildContributionToken(owner);
    }
    
    /// @notice Test contract deployment and initial state
    function test_Deployment() public view {
        assertEq(token.name(), "TheGuild Contribution Token");
        assertEq(token.symbol(), "TGC");
        assertEq(token.decimals(), 18);
        assertEq(token.totalSupply(), 0);
        assertEq(token.owner(), owner);
        assertEq(token.maxSupply(), 1_000_000_000 * 10**18);
        assertEq(token.remainingSupply(), 1_000_000_000 * 10**18);
    }
    
    /// @notice Test basic minting functionality
    function test_Mint() public {
        uint256 amount = 1000 * 10**18; // 1000 tokens
        
        vm.prank(owner);
        token.mint(user1, amount);
        
        assertEq(token.balanceOf(user1), amount);
        assertEq(token.totalSupply(), amount);
        assertEq(token.remainingSupply(), token.maxSupply() - amount);
    }
    
    /// @notice Test minting with reason functionality
    function test_MintWithReason() public {
        uint256 amount = 1000 * 10**18; // 1000 tokens
        bytes32 reason = keccak256("GitHub-Issue-123");
        
        vm.prank(owner);
        token.mintWithReason(user1, amount, reason);
        
        assertEq(token.balanceOf(user1), amount);
        assertEq(token.totalSupply(), amount);
        assertEq(token.remainingSupply(), token.maxSupply() - amount);
    }
    
    /// @notice Test minting with empty reason fails
    function test_MintWithReasonEmptyReasonFails() public {
        uint256 amount = 1000 * 10**18;
        bytes32 emptyReason = bytes32(0);
        
        vm.prank(owner);
        vm.expectRevert("TGC: reason cannot be empty");
        token.mintWithReason(user1, amount, emptyReason);
    }
    
    /// @notice Test that only owner can mint
    function test_OnlyOwnerCanMint() public {
        uint256 amount = 1000 * 10**18;
        
        vm.prank(user1);
        vm.expectRevert();
        token.mint(user2, amount);
    }
    
    /// @notice Test minting to zero address fails
    function test_MintToZeroAddressFails() public {
        uint256 amount = 1000 * 10**18;
        
        vm.prank(owner);
        vm.expectRevert("TGC: cannot mint to zero address");
        token.mint(address(0), amount);
    }
    
    /// @notice Test minting zero amount fails
    function test_MintZeroAmountFails() public {
        vm.prank(owner);
        vm.expectRevert("TGC: amount must be greater than zero");
        token.mint(user1, 0);
    }
    
    /// @notice Test batch minting functionality
    function test_BatchMint() public {
        address[] memory recipients = new address[](3);
        uint256[] memory amounts = new uint256[](3);
        
        recipients[0] = user1;
        recipients[1] = user2;
        recipients[2] = user3;
        
        amounts[0] = 1000 * 10**18;
        amounts[1] = 2000 * 10**18;
        amounts[2] = 500 * 10**18;
        
        vm.prank(owner);
        token.batchMint(recipients, amounts);
        
        assertEq(token.balanceOf(user1), amounts[0]);
        assertEq(token.balanceOf(user2), amounts[1]);
        assertEq(token.balanceOf(user3), amounts[2]);
        assertEq(token.totalSupply(), amounts[0] + amounts[1] + amounts[2]);
    }
    
    /// @notice Test batch minting with reasons functionality
    function test_BatchMintWithReasons() public {
        address[] memory recipients = new address[](3);
        uint256[] memory amounts = new uint256[](3);
        bytes32[] memory reasons = new bytes32[](3);
        
        recipients[0] = user1;
        recipients[1] = user2;
        recipients[2] = user3;
        
        amounts[0] = 1000 * 10**18;
        amounts[1] = 2000 * 10**18;
        amounts[2] = 500 * 10**18;
        
        reasons[0] = keccak256("GitHub-Issue-123");
        reasons[1] = keccak256("GitHub-Issue-456");
        reasons[2] = keccak256("GitHub-Issue-789");
        
        vm.prank(owner);
        token.batchMintWithReasons(recipients, amounts, reasons);
        
        assertEq(token.balanceOf(user1), amounts[0]);
        assertEq(token.balanceOf(user2), amounts[1]);
        assertEq(token.balanceOf(user3), amounts[2]);
        assertEq(token.totalSupply(), amounts[0] + amounts[1] + amounts[2]);
    }
    
    /// @notice Test batch mint with reasons array length mismatch fails
    function test_BatchMintWithReasonsMismatchedArraysFails() public {
        address[] memory recipients = new address[](2);
        uint256[] memory amounts = new uint256[](2);
        bytes32[] memory reasons = new bytes32[](3); // Wrong length
        
        recipients[0] = user1;
        recipients[1] = user2;
        
        amounts[0] = 1000 * 10**18;
        amounts[1] = 2000 * 10**18;
        
        reasons[0] = keccak256("GitHub-Issue-123");
        reasons[1] = keccak256("GitHub-Issue-456");
        reasons[2] = keccak256("GitHub-Issue-789");
        
        vm.prank(owner);
        vm.expectRevert("TGC: recipients/reasons length mismatch");
        token.batchMintWithReasons(recipients, amounts, reasons);
    }
    
    /// @notice Test batch mint with empty reason fails
    function test_BatchMintWithReasonsEmptyReasonFails() public {
        address[] memory recipients = new address[](2);
        uint256[] memory amounts = new uint256[](2);
        bytes32[] memory reasons = new bytes32[](2);
        
        recipients[0] = user1;
        recipients[1] = user2;
        
        amounts[0] = 1000 * 10**18;
        amounts[1] = 2000 * 10**18;
        
        reasons[0] = keccak256("GitHub-Issue-123");
        reasons[1] = bytes32(0); // Empty reason
        
        vm.prank(owner);
        vm.expectRevert("TGC: reason cannot be empty");
        token.batchMintWithReasons(recipients, amounts, reasons);
    }
    
    /// @notice Test batch mint with mismatched arrays fails
    function test_BatchMintMismatchedArraysFails() public {
        address[] memory recipients = new address[](2);
        uint256[] memory amounts = new uint256[](3);
        
        recipients[0] = user1;
        recipients[1] = user2;
        
        amounts[0] = 1000 * 10**18;
        amounts[1] = 2000 * 10**18;
        amounts[2] = 500 * 10**18;
        
        vm.prank(owner);
        vm.expectRevert("TGC: arrays length mismatch");
        token.batchMint(recipients, amounts);
    }
    
    /// @notice Test batch mint with empty arrays fails
    function test_BatchMintEmptyArraysFails() public {
        address[] memory recipients = new address[](0);
        uint256[] memory amounts = new uint256[](0);
        
        vm.prank(owner);
        vm.expectRevert("TGC: empty arrays");
        token.batchMint(recipients, amounts);
    }
    
    /// @notice Test batch mint with zero address fails
    function test_BatchMintZeroAddressFails() public {
        address[] memory recipients = new address[](2);
        uint256[] memory amounts = new uint256[](2);
        
        recipients[0] = user1;
        recipients[1] = address(0);
        
        amounts[0] = 1000 * 10**18;
        amounts[1] = 2000 * 10**18;
        
        vm.prank(owner);
        vm.expectRevert("TGC: cannot mint to zero address");
        token.batchMint(recipients, amounts);
    }
    
    /// @notice Test batch mint with zero amount fails
    function test_BatchMintZeroAmountFails() public {
        address[] memory recipients = new address[](2);
        uint256[] memory amounts = new uint256[](2);
        
        recipients[0] = user1;
        recipients[1] = user2;
        
        amounts[0] = 1000 * 10**18;
        amounts[1] = 0;
        
        vm.prank(owner);
        vm.expectRevert("TGC: amount must be greater than zero");
        token.batchMint(recipients, amounts);
    }
    
    /// @notice Test max supply enforcement
    function test_MaxSupplyEnforcement() public {
        uint256 maxSupply = token.maxSupply();
        
        vm.prank(owner);
        vm.expectRevert("TGC: would exceed max supply");
        token.mint(user1, maxSupply + 1);
    }
    
    /// @notice Test max supply enforcement in batch mint
    function test_BatchMintMaxSupplyEnforcement() public {
        uint256 maxSupply = token.maxSupply();
        
        address[] memory recipients = new address[](2);
        uint256[] memory amounts = new uint256[](2);
        
        recipients[0] = user1;
        recipients[1] = user2;
        
        amounts[0] = maxSupply / 2;
        amounts[1] = (maxSupply / 2) + 1;
        
        vm.prank(owner);
        vm.expectRevert("TGC: would exceed max supply");
        token.batchMint(recipients, amounts);
    }
    
    /// @notice Test minting up to max supply works
    function test_MintToMaxSupply() public {
        uint256 maxSupply = token.maxSupply();
        
        vm.prank(owner);
        token.mint(user1, maxSupply);
        
        assertEq(token.balanceOf(user1), maxSupply);
        assertEq(token.totalSupply(), maxSupply);
        assertEq(token.remainingSupply(), 0);
    }
    
    /// @notice Test standard ERC20 transfer functionality
    function test_Transfer() public {
        uint256 amount = 1000 * 10**18;
        
        // Mint tokens to user1
        vm.prank(owner);
        token.mint(user1, amount);
        
        // Transfer from user1 to user2
        uint256 transferAmount = 300 * 10**18;
        vm.prank(user1);
        bool success = token.transfer(user2, transferAmount);
        
        assertTrue(success);
        assertEq(token.balanceOf(user1), amount - transferAmount);
        assertEq(token.balanceOf(user2), transferAmount);
    }
    
    /// @notice Test allowance and transferFrom functionality
    function test_Allowance() public {
        uint256 amount = 1000 * 10**18;
        uint256 allowanceAmount = 300 * 10**18;
        
        // Mint tokens to user1
        vm.prank(owner);
        token.mint(user1, amount);
        
        // User1 approves user2 to spend tokens
        vm.prank(user1);
        token.approve(user2, allowanceAmount);
        
        assertEq(token.allowance(user1, user2), allowanceAmount);
        
        // User2 transfers from user1 to user3
        uint256 transferAmount = 200 * 10**18;
        vm.prank(user2);
        bool success = token.transferFrom(user1, user3, transferAmount);
        
        assertTrue(success);
        assertEq(token.balanceOf(user1), amount - transferAmount);
        assertEq(token.balanceOf(user3), transferAmount);
        assertEq(token.allowance(user1, user2), allowanceAmount - transferAmount);
    }
    
    /// @notice Test ownership transfer
    function test_OwnershipTransfer() public {
        vm.prank(owner);
        token.transferOwnership(user1);
        
        assertEq(token.owner(), user1);
        
        // Old owner can't mint anymore
        vm.prank(owner);
        vm.expectRevert();
        token.mint(user2, 1000 * 10**18);
        
        // New owner can mint
        vm.prank(user1);
        token.mint(user2, 1000 * 10**18);
        assertEq(token.balanceOf(user2), 1000 * 10**18);
    }
    
    /// @notice Test renouncing ownership
    function test_RenounceOwnership() public {
        vm.prank(owner);
        token.renounceOwnership();
        
        assertEq(token.owner(), address(0));
        
        // No one can mint after renouncing
        vm.prank(owner);
        vm.expectRevert();
        token.mint(user1, 1000 * 10**18);
    }
    
    /// @notice Test events are emitted correctly
    function test_Events() public {
        uint256 amount = 1000 * 10**18;
        bytes32 reason = keccak256("GitHub-Issue-123");
        
        // Test Mint event
        vm.expectEmit(true, false, false, true);
        emit TheGuildContributionToken.Mint(user1, amount);
        
        vm.prank(owner);
        token.mint(user1, amount);
        
        // Test ContributionTokenMinted event
        vm.expectEmit(true, false, true, true);
        emit TheGuildContributionToken.ContributionTokenMinted(user2, amount, reason);
        
        vm.prank(owner);
        token.mintWithReason(user2, amount, reason);
        
        // Test BatchMint event
        address[] memory recipients = new address[](2);
        uint256[] memory amounts = new uint256[](2);
        
        recipients[0] = user1;
        recipients[1] = user3;
        amounts[0] = 500 * 10**18;
        amounts[1] = 750 * 10**18;
        
        uint256 totalAmount = amounts[0] + amounts[1];
        
        vm.expectEmit(true, false, false, true);
        emit TheGuildContributionToken.BatchMint(owner, totalAmount, 2);
        
        vm.prank(owner);
        token.batchMint(recipients, amounts);
    }
    
    /// @notice Fuzz test minting various amounts
    function testFuzz_Mint(uint256 amount) public {
        vm.assume(amount > 0 && amount <= token.maxSupply());
        
        vm.prank(owner);
        token.mint(user1, amount);
        
        assertEq(token.balanceOf(user1), amount);
        assertEq(token.totalSupply(), amount);
    }
    
    /// @notice Test gas consumption for batch operations
    function test_GasConsumption() public {
        uint256 batchSize = 10;
        address[] memory recipients = new address[](batchSize);
        uint256[] memory amounts = new uint256[](batchSize);
        
        for (uint256 i = 0; i < batchSize; i++) {
            recipients[i] = makeAddr(string(abi.encodePacked("user", i)));
            amounts[i] = (i + 1) * 100 * 10**18;
        }
        
        uint256 gasBefore = gasleft();
        
        vm.prank(owner);
        token.batchMint(recipients, amounts);
        
        uint256 gasUsed = gasBefore - gasleft();
        console.log("Gas used for batch mint of", batchSize, "recipients:", gasUsed);
        
        // Ensure gas usage is reasonable (adjust threshold as needed)
        assertLt(gasUsed, 500000); // Less than 500k gas
    }
}
=======
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
>>>>>>> 9dff903106d20ed0497926497613df5111737be9
