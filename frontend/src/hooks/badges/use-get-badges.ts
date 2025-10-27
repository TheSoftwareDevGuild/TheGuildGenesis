import { useMemo } from "react";
import { useReadContract, useReadContracts } from "wagmi";
import { badgeRegistryAbi } from "@/lib/abis/badgeRegistryAbi";
import { BADGE_REGISTRY_ADDRESS } from "@/lib/constants/blockchainConstants";
import type { Badge } from "@/lib/types/badges";
import { bytes32ToString } from "@/lib/utils/blockchainUtils";

export function useGetBadges(): {
  data: Badge[] | undefined;
  isLoading: boolean;
  error: Error | null;
  refetch: () => void;
} {
  const address = BADGE_REGISTRY_ADDRESS;

  const totalBadgesQuery = useReadContract({
    abi: badgeRegistryAbi,
    address,
    functionName: "totalBadges",
    query: {
      enabled: Boolean(address),
    },
  });

  const count = Number((totalBadgesQuery.data as bigint | undefined) ?? 0n);

  const badgeContracts = useMemo(
    () =>
      count > 0
        ? Array.from({ length: count }, (_, i) => ({
            abi: badgeRegistryAbi,
            address,
            functionName: "getBadgeAt" as const,
            args: [BigInt(i)],
          }))
        : [],
    [address, count]
  );

  const badgesQuery = useReadContracts({
    contracts: badgeContracts,
    allowFailure: false,
    query: {
      enabled: Boolean(address) && count > 0,
    },
  });

  const data: Badge[] | undefined = useMemo(() => {
    const results = badgesQuery.data as
      | [`0x${string}`, `0x${string}`, `0x${string}`, bigint][]
      | undefined;
    if (!results) return undefined;
    return results.map(([nameBytes, descriptionBytes, creator, voteScore]) => ({
      name: bytes32ToString(nameBytes),
      description: bytes32ToString(descriptionBytes),
      creator,
      voteScore: Number(voteScore),
    }));
  }, [badgesQuery.data]);

  const isLoading = totalBadgesQuery.isLoading || badgesQuery.isLoading;

  const error =
    (totalBadgesQuery.error as Error | null) ||
    (badgesQuery.error as Error | null) ||
    null;

  return { data, isLoading, error, refetch: totalBadgesQuery.refetch };
}
