import type { Address } from "viem";

export const EAS_CONTRACT_ADDRESS = (import.meta.env
  .PUBLIC_EAS_CONTRACT_ADDRESS ||
  "0xC2679fBD37d54388Ce493F1DB75320D236e1815e") as Address; // Sepolia default

export const SCHEMA_ID = (import.meta.env.PUBLIC_SCHEMA_ID ||
  "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef") as `0x${string}`;

// ERC20 TGA token address (for balances/transfers)
export const ACTIVITY_TOKEN_ADDRESS = (import.meta.env
  .PUBLIC_ACTIVITY_TOKEN_ADDRESS || "") as Address;

// EAS resolver contract that exposes attestation getters
export const ATTESTATION_RESOLVER_ADDRESS = (import.meta.env
  .PUBLIC_ATTESTATION_RESOLVER_ADDRESS || "") as Address;

export const BADGE_REGISTRY_ADDRESS = (import.meta.env
  .PUBLIC_BADGE_REGISTRY_ADDRESS || "") as Address;
