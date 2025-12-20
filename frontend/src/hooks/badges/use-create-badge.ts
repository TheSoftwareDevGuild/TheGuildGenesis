import { useMemo } from "react";
import { useWriteContract, useWaitForTransactionReceipt } from "wagmi";
import { BADGE_REGISTRY_ADDRESS } from "@/lib/constants/blockchainConstants";
import {
  badgeRegistryAbiV1,
  badgeRegistryAbiV2,
} from "@/lib/abis/badgeRegistryAbi";
import { stringToBytes32, stringToBytes } from "@/lib/utils/blockchainUtils";

export function useCreateBadge() {
  const { writeContractAsync, isPending, error, data, reset } =
    useWriteContract();

  const createBadge = useMemo(() => {
    return async (name: string, description: string) => {
      if (!BADGE_REGISTRY_ADDRESS) throw new Error("Missing registry address");
      const nameBytes = stringToBytes32(name);

      // TODO(cleanup): Remove V1 fallback after V2 full deployment
      // Try V2 first (bytes description), fallback to V1 (bytes32)
      try {
        return await writeContractAsync({
          abi: badgeRegistryAbiV2,
          address: BADGE_REGISTRY_ADDRESS,
          functionName: "createBadge",
          args: [nameBytes, stringToBytes(description)],
        });
      } catch {
        // Fallback to V1 (bytes32 description with truncation/padding)
        return writeContractAsync({
          abi: badgeRegistryAbiV1,
          address: BADGE_REGISTRY_ADDRESS,
          functionName: "createBadge",
          args: [nameBytes, stringToBytes32(description)],
        });
      }
    };
  }, [writeContractAsync]);

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
