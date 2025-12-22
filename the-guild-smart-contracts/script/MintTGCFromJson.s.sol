// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import {Script, stdJson} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {TheGuildContributionToken} from "../src/TheGuildContributionToken.sol";

/// @notice Batch-mints TGC to recipients defined in a JSON file using mintWithReason.
/// @dev JSON format:
/// {
///   "mints": [
///     {
///       "recipient": "0x...",
///       "amount": "1000000000000000000",
///       "reason": "Some human readable text"
///     }
///   ]
/// }
contract MintTGCFromJson is Script {
    using stdJson for string;

    uint256 constant MAX_ENTRIES = 1000;

    struct MintData {
        address recipient;
        uint256 amount;
        bytes reason;
    }

    function run() public {
        bool isDryRun = vm.envOr("DRY_RUN", false);

        console.log("=== TheGuild TGC Mint-From-JSON Script ===");
        console.log("Dry run mode:", isDryRun ? "ENABLED" : "DISABLED");

        address proxyAddress = vm.envAddress("TGC_PROXY_ADDRESS");
        require(proxyAddress != address(0), "TGC_PROXY_ADDRESS not set");
        TheGuildContributionToken tgc = TheGuildContributionToken(proxyAddress);

        // Read JSON file
        string memory jsonPath = vm.envOr(
            "JSON_PATH",
            string("contribution-tokens-latest.json")
        );
        console.log("Reading JSON from:", jsonPath);

        string memory jsonData = vm.readFile(jsonPath);
        MintData[] memory mints = parseAndValidateJson(jsonData);

        console.log(
            string(
                abi.encodePacked(
                    "Parsed ",
                    vm.toString(mints.length),
                    " mint entries from JSON"
                )
            )
        );

        if (isDryRun) {
            for (uint256 i = 0; i < mints.length; i++) {
                console.log(
                    string(
                        abi.encodePacked(
                            "Mint ",
                            vm.toString(i + 1),
                            ": to=",
                            vm.toString(mints[i].recipient),
                            ", amount=",
                            vm.toString(mints[i].amount)
                        )
                    )
                );
            }
            console.log("Dry run completed successfully!");
            return;
        }

        executeMints(tgc, mints);
    }

    function parseAndValidateJson(
        string memory jsonData
    ) internal view returns (MintData[] memory) {
        console.log("Parsing JSON mints array...");

        MintData[] memory tempMints = new MintData[](MAX_ENTRIES);
        uint256 count = 0;

        for (uint256 i = 0; i < MAX_ENTRIES; i++) {
            string memory basePath = string(
                abi.encodePacked(".mints[", vm.toString(i), "]")
            );

            // Detect existence via recipient
            bytes memory recipientRaw = vm.parseJson(
                jsonData,
                string(abi.encodePacked(basePath, ".recipient"))
            );
            if (recipientRaw.length == 0) break;

            address recipient = abi.decode(recipientRaw, (address));

            uint256 amount = abi.decode(
                vm.parseJson(
                    jsonData,
                    string(abi.encodePacked(basePath, ".amount"))
                ),
                (uint256)
            );

            // Parse reason as a normal UTF-8 string and convert to bytes.
            // This makes the JSON human-readable for non-technical contributors.
            string memory reasonStr = jsonData.readString(
                string(abi.encodePacked(basePath, ".reason"))
            );
            bytes memory reason = bytes(reasonStr);

            tempMints[count] = MintData({
                recipient: recipient,
                amount: amount,
                reason: reason
            });

            count++;
        }

        MintData[] memory mints = new MintData[](count);
        for (uint256 i = 0; i < count; i++) {
            mints[i] = tempMints[i];
        }

        console.log("Found", mints.length, " mint entries in JSON");

        require(mints.length > 0, "JSON must contain at least 1 mint entry");
        require(
            mints.length <= MAX_ENTRIES,
            "Too many mint entries, max 1000 allowed"
        );

        for (uint256 i = 0; i < mints.length; i++) {
            require(
                mints[i].recipient != address(0),
                string(
                    abi.encodePacked(
                        "Mint ",
                        vm.toString(i),
                        ": invalid recipient address"
                    )
                )
            );
            require(
                mints[i].amount > 0,
                string(
                    abi.encodePacked(
                        "Mint ",
                        vm.toString(i),
                        ": amount must be > 0"
                    )
                )
            );
        }

        return mints;
    }

    function executeMints(
        TheGuildContributionToken tgc,
        MintData[] memory mints
    ) internal {
        uint256 pk = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(pk);

        console.log("Starting TGC minting...");

        for (uint256 i = 0; i < mints.length; i++) {
            MintData memory m = mints[i];
            tgc.mintWithReason(m.recipient, m.amount, m.reason);
            console.log(
                string(
                    abi.encodePacked(
                        "Minted ",
                        vm.toString(m.amount),
                        " TGC to ",
                        vm.toString(m.recipient)
                    )
                )
            );
        }

        vm.stopBroadcast();

        console.log("=== TGC Mint Summary ===");
        console.log("Proxy (TGC) address:", address(tgc));
        console.log("Total mints processed:", mints.length);
        console.log("Execution completed successfully!");
    }
}
