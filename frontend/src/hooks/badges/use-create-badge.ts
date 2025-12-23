import { useMemo } from "react";
import { useWriteContract, useWaitForTransactionReceipt, useAccount, useConfig } from "wagmi";
import { simulateContract, readContract } from "@wagmi/core";
import { BADGE_REGISTRY_ADDRESS } from "@/lib/constants/blockchainConstants";
import {
  badgeRegistryAbiV1,
  badgeRegistryAbiV2,
} from "@/lib/abis/badgeRegistryAbi";
import { stringToBytes32, stringToBytes } from "@/lib/utils/blockchainUtils";

/**
 * TODO(cleanup): Remove decode error detection after V2 full deployment
 * Check if error is a decode error (ABI mismatch).
 * V2 ABI will fail to decode V1 contract responses with decode errors.
 * Must NOT classify RPC/network errors as decode errors.
 */
function isDecodeError(error: Error | null): boolean {
  if (!error) return false;
  const message = error.message.toLowerCase();
  const name = error.name.toLowerCase();
  // Only detect actual decode errors, not RPC/network issues
  return (
    name.includes("positionoutofbounds") ||
    name.includes("decodefunctionresult") ||
    (name.includes("contractfunctionexecution") &&
      (message.includes("decode") || message.includes("position"))) ||
    (message.includes("decode") &&
      (message.includes("function") || message.includes("abi"))) ||
    (message.includes("position") && message.includes("out of bounds"))
  );
}

/**
 * TODO(cleanup): Remove function selector error detection after V2 full deployment
 * Check if error indicates function selector not found (ABI mismatch for write operations).
 * When simulating a write with wrong ABI, we get "function selector not found" errors.
 * Must NOT classify other revert reasons (EMPTY_NAME, DUPLICATE_NAME, etc.) as selector errors.
 */
function isFunctionSelectorError(error: Error | null): boolean {
  if (!error) return false;
  const message = error.message.toLowerCase();
  const name = error.name.toLowerCase();
  // Detect function selector not found errors
  return (
    name.includes("functionnotfound") ||
    name.includes("functionselector") ||
    (message.includes("function") &&
      (message.includes("not found") || message.includes("selector"))) ||
    (message.includes("selector") && message.includes("not found"))
  );
}

export function useCreateBadge() {
  const config = useConfig();
  const { address: account, chainId } = useAccount();
  const { writeContractAsync, isPending, error, data, reset } =
    useWriteContract();


  const createBadge = useMemo(() => {
    return async (name: string, description: string) => {
      if (!BADGE_REGISTRY_ADDRESS) {
        throw new Error("Badge registry address not configured");
      }
      if (!account) {
        throw new Error("No wallet connected");
      }
      if (!chainId) {
        throw new Error("No chain ID available");
      }

      const nameBytes = stringToBytes32(name);

      // Determine ABI mode deterministically BEFORE sending transaction
      // All version detection happens on-demand here to prevent refetch storm
      let finalAbiMode: "v1" | "v2";

      // Fetch totalBadges on-demand
      const totalBadgesResult = await readContract(config, {
        abi: badgeRegistryAbiV2,
        address: BADGE_REGISTRY_ADDRESS,
        functionName: "totalBadges",
      });
      const currentCount = Number(totalBadgesResult ?? 0n);

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
          finalAbiMode = "v2";
        } catch (err) {
          // Check if decode error (V1 contract)
          if (isDecodeError(err as Error)) {
            finalAbiMode = "v1";
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
            account: account as `0x${string}` | undefined,
            chainId,
          });
          // V2 simulation succeeded, contract is V2
          finalAbiMode = "v2";
        } catch (err) {
          // Check if error indicates function selector not found (V1 contract)
          if (isFunctionSelectorError(err as Error)) {
            // Function selector not found, contract is V1
            finalAbiMode = "v1";
          } else {
            // Other error (EMPTY_NAME, DUPLICATE_NAME, network, etc.) - surface it
            throw err;
          }
        }
      }

      // Now that ABI mode is determined, simulate and send with correct ABI
      const simulation = await simulateContract(config, {
        abi: finalAbiMode === "v2" ? badgeRegistryAbiV2 : badgeRegistryAbiV1,
        address: BADGE_REGISTRY_ADDRESS,
        functionName: "createBadge",
        args:
          finalAbiMode === "v2"
            ? [nameBytes, stringToBytes(description)]
            : [nameBytes, stringToBytes32(description)],
        account: account as `0x${string}` | undefined,
        chainId,
      });

      // Single writeContractAsync call with correct ABI
      return await writeContractAsync(simulation.request);
    };
  }, [writeContractAsync, account, chainId, config]);

  const wait = useWaitForTransactionReceipt({
    hash: data as `0x${string}` | undefined,
    confirmations: 6,
    query: { enabled: Boolean(data) },
  });

  return {
    createBadge,
    isPending,
    error,
    data,
    isConfirming: wait.isLoading,
    isConfirmed: wait.isSuccess,
    receipt: wait.data,
    waitError: wait.error as Error | null,
    reset,
  };
}
