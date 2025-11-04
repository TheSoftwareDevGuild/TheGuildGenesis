// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {ERC20} from "openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";
import {Ownable} from "openzeppelin-contracts/contracts/access/Ownable.sol";

/// @title TheGuildActivityToken (TGA)
/// @notice Pure ERC20 token; ownership is transferred to resolver to mint on attest.
contract TheGuildActivityToken is ERC20, Ownable {
    constructor(
        address initialOwner
    ) ERC20("TheGuildActivityToken", "TGA") Ownable(initialOwner) {}

    /// @notice Mint tokens to a recipient. Only owner can mint.
    function mint(address to, uint256 amount) external onlyOwner {
        _mint(to, amount);
    }
}
