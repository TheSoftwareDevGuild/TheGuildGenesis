// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import {Script, stdJson} from "forge-std/Script.sol";
import {TheGuildBadgeRegistry} from "../src/TheGuildBadgeRegistry.sol";
import {console} from "forge-std/console.sol";

contract CreateBadgesFromJson is Script {
    using stdJson for string;

    // Configuration constants
    uint256 constant MAX_BADGES = 1000; // Reasonable limit for processing

    struct BadgeData {
        bytes32 name;
        string description;
    }

    function run() public {
        bool isDryRun = vm.envOr("DRY_RUN", false);

        console.log("=== TheGuild Create Badges from JSON Script ===");
        console.log("Dry run mode:", isDryRun ? "ENABLED" : "DISABLED");

        // Get badge registry address
        address badgeRegistryAddress = vm.envOr(
            "BADGE_REGISTRY_ADDRESS",
            address(0)
        );
        require(
            badgeRegistryAddress != address(0),
            "BADGE_REGISTRY_ADDRESS must be set"
        );
        console.log("Badge Registry Address:", badgeRegistryAddress);

        TheGuildBadgeRegistry badgeRegistry = TheGuildBadgeRegistry(
            badgeRegistryAddress
        );

        // Read and validate JSON file
        string memory jsonPath = vm.envOr(
            "JSON_PATH",
            string("badges-latest.json")
        );
        console.log("Reading JSON from:", jsonPath);

        string memory jsonData = vm.readFile(jsonPath);
        BadgeData[] memory badges = parseAndValidateJson(jsonData);

        console.log(
            string(
                abi.encodePacked(
                    "Parsed ",
                    vm.toString(badges.length),
                    " badges from JSON"
                )
            )
        );

        if (isDryRun) {
            console.log(
                string(
                    abi.encodePacked(
                        "DRY RUN: Would create ",
                        vm.toString(badges.length),
                        " badges"
                    )
                )
            );
            for (uint256 i = 0; i < badges.length; i++) {
                console.log(
                    string(
                        abi.encodePacked(
                            "Badge ",
                            vm.toString(i + 1),
                            ": ",
                            vm.toString(badges[i].name),
                            " - ",
                            badges[i].description
                        )
                    )
                );
            }
            console.log("Dry run completed successfully!");
            return;
        }

        // Execute badge creation
        createBadges(badgeRegistry, badges);
    }

    function parseAndValidateJson(
        string memory jsonData
    ) internal view returns (BadgeData[] memory) {
        console.log("Parsing JSON badges array...");

        // Count badges - parse the full array to get length
        BadgeData[] memory tempBadges = new BadgeData[](MAX_BADGES);
        uint256 count = 0;

        // Parse badges until we run out
        for (uint256 i = 0; i < MAX_BADGES; i++) {
            string memory basePath = string(
                abi.encodePacked(".badges[", vm.toString(i), "]")
            );

            // Try to parse this badge - if it doesn't exist, parseJson will revert
            bytes memory testPath = vm.parseJson(
                jsonData,
                string(abi.encodePacked(basePath, ".name"))
            );
            if (testPath.length == 0) break; // No more badges

            // Parse name with proper bytes32 conversion
            string memory nameStr = abi.decode(
                vm.parseJson(
                    jsonData,
                    string(abi.encodePacked(basePath, ".name"))
                ),
                (string)
            );
            bytes32 name;
            bytes memory nameBytes = bytes(nameStr);
            require(nameBytes.length <= 32, "name too long");
            assembly {
                name := mload(add(nameBytes, 32))
            }

            // Parse description as string
            string memory description = abi.decode(
                vm.parseJson(
                    jsonData,
                    string(abi.encodePacked(basePath, ".description"))
                ),
                (string)
            );

            tempBadges[count] = BadgeData({
                name: name,
                description: description
            });

            count++;
        }

        // Copy to properly sized array
        BadgeData[] memory badges = new BadgeData[](count);
        for (uint256 i = 0; i < count; i++) {
            badges[i] = tempBadges[i];
        }

        console.log("Found", badges.length, "badges in JSON");

        require(badges.length > 0, "JSON must contain at least 1 badge");
        require(
            badges.length <= MAX_BADGES,
            "Too many badges, max 1000 allowed"
        );

        // Validate each badge
        for (uint256 i = 0; i < badges.length; i++) {
            require(
                badges[i].name != bytes32(0),
                string(
                    abi.encodePacked(
                        "Badge ",
                        vm.toString(i),
                        ": name cannot be empty"
                    )
                )
            );
        }

        return badges;
    }

    function createBadges(
        TheGuildBadgeRegistry badgeRegistry,
        BadgeData[] memory badges
    ) internal {
        uint256 pk = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(pk);

        console.log("Starting badge creation...");

        uint256 successCount = 0;
        uint256 skipCount = 0;

        for (uint256 i = 0; i < badges.length; i++) {
            BadgeData memory badge = badges[i];

            // Check if badge already exists
            bool exists = badgeRegistry.exists(badge.name);
            if (exists) {
                console.log(
                    string(
                        abi.encodePacked(
                            "Skipping badge ",
                            vm.toString(i + 1),
                            "/",
                            vm.toString(badges.length),
                            ": ",
                            vm.toString(badge.name),
                            " already exists"
                        )
                    )
                );
                skipCount++;
                continue;
            }

            try badgeRegistry.createBadge(badge.name, bytes(badge.description)) {
                console.log(
                    string(
                        abi.encodePacked(
                            "Created badge ",
                            vm.toString(i + 1),
                            "/",
                            vm.toString(badges.length),
                            ": ",
                            vm.toString(badge.name),
                            " - ",
                            badge.description
                        )
                    )
                );
                successCount++;
            } catch Error(string memory reason) {
                console.log(
                    string(
                        abi.encodePacked(
                            "Badge ",
                            vm.toString(i + 1),
                            " failed: ",
                            reason
                        )
                    )
                );
                revert(
                    string(
                        abi.encodePacked(
                            "Badge ",
                            vm.toString(i + 1),
                            " failed: ",
                            reason
                        )
                    )
                );
            } catch {
                console.log(
                    string(
                        abi.encodePacked(
                            "Badge ",
                            vm.toString(i + 1),
                            " failed with unknown error"
                        )
                    )
                );
                revert(
                    string(
                        abi.encodePacked(
                            "Badge ",
                            vm.toString(i + 1),
                            " failed with unknown error"
                        )
                    )
                );
            }
        }

        vm.stopBroadcast();

        console.log("=== Badge Creation Summary ===");
        console.log("Total badges processed:", badges.length);
        console.log("Successfully created:", successCount);
        console.log("Skipped (already exist):", skipCount);
        console.log("Execution completed successfully!");
    }
}
