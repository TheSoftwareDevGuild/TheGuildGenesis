import { useWriteContract, useWaitForTransactionReceipt } from "wagmi";
import { badgeRegistryAbi } from "@/lib/abis/badgeRegistryAbi";
import { BADGE_REGISTRY_ADDRESS } from "@/lib/constants/blockchainConstants";

export function useVoteBadge() {
  const { writeContract, data: hash, isPending, error } = useWriteContract();

  const { isLoading: isConfirming, isSuccess } = useWaitForTransactionReceipt({
    hash,
  });

  const vote = (badgeName: string, isUpvote: boolean) => {
    writeContract({
      abi: badgeRegistryAbi,
      address: BADGE_REGISTRY_ADDRESS,
      functionName: "vote",
      args: [badgeName as `0x${string}`, isUpvote],
    });
  };

  return {
    vote,
    isPending,
    isConfirming,
    isSuccess,
    error,
  };
}