import { useWriteContract, useWaitForTransactionReceipt } from "wagmi";
import { badgeRegistryAbi } from "@/lib/abis/badgeRegistryAbi";
import { BADGE_REGISTRY_ADDRESS } from "@/lib/constants/blockchainConstants";

export function useAddPointers() {
  const { writeContract, data: hash, isPending, error } = useWriteContract();

  const { isLoading: isConfirming, isSuccess } = useWaitForTransactionReceipt({
    hash,
  });

  const addPointers = (fromBadge: string, toBadges: string[]) => {
    writeContract({
      abi: badgeRegistryAbi,
      address: BADGE_REGISTRY_ADDRESS,
      functionName: "addPointers",
      args: [fromBadge as `0x${string}`, toBadges as `0x${string}`[]],
    });
  };

  return {
    addPointers,
    isPending,
    isConfirming,
    isSuccess,
    error,
  };
}