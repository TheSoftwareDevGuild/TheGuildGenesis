// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

/// @title TheGuildBadgeRegistry
/// @notice Minimal registry of community-created badges. Anyone can create a badge.
///         Badge names are unique and cannot be duplicated.
contract TheGuildBadgeRegistry {
    /// @notice Representation of a badge.
    struct Badge {
        bytes32 name;
        bytes description;
        address creator;
    }

    /// @notice Emitted when a new badge is created.
    event BadgeCreated(
        bytes32 indexed name,
        bytes description,
        address indexed creator
    );

    // name => badge
    mapping(bytes32 => Badge) private nameToBadge;
    // track existence to prevent duplicates
    mapping(bytes32 => bool) private nameExists;

    // enumerate badge names
    bytes32[] private badgeNames;

    /// @notice Create a new badge with a unique name.
    /// @param name The unique badge name (bytes32).
    /// @param description The badge description (bytes32).
    function createBadge(bytes32 name, bytes calldata description) external {
        require(name != bytes32(0), "EMPTY_NAME");
        require(!nameExists[name], "DUPLICATE_NAME");

        Badge memory badge = Badge({
            name: name,
            description: description,
            creator: msg.sender
        });
        nameToBadge[name] = badge;
        nameExists[name] = true;
        badgeNames.push(name);

        emit BadgeCreated(name, description, msg.sender);
    }

    /// @notice Get a badge by its name.
    /// @dev Reverts if the badge does not exist.
    function getBadge(
        bytes32 name
    ) external view returns (bytes32, bytes memory, address) {
        require(nameExists[name], "NOT_FOUND");
        Badge memory b = nameToBadge[name];
        return (b.name, b.description, b.creator);
    }

    /// @notice Get whether a badge name exists.
    function exists(bytes32 name) external view returns (bool) {
        return nameExists[name];
    }

    /// @notice Total number of badges created.
    function totalBadges() external view returns (uint256) {
        return badgeNames.length;
    }

    /// @notice Get badge name at a specific index for enumeration.
    /// @dev Reverts if index is out of bounds.
    function badgeNameAt(uint256 index) external view returns (bytes32) {
        return badgeNames[index];
    }

    /// @notice Get a badge at a specific index for enumeration.
    /// @dev Reverts if index is out of bounds.
    function getBadgeAt(
        uint256 index
    ) external view returns (bytes32, bytes memory, address) {
        bytes32 name = badgeNames[index];
        Badge memory b = nameToBadge[name];
        return (b.name, b.description, b.creator);
    }
}
