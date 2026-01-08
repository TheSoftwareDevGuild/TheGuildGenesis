# BadgeRegistry V2 Cleanup Guide

## Overview

This document outlines the cleanup steps required after full V2 deployment of BadgeRegistry contracts. The current codebase includes retro-compatibility logic to support both V1 and V2 contracts during the migration period. Once all registries are upgraded to V2, this temporary code can be removed.

## Files to Delete

- `frontend/src/lib/utils/abiDetection.ts` - Error-based ABI detection utilities
- `frontend/src/lib/badges/registryVersion.ts` - Version detection module

## Functions / Logic to Remove

- `detectBadgeRegistryVersion()` - Version detection function
- `isDecodeError()` - Decode error detection utility
- `isFunctionSelectorError()` - Function selector error detection utility
- `buildCreateBadgeArgs()` - Conditional argument builder for V1/V2 differences
- Any version-probe logic / error-based ABI inference

## Hook Simplifications

### use-create-badge.ts

- Remove version detection and conditional ABI selection
- Remove `detectBadgeRegistryVersion()` call
- Always use `badgeRegistryAbiV2` (remove conditional `finalAbiMode === "v2" ? badgeRegistryAbiV2 : badgeRegistryAbiV1`)
- Always use `bytes` description format: replace `buildCreateBadgeArgs()` with direct `stringToBytes(description)`
- Remove `badgeRegistryAbiV1` import
- Remove `detectBadgeRegistryVersion` import
- Remove `buildCreateBadgeArgs` import

### use-get-badges.ts

- Remove version probing / `abiMode` inference
- Remove `versionProbeQuery` query
- Remove `abiMode` useMemo logic
- Always use `badgeRegistryAbiV2` in `badgeContracts` (remove conditional ABI selection)
- Always decode description as `bytes` using `bytesToString()` (remove conditional `bytesToString` vs `bytes32ToString`)
- Remove `isDecodeError` import and usage
- Remove `badgeRegistryAbiV1` import

## Expected End State

- Only V2 ABI (`badgeRegistryAbiV2`) used throughout the codebase
- No fallback branches or conditional logic based on contract version
- No error-based ABI detection or version probing
- Simpler, more maintainable codebase with reduced complexity

