import { useMemo } from "react";
import { useReadContract } from "wagmi";
import { badgeRegistryAbi } from "@/lib/abis/badgeRegistryAbi";
import { BADGE_REGISTRY_ADDRESS } from "@/lib/constants/blockchainConstants";
import type { Badge } from "@/lib/types/badges";
import { bytes32ToString } from "@/lib/utils/blockchainUtils";

export function useGetBadgeDetails(badgeName?: string) {
  const address = BADGE_REGISTRY_ADDRESS;

  const badgeQuery = useReadContract({
    abi: badgeRegistryAbi,
    address,
    functionName: "getBadge",
    args: badgeName ? [badgeName as `0x${string}`] : undefined,
    query: {
      enabled: Boolean(address) && Boolean(badgeName),
    },
  });

  const pointersQuery = useReadContract({
    abi: badgeRegistryAbi,
    address,
    functionName: "getPointers",
    args: badgeName ? [badgeName as `0x${string}`] : undefined,
    query: {
      enabled: Boolean(address) && Boolean(badgeName),
    },
  });

  const data: Badge | undefined = useMemo(() => {
    const badgeResult = badgeQuery.data as
      | [`0x${string}`, `0x${string}`, `0x${string}`, bigint]
      | undefined;
    const pointersResult = pointersQuery.data as `0x${string}`[] | undefined;

    if (!badgeResult) return undefined;

    const [nameBytes, descriptionBytes, creator, voteScore] = badgeResult;
    return {
      name: bytes32ToString(nameBytes),
      description: bytes32ToString(descriptionBytes),
      creator,
      voteScore: Number(voteScore),
      pointers: pointersResult?.map(bytes32ToString),
    };
  }, [badgeQuery.data, pointersQuery.data]);

  const isLoading = badgeQuery.isLoading || pointersQuery.isLoading;
  const error = badgeQuery.error || pointersQuery.error;

  return { data, isLoading, error, refetch: badgeQuery.refetch };
}