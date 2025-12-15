// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import {ERC20Upgradeable} from "@openzeppelin/contracts-upgradeable/token/ERC20/ERC20Upgradeable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

/// @title TheGuildContributionToken (TGC)
/// @notice Upgradable ERC20 token used to reward contributions within The Guild ecosystem.
///         Deployer is the initial owner and sole minter by default.
contract TheGuildContributionToken is
    ERC20Upgradeable,
    OwnableUpgradeable,
    UUPSUpgradeable
{
    /// @notice Emitted when tokens are minted with an associated reason.
    /// @param to The recipient of the minted tokens.
    /// @param amount The amount of tokens minted.
    /// @param reason Arbitrary bytes describing why the user received contribution tokens.
    event MintedWithReason(address indexed to, uint256 amount, bytes reason);

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        // Prevent initializing the implementation contract directly
        _disableInitializers();
    }

    /// @notice Initialize the upgradable token (called via proxy).
    /// @dev This replaces the constructor for upgradeable contracts.
    function initialize() public initializer {
        __ERC20_init("The Guild Contribution Token", "TGC");
        __Ownable_init(msg.sender);
        __UUPSUpgradeable_init();
    }

    /// @notice Mint tokens to a recipient.
    /// @dev Only the owner (deployer/admin) can mint.
    function mint(address to, uint256 amount) external onlyOwner {
        _mint(to, amount);
    }

    /// @notice Mint tokens to a recipient with an associated reason.
    /// @param to Recipient address.
    /// @param amount Token amount to mint.
    /// @param reason Arbitrary bytes describing why the user received tokens.
    function mintWithReason(
        address to,
        uint256 amount,
        bytes calldata reason
    ) external onlyOwner {
        _mint(to, amount);
        emit MintedWithReason(to, amount, reason);
    }

    /// @notice Batch mint tokens to multiple recipients.
    /// @param recipients Array of recipient addresses.
    /// @param amounts Array of token amounts to mint.
    function batchMint(
        address[] calldata recipients,
        uint256[] calldata amounts
    ) external onlyOwner {
        uint256 length = recipients.length;
        require(length == amounts.length, "LENGTH_MISMATCH");

        for (uint256 i = 0; i < length; i++) {
            _mint(recipients[i], amounts[i]);
        }
    }

    /// @notice Batch mint tokens with reasons to multiple recipients.
    /// @param recipients Array of recipient addresses.
    /// @param amounts Array of token amounts to mint.
    /// @param reasons Array of reasons (bytes) for each mint.
    function batchMintWithReason(
        address[] calldata recipients,
        uint256[] calldata amounts,
        bytes[] calldata reasons
    ) external onlyOwner {
        uint256 length = recipients.length;
        require(length == amounts.length, "LENGTH_MISMATCH");
        require(length == reasons.length, "REASONS_LENGTH_MISMATCH");

        for (uint256 i = 0; i < length; i++) {
            _mint(recipients[i], amounts[i]);
            emit MintedWithReason(recipients[i], amounts[i], reasons[i]);
        }
    }

    /// @notice Override decimals to keep normal ERC20 18 decimals.
    function decimals() public pure override returns (uint8) {
        return 18;
    }

    /// @dev Authorize contract upgrades. Only the owner can upgrade.
    function _authorizeUpgrade(
        address newImplementation
    ) internal override onlyOwner {}
}
