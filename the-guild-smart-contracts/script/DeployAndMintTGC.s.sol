// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {TheGuildContributionToken} from "../src/TheGuildContributionToken.sol";

contract DeployAndMintTGC is Script {
    function run() public {
        // CSV file path relative to project root
        string memory csvPath = "script/data/initial_tgc_mints.csv";

        vm.startBroadcast();

        TheGuildContributionToken token = new TheGuildContributionToken();

        // read CSV and prepare arrays for batchMint
        string memory csv = vm.readFile(csvPath);
        string[] memory lines = csvSplit(csv);

        // count non-empty lines
        uint256 count = 0;
        for (uint256 i = 0; i < lines.length; i++) if (bytes(lines[i]).length != 0) count++;

        address[] memory recipients = new address[](count);
        uint256[] memory amounts = new uint256[](count);
        bytes32[] memory reasons = new bytes32[](count);

        uint256 idx = 0;
        for (uint256 i = 0; i < lines.length; i++) {
            string memory line = lines[i];
            if (bytes(line).length == 0) continue;
            (address recipient, uint256 amount, bytes32 reason) = parseLine(line);
            recipients[idx] = recipient;
            amounts[idx] = amount;
            reasons[idx] = reason;
            idx++;
        }

        // compute distributionId as keccak256(csv)
        bytes32 distributionId = keccak256(bytes(csv));

        // perform batch mint
        token.batchMint(distributionId, recipients, amounts, reasons);

        vm.stopBroadcast();
    }

    // --- small CSV parser helpers ---
    function csvSplit(string memory s) internal pure returns (string[] memory) {
        // naive split on '\n'
        bytes memory b = bytes(s);
        uint256 linesCount = 1;
        for (uint256 i = 0; i < b.length; i++) {
            if (b[i] == "\n") linesCount++;
        }
        string[] memory parts = new string[](linesCount);
        uint256 idx = 0;
        bytes memory cur;
        for (uint256 i = 0; i < b.length; i++) {
            if (b[i] == "\n") {
                parts[idx] = string(cur);
                idx++;
                cur = "";
            } else {
                cur = abi.encodePacked(cur, b[i]);
            }
        }
        // last
        if (cur.length != 0) {
            parts[idx] = string(cur);
        }
        return parts;
    }

    function parseLine(string memory line) internal pure returns (address, uint256, bytes32) {
        // naive split by comma
        bytes memory b = bytes(line);
        string[] memory cols = new string[](3);
        uint256 col = 0;
        bytes memory cur;
        for (uint256 i = 0; i < b.length; i++) {
            if (b[i] == ",") {
                cols[col] = string(cur);
                col++;
                cur = "";
            } else {
                cur = abi.encodePacked(cur, b[i]);
            }
        }
        if (cur.length != 0) cols[col] = string(cur);

        address recipient = parseAddr(cols[0]);
        uint256 amount = parseUint(cols[1]);
        bytes32 reason = parseBytes32Hex(cols[2]);
        return (recipient, amount, reason);
    }

    function parseAddr(string memory s) internal pure returns (address) {
        bytes memory bb = bytes(s);
        // fallback to address(0) if empty
        if (bb.length == 0) return address(0);
        return parseAddrFromHex(s);
    }

    function parseAddrFromHex(string memory s) internal pure returns (address) {
        bytes memory _s = bytes(s);
        uint256 start = 0;
        if (_s.length >= 2 && _s[0] == '0' && (_s[1] == 'x' || _s[1] == 'X')) start = 2;
        require(_s.length - start == 40, "INVALID_ADDR_LENGTH");
        uint160 addr = 0;
        for (uint256 i = start; i < _s.length; i++) {
            addr <<= 4;
            uint8 c = uint8(_s[i]);
            if (c >= 48 && c <= 57) addr |= uint160(c - 48);
            else if (c >= 65 && c <= 70) addr |= uint160(c - 55);
            else if (c >= 97 && c <= 102) addr |= uint160(c - 87);
            else revert("INVALID_HEX_CHAR");
        }
        return address(addr);
    }

    function parseUint(string memory s) internal pure returns (uint256) {
        bytes memory b = bytes(s);
        uint256 n = 0;
        for (uint256 i = 0; i < b.length; i++) {
            uint8 c = uint8(b[i]);
            if (c >= 48 && c <= 57) n = n * 10 + (c - 48);
        }
        return n;
    }

    function parseBytes32Hex(string memory s) internal pure returns (bytes32) {
        bytes memory _s = bytes(s);
        if (_s.length == 0) return bytes32(0);
        uint256 start = 0;
        if (_s.length >= 2 && _s[0] == '0' && (_s[1] == 'x' || _s[1] == 'X')) start = 2;
        bytes32 res = bytes32(0);
        uint256 chars = _s.length - start;
        // read up to 64 hex chars (32 bytes)
        uint256 toRead = chars > 64 ? 64 : chars;
        for (uint256 i = 0; i < toRead; i++) {
            res <<= 4;
            uint8 c = uint8(_s[start + i]);
            if (c >= 48 && c <= 57) res |= bytes32(uint256(c - 48));
            else if (c >= 65 && c <= 70) res |= bytes32(uint256(c - 55));
            else if (c >= 97 && c <= 102) res |= bytes32(uint256(c - 87));
            else revert("INVALID_HEX_CHAR");
        }
        // shift left to make it right-aligned? Keep as parsed (big-endian)
        return res;
    }
}
