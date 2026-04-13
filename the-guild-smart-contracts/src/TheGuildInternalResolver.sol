// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {SchemaResolver} from "eas-contracts/resolver/SchemaResolver.sol";
import {IEAS, Attestation} from "eas-contracts/IEAS.sol";
import {Ownable} from "openzeppelin-contracts/contracts/access/Ownable.sol";

/// @title TheGuildInternalResolver
/// @notice EAS schema resolver that restricts attestations to authorized Guild accounts only.
contract TheGuildInternalResolver is SchemaResolver, Ownable {
    mapping(address => bool) private _authorizedAttesters;

    event AttesterAuthorized(address indexed attester, bool authorized);

    constructor(IEAS eas, address initialOwner) SchemaResolver(eas) Ownable(initialOwner) {
        _authorizedAttesters[initialOwner] = true;
        emit AttesterAuthorized(initialOwner, true);
    }

    /// @notice Authorize or deauthorize an account to create attestations.
    function setAuthorizedAttester(address attester, bool authorized) external onlyOwner {
        _authorizedAttesters[attester] = authorized;
        emit AttesterAuthorized(attester, authorized);
    }

    /// @notice Check if an account is an authorized attester.
    function isAuthorizedAttester(address attester) public view returns (bool) {
        return _authorizedAttesters[attester];
    }

    /// @inheritdoc SchemaResolver
    function onAttest(
        Attestation calldata attestation,
        uint256
    ) internal view override returns (bool) {
        return _authorizedAttesters[attestation.attester];
    }

    /// @inheritdoc SchemaResolver
    function onRevoke(
        Attestation calldata attestation,
        uint256
    ) internal view override returns (bool) {
        return _authorizedAttesters[attestation.attester];
    }
}
