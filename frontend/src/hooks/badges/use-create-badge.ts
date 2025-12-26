import { useMemo } from "react";
import { useWriteContract, useWaitForTransactionReceipt, useAccount, useConfig } from "wagmi";
import { simulateContract, readContract } from "@wagmi/core";
import { BADGE_REGISTRY_ADDRESS } from "@/lib/constants/blockchainConstants";
import {
  badgeRegistryAbiV1,
  badgeRegistryAbiV2,
} from "@/lib/abis/badgeRegistryAbi";
import { stringToBytes32, buildCreateBadgeArgs } from "@/lib/utils/blockchainUtils";
import { detectBadgeRegistryVersion } from "@/lib/badges/registryVersion";

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
      // Fetch totalBadges on-demand
      const totalBadgesResult = await readContract(config, {
        abi: badgeRegistryAbiV2,
        address: BADGE_REGISTRY_ADDRESS,
        functionName: "totalBadges",
      });
      const currentCount = Number(totalBadgesResult ?? 0n);

      const finalAbiMode = await detectBadgeRegistryVersion(
        config,
        account as `0x${string}`,
        chainId,
        currentCount
      );

      // Now that ABI mode is determined, simulate and send with correct ABI
      const simulation = await simulateContract(config, {
        abi: finalAbiMode === "v2" ? badgeRegistryAbiV2 : badgeRegistryAbiV1,
        address: BADGE_REGISTRY_ADDRESS,
        functionName: "createBadge",
        args: buildCreateBadgeArgs(nameBytes, description, finalAbiMode),
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
