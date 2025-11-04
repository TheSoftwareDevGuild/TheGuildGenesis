// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {SchemaResolver} from "eas-contracts/resolver/SchemaResolver.sol";
import {IEAS, Attestation} from "eas-contracts/IEAS.sol";
import {TheGuildActivityToken} from "./TheGuildActivityToken.sol";
import {TheGuildBadgeRegistry} from "./TheGuildBadgeRegistry.sol";

/// @title TheGuildAttestationResolver
/// @notice EAS schema resolver that mints TGA tokens on attest.
contract TheGuildAttestationResolver is SchemaResolver {
    TheGuildActivityToken private _token;
    TheGuildBadgeRegistry private _badgeRegistry;

    // TODO: get rid of this when we have our graphQL setup
    bytes32[] private _attestationIds;

    constructor(
        IEAS eas,
        TheGuildActivityToken token,
        TheGuildBadgeRegistry badgeRegistry
    ) SchemaResolver(eas) {
        _token = token;
        _badgeRegistry = badgeRegistry;
    }

    /// @inheritdoc SchemaResolver
    function onAttest(
        Attestation calldata attestation,
        uint256
    ) internal override returns (bool) {
        (bytes32 badgeName, ) = abi.decode(attestation.data, (bytes32, bytes));
        if (!_badgeRegistry.exists(badgeName)) {
            return false;
        }

        _token.mint(attestation.attester, 10 * (10 ** _token.decimals()));
        // TODO: get rid of this when we have our graphQL setup
        _attestationIds.push(attestation.uid);
        return true;
    }

    /// @inheritdoc SchemaResolver
    function onRevoke(
        Attestation calldata,
        uint256
    ) internal pure override returns (bool) {
        return true;
    }

    // TODO: get rid of this when we have our graphQL setup
    function getAttestationCount() external view returns (uint256) {
        return _attestationIds.length;
    }

    function getAttestationAtIndex(
        uint256 index
    ) external view returns (Attestation memory) {
        return _eas.getAttestation(_attestationIds[index]);
    }
}
