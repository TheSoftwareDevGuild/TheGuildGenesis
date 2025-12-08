// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import {Test} from "forge-std/Test.sol";
import {EmitAttestationsJson} from "../script/EmitAttestationsJson.s.sol";

contract EmitAttestationsJsonTest is Test {
    EmitAttestationsJson private script;

    function setUp() public {
        script = new EmitAttestationsJson();
    }

    function test_AttestationDataStructure() public pure {
        // Test that the AttestationData struct is properly defined
        EmitAttestationsJson.AttestationData
            memory testData = EmitAttestationsJson.AttestationData({
                recipient: 0x742d35Cc6634C0532925a3b844Bc454e4438f44e,
                badgeName: bytes32(abi.encodePacked("Rust")),
                justification: bytes(
                    "Awarded for outstanding Rust contributions"
                )
            });

        assertEq(
            testData.recipient,
            0x742d35Cc6634C0532925a3b844Bc454e4438f44e
        );
        assertEq(testData.badgeName, bytes32(abi.encodePacked("Rust")));
        assertEq(
            testData.justification,
            bytes("Awarded for outstanding Rust contributions")
        );
    }

    function test_AttestationDataArray() public pure {
        // Test that we can create arrays of AttestationData
        EmitAttestationsJson.AttestationData[]
            memory attestations = new EmitAttestationsJson.AttestationData[](2);

        attestations[0] = EmitAttestationsJson.AttestationData({
            recipient: 0x742d35Cc6634C0532925a3b844Bc454e4438f44e,
            badgeName: bytes32(abi.encodePacked("Rust")),
            justification: bytes("Rust expert")
        });

        attestations[1] = EmitAttestationsJson.AttestationData({
            recipient: 0x742d35Cc6634C0532925a3b844Bc454e4438f44e,
            badgeName: bytes32(abi.encodePacked("Solidity")),
            justification: bytes("Solidity expert")
        });

        assertEq(attestations.length, 2);
        assertEq(attestations[0].badgeName, bytes32(abi.encodePacked("Rust")));
        assertEq(
            attestations[1].badgeName,
            bytes32(abi.encodePacked("Solidity"))
        );
    }
}
