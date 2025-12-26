import type { Config } from "wagmi";
import { simulateContract, readContract } from "@wagmi/core";
import { BADGE_REGISTRY_ADDRESS } from "@/lib/constants/blockchainConstants";
import { badgeRegistryAbiV2 } from "@/lib/abis/badgeRegistryAbi";
import { stringToBytes32, stringToBytes } from "@/lib/utils/blockchainUtils";
import { isDecodeError, isFunctionSelectorError } from "@/lib/utils/abiDetection";

/**
 * Detects BadgeRegistry contract version (V1 or V2) by probing with V2 ABI.
 * Returns "v2" if probe succeeds, "v1" if decode/selector error, throws otherwise.
 * 
 * Strategy:
 * - For non-empty registries: probes with getBadgeAt(0) read call.
 * - For empty registries: probes with simulateContract on createBadge (V2-first assumption).
 * 
 * This function encapsulates the version detection logic to make it testable
 * and reusable. The detection happens on-demand to prevent refetch storms.
 * 
 * NOTE: This is a pure "move + rename" refactor. The exact RPC call order and
 * branching is preserved: totalBadges -> getBadgeAt probe if >0 else simulate probe.
 */
export async function detectBadgeRegistryVersion(
  config: Config,
  account: `0x${string}`,
  chainId: number,
  currentCount: number
): Promise<"v1" | "v2"> {
  if (currentCount > 0) {
    // Probe version with getBadgeAt(0) - same logic as use-get-badges.ts
    try {
      await readContract(config, {
        abi: badgeRegistryAbiV2,
        address: BADGE_REGISTRY_ADDRESS,
        functionName: "getBadgeAt",
        args: [0n],
      });
      // Probe succeeded, contract is V2
      return "v2";
    } catch (err) {
      // Check if decode error (V1 contract)
      if (isDecodeError(err as Error)) {
        return "v1";
      } else {
        // Other error - surface it
        throw err;
      }
    }
  } else {
    // Empty registry (count === 0): probe with simulateContract
    // Use a unique name to avoid EMPTY_NAME/DUPLICATE_NAME false negatives
    const probeName = stringToBytes32(`__probe_${Date.now()}_${Math.random()}`);
    try {
      // Try V2 first (assumption: new registries are V2)
      await simulateContract(config, {
        abi: badgeRegistryAbiV2,
        address: BADGE_REGISTRY_ADDRESS,
        functionName: "createBadge",
        args: [probeName, stringToBytes("probe")],
        account,
        chainId,
      });
      // V2 simulation succeeded, contract is V2
      return "v2";
    } catch (err) {
      // Check if error indicates function selector not found (V1 contract)
      if (isFunctionSelectorError(err as Error)) {
        return "v1";
      } else {
        // Other error (EMPTY_NAME, DUPLICATE_NAME, network, etc.) - surface it
        throw err;
      }
    }
  }
}

