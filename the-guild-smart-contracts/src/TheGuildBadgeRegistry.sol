// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

/// @title TheGuildBadgeRegistry
/// @notice Minimal registry of community-created badges. Anyone can create a badge.
///         Badge names are unique and cannot be duplicated.
contract TheGuildBadgeRegistry {
    /// @notice Representation of a badge.
    struct Badge {
        bytes32 name;
        bytes32 description;
        address creator;
        bytes32[] pointers; // badges this badge points to (hierarchy)
        mapping(address => bool) hasVoted; // prevent double voting
        int256 voteScore; // net upvotes - downvotes
    }

    /// @notice Emitted when a new badge is created.
    event BadgeCreated(
        bytes32 indexed name,
        bytes32 description,
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
    function createBadge(bytes32 name, bytes32 description) external {
        require(name != bytes32(0), "EMPTY_NAME");
        require(!nameExists[name], "DUPLICATE_NAME");

        Badge storage badge = nameToBadge[name];
        badge.name = name;
        badge.description = description;
        badge.creator = msg.sender;
        badge.voteScore = 0;
        nameExists[name] = true;
        badgeNames.push(name);

        emit BadgeCreated(name, description, msg.sender);
    }

    /// @notice Add pointers from this badge to other badges (hierarchy).
    /// @param fromBadge The badge that points to others.
    /// @param toBadges Array of badge names this badge points to.
    function addPointers(bytes32 fromBadge, bytes32[] calldata toBadges) external {
        require(nameExists[fromBadge], "FROM_BADGE_NOT_FOUND");
        Badge storage badge = nameToBadge[fromBadge];
        require(badge.creator == msg.sender, "ONLY_CREATOR_CAN_ADD_POINTERS");
        
        for (uint256 i = 0; i < toBadges.length; i++) {
            require(nameExists[toBadges[i]], "TO_BADGE_NOT_FOUND");
            badge.pointers.push(toBadges[i]);
        }
    }
    /// @param badgeName The badge to vote on.
    /// @param isUpvote True for upvote, false for downvote.
    function vote(bytes32 badgeName, bool isUpvote) external {
        require(nameExists[badgeName], "BADGE_NOT_FOUND");
        Badge storage badge = nameToBadge[badgeName];
        require(!badge.hasVoted[msg.sender], "ALREADY_VOTED");
        
        badge.hasVoted[msg.sender] = true;
        if (isUpvote) {
            badge.voteScore += 1;
        } else {
            badge.voteScore -= 1;
        }
    }

    /// @notice Get pointers for a badge.
    /// @param name The badge name.
    /// @return Array of badge names this badge points to.
    function getPointers(bytes32 name) external view returns (bytes32[] memory) {
        require(nameExists[name], "NOT_FOUND");
        return nameToBadge[name].pointers;
    }

    /// @notice Get vote score for a badge.
    /// @param name The badge name.
    /// @return The net vote score (upvotes - downvotes).
    function getVoteScore(bytes32 name) external view returns (int256) {
        require(nameExists[name], "NOT_FOUND");
        return nameToBadge[name].voteScore;
    }

    /// @notice Get whether a badge name exists.
    function exists(bytes32 name) external view returns (bool) {
        return nameExists[name];
    }

    /// @notice Get a badge by its name.
    /// @dev Reverts if the badge does not exist.
    function getBadge(
        bytes32 name
    ) external view returns (bytes32, bytes32, address, int256) {
        require(nameExists[name], "NOT_FOUND");
        Badge storage b = nameToBadge[name];
        return (b.name, b.description, b.creator, b.voteScore);
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
    ) external view returns (bytes32, bytes32, address, int256) {
        bytes32 name = badgeNames[index];
        Badge storage b = nameToBadge[name];
        return (b.name, b.description, b.creator, b.voteScore);
    }
}
