import { useMemo } from "react";
import { useReadContract, useReadContracts } from "wagmi";
import {
  badgeRegistryAbiV1,
  badgeRegistryAbiV2,
} from "@/lib/abis/badgeRegistryAbi";
import { BADGE_REGISTRY_ADDRESS } from "@/lib/constants/blockchainConstants";
import type { Badge } from "@/lib/types/badges";
import { bytes32ToString, bytesToString } from "@/lib/utils/blockchainUtils";
import { useBadgeRegistryVersion } from "./use-badge-registry-version";

export function useGetBadges(): {
  data: Badge[] | undefined;
  isLoading: boolean;
  error: Error | null;
  refetch: () => void;
} {
  const address = BADGE_REGISTRY_ADDRESS;
  const {
    version,
    isLoading: versionLoading,
    error: versionError,
  } = useBadgeRegistryVersion(address);

  const totalBadgesQuery = useReadContract({
    abi: badgeRegistryAbiV2, // totalBadges has same signature in both versions
    address,
    functionName: "totalBadges",
    query: {
      enabled: Boolean(address),
    },
  });

  const count = Number((totalBadgesQuery.data as bigint | undefined) ?? 0n);

  const badgeContracts = useMemo(
    () =>
      count > 0 && version !== null
        ? Array.from({ length: count }, (_, i) => ({
            abi: version === "v2" ? badgeRegistryAbiV2 : badgeRegistryAbiV1,
            address,
            functionName: "getBadgeAt" as const,
            args: [BigInt(i)],
          }))
        : [],
    [address, count, version]
  );

  const badgesQuery = useReadContracts({
    contracts: badgeContracts,
    allowFailure: false,
    query: {
      enabled: Boolean(address) && count > 0 && version !== null,
    },
  });

  const data: Badge[] | undefined = useMemo(() => {
    const results = badgesQuery.data as
      | [`0x${string}`, `0x${string}`, `0x${string}`][]
      | undefined;
    if (!results) return undefined;
    const isV2 = version === "v2";
    return results.map(([nameBytes, descriptionBytes]) => ({
      name: bytes32ToString(nameBytes),
      description: isV2
        ? bytesToString(descriptionBytes)
        : bytes32ToString(descriptionBytes),
    }));
  }, [badgesQuery.data, version]);

  const isLoading =
    versionLoading || totalBadgesQuery.isLoading || badgesQuery.isLoading;

  const error =
    versionError ||
    (totalBadgesQuery.error as Error | null) ||
    (badgesQuery.error as Error | null) ||
    null;

  return { data, isLoading, error, refetch: totalBadgesQuery.refetch };
}
