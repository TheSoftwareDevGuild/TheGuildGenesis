// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import {Script, stdJson} from "forge-std/Script.sol";
import {EAS} from "eas-contracts/EAS.sol";
import {AttestationRequestData, MultiAttestationRequest} from "eas-contracts/IEAS.sol";
import {EASUtils} from "./utils/EASUtils.s.sol";
import {console} from "forge-std/console.sol";

contract EmitAttestationsJson is Script {
    using stdJson for string;

    // Configuration constants
    uint256 constant MAX_BATCH_SIZE = 50; // Limit batch size to prevent gas issues
    uint256 constant MAX_ATTESTATIONS = 1000; // Reasonable limit for processing

    struct AttestationData {
        address recipient;
        bytes32 badgeName;
        bytes justification;
    }

    function run() public {
        bool isDryRun = vm.envOr("DRY_RUN", false);

        console.log("=== TheGuild Batch Attestation Script ===");
        console.log("Dry run mode:", isDryRun ? "ENABLED" : "DISABLED");

        // Get EAS address for current network
        EAS eas = EAS(EASUtils.getEASAddress(vm));
        console.log("EAS Address:", address(eas));

        // Read and validate JSON file
        string memory jsonPath = vm.envOr(
            "JSON_PATH",
            string("attestations-latest.json")
        );
        console.log("Reading JSON from:", jsonPath);

        string memory jsonData = vm.readFile(jsonPath);
        AttestationData[] memory attestations = parseAndValidateJson(jsonData);

        console.log(
            string(
                abi.encodePacked(
                    "Parsed ",
                    vm.toString(attestations.length),
                    " attestations from JSON"
                )
            )
        );

        // Get schema ID
        bytes32 schemaId = vm.envOr(
            "SCHEMA_ID",
            bytes32(
                0xbcd7561083784f9b5a1c2b3ddb7aa9db263d43c58f7374cfa4875646824a47de
            )
        );
        console.log("Using Schema ID:", vm.toString(schemaId));

        // Create batch requests
        MultiAttestationRequest[] memory requests = createBatchRequests(
            attestations,
            schemaId
        );

        if (isDryRun) {
            console.log(
                string(
                    abi.encodePacked(
                        "DRY RUN: Would process ",
                        vm.toString(requests.length),
                        " batches"
                    )
                )
            );
            for (uint256 i = 0; i < requests.length; i++) {
                console.log(
                    string(
                        abi.encodePacked(
                            "Batch ",
                            vm.toString(i),
                            ": ",
                            vm.toString(requests[i].data.length),
                            " attestations"
                        )
                    )
                );
            }
            console.log("Dry run completed successfully!");
            return;
        }

        // Execute attestations
        executeAttestations(eas, requests);
    }

    function parseAndValidateJson(
        string memory jsonData
    ) internal view returns (AttestationData[] memory) {
        // JSON parseJson returns data with types inferred from JSON - addresses as address, strings as string
        console.log("Parsing JSON attestations array...");

        // Count attestations - parse the full array to get length
        // We'll use a temporary storage to collect attestations
        AttestationData[] memory tempAttestations = new AttestationData[](
            MAX_ATTESTATIONS
        );
        uint256 count = 0;

        // Parse attestations until we run out
        for (uint256 i = 0; i < MAX_ATTESTATIONS; i++) {
            string memory basePath = string(
                abi.encodePacked(".attestations[", vm.toString(i), "]")
            );

            // Try to parse this attestation - if it doesn't exist, parseJson will revert
            bytes memory testPath = vm.parseJson(
                jsonData,
                string(abi.encodePacked(basePath, ".recipient"))
            );
            if (testPath.length == 0) break; // No more attestations

            // Parse badgeName with proper bytes32 conversion
            string memory badgeNameStr = abi.decode(
                vm.parseJson(
                    jsonData,
                    string(abi.encodePacked(basePath, ".badgeName"))
                ),
                (string)
            );
            bytes32 badgeName;
            bytes memory badgeNameBytes = bytes(badgeNameStr);
            require(badgeNameBytes.length <= 32, "badgeName too long");
            assembly {
                badgeName := mload(add(badgeNameBytes, 32))
            }

            bytes memory justification = abi.decode(
                vm.parseJson(
                    jsonData,
                    string(abi.encodePacked(basePath, ".justification"))
                ),
                (bytes)
            );

            tempAttestations[count] = AttestationData({
                recipient: abi.decode(testPath, (address)),
                badgeName: badgeName,
                justification: justification
            });

            count++;
        }

        // Copy to properly sized array
        AttestationData[] memory attestations = new AttestationData[](count);
        for (uint256 i = 0; i < count; i++) {
            attestations[i] = tempAttestations[i];
        }

        console.log("Found", attestations.length, "attestations in JSON");

        require(
            attestations.length > 0,
            "JSON must contain at least 1 attestation"
        );
        require(
            attestations.length <= MAX_ATTESTATIONS,
            "Too many attestations, max 1000 allowed"
        );

        // Validate each attestation
        for (uint256 i = 0; i < attestations.length; i++) {
            require(
                attestations[i].recipient != address(0),
                string(
                    abi.encodePacked(
                        "Attestation ",
                        vm.toString(i),
                        ": invalid recipient address"
                    )
                )
            );
            require(
                attestations[i].badgeName != bytes32(0),
                string(
                    abi.encodePacked(
                        "Attestation ",
                        vm.toString(i),
                        ": badgeName cannot be empty"
                    )
                )
            );
        }

        return attestations;
    }

    function createBatchRequests(
        AttestationData[] memory attestations,
        bytes32 schemaId
    ) internal pure returns (MultiAttestationRequest[] memory) {
        uint256 totalBatches = (attestations.length + MAX_BATCH_SIZE - 1) /
            MAX_BATCH_SIZE;
        MultiAttestationRequest[]
            memory requests = new MultiAttestationRequest[](totalBatches);

        for (uint256 batchIndex = 0; batchIndex < totalBatches; batchIndex++) {
            uint256 startIdx = batchIndex * MAX_BATCH_SIZE;
            uint256 endIdx = startIdx + MAX_BATCH_SIZE;
            if (endIdx > attestations.length) endIdx = attestations.length;

            uint256 batchSize = endIdx - startIdx;
            requests[batchIndex].schema = schemaId;
            requests[batchIndex].data = new AttestationRequestData[](batchSize);

            for (uint256 i = 0; i < batchSize; i++) {
                AttestationData memory att = attestations[startIdx + i];
                requests[batchIndex].data[i] = AttestationRequestData({
                    recipient: att.recipient,
                    expirationTime: 0, // No expiration
                    revocable: true,
                    refUID: bytes32(0), // No reference
                    data: abi.encode(att.badgeName, att.justification),
                    value: 0 // No ETH value
                });
            }
        }

        return requests;
    }

    function executeAttestations(
        EAS eas,
        MultiAttestationRequest[] memory requests
    ) internal {
        uint256 pk = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(pk);

        console.log("Starting batch attestation execution...");

        uint256 totalAttestations = 0;
        for (
            uint256 batchIndex = 0;
            batchIndex < requests.length;
            batchIndex++
        ) {
            console.log(
                string(
                    abi.encodePacked(
                        "Processing batch ",
                        vm.toString(batchIndex + 1),
                        "/",
                        vm.toString(requests.length),
                        " - ",
                        vm.toString(requests[batchIndex].data.length),
                        " attestations"
                    )
                )
            );

            // Create single-element array for this batch
            MultiAttestationRequest[]
                memory singleBatch = new MultiAttestationRequest[](1);
            singleBatch[0] = requests[batchIndex];

            try eas.multiAttest(singleBatch) returns (bytes32[] memory uids) {
                console.log(
                    string(
                        abi.encodePacked(
                            "Batch ",
                            vm.toString(batchIndex + 1),
                            " completed with ",
                            vm.toString(uids.length),
                            " attestations"
                        )
                    )
                );
                totalAttestations += uids.length;

                // Log first few UIDs
                for (uint256 i = 0; i < uids.length && i < 3; i++) {
                    console.log(
                        string(
                            abi.encodePacked(
                                "  UID ",
                                vm.toString(i),
                                ": ",
                                vm.toString(uids[i])
                            )
                        )
                    );
                }
                if (uids.length > 3) {
                    console.log(
                        string(
                            abi.encodePacked(
                                "  ... and ",
                                vm.toString(uids.length - 3),
                                " more"
                            )
                        )
                    );
                }
            } catch Error(string memory reason) {
                console.log(
                    string(
                        abi.encodePacked(
                            "Batch ",
                            vm.toString(batchIndex + 1),
                            " failed: ",
                            reason
                        )
                    )
                );
                revert(
                    string(
                        abi.encodePacked(
                            "Batch ",
                            vm.toString(batchIndex + 1),
                            " failed: ",
                            reason
                        )
                    )
                );
            } catch {
                console.log(
                    string(
                        abi.encodePacked(
                            "Batch ",
                            vm.toString(batchIndex + 1),
                            " failed with unknown error"
                        )
                    )
                );
                revert(
                    string(
                        abi.encodePacked(
                            "Batch ",
                            vm.toString(batchIndex + 1),
                            " failed with unknown error"
                        )
                    )
                );
            }
        }

        vm.stopBroadcast();

        console.log("=== Attestation Summary ===");
        console.log("Total batches processed:", requests.length);
        console.log("Total attestations created:", totalAttestations);
        console.log("Execution completed successfully!");
    }
}
