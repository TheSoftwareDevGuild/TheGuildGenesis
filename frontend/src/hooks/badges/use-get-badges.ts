import { useMemo } from "react";
import { useReadContract, useReadContracts } from "wagmi";
import {
  badgeRegistryAbiV1,
  badgeRegistryAbiV2,
} from "@/lib/abis/badgeRegistryAbi";
import { BADGE_REGISTRY_ADDRESS } from "@/lib/constants/blockchainConstants";
import type { Badge } from "@/lib/types/badges";
import { bytes32ToString, bytesToString } from "@/lib/utils/blockchainUtils";
import { isDecodeError } from "@/lib/utils/abiDetection";

export function useGetBadges(): {
  data: Badge[] | undefined;
  isLoading: boolean;
  error: Error | null;
  refetch: () => void;
} {
  const address = BADGE_REGISTRY_ADDRESS;

  const totalBadgesQuery = useReadContract({
    abi: badgeRegistryAbiV2,
    address,
    functionName: "totalBadges",
    query: {
      enabled: Boolean(address),
      retry: false,
    },
  });

  const count = Number((totalBadgesQuery.data as bigint | undefined) ?? 0n);

  // TODO(cleanup-after-v2): Remove V1 fallback logic after V2 full deployment. Always use V2 ABI. See docs/V2_CLEANUP.md.
  // TODO(cleanup): Remove version probe after V2 full deployment
  // Probe version with single getBadgeAt(0) call
  // This probe runs once per address and is cached forever
  // Only enabled if count > 0 (no probe for empty registries)
  const versionProbeQuery = useReadContract({
    abi: badgeRegistryAbiV2,
    address,
    functionName: "getBadgeAt",
    args: [0n],
    query: {
      enabled: Boolean(address) && count > 0,
      staleTime: Infinity,
      retry: 0,
      refetchOnWindowFocus: false,
      refetchOnReconnect: false,
      refetchOnMount: false,
    },
  });

  // TODO(cleanup): Simplify to always use V2 ABI after V2 full deployment
  // Determine ABI mode: V2 if probe succeeds, V1 if decode error, undefined while loading
  // If count === 0, abiMode remains undefined (no probe, no multicall)
  const abiMode = useMemo<"v1" | "v2" | undefined>(() => {
    if (count === 0) return undefined; // No badges, no ABI needed
    if (versionProbeQuery.isSuccess) return "v2";
    if (versionProbeQuery.error && isDecodeError(versionProbeQuery.error)) {
      return "v1"; // Decode error indicates V1 contract
    }
    return undefined; // Still loading or unknown
  }, [count, versionProbeQuery.isSuccess, versionProbeQuery.error]);

  // Build multicall contracts with correct ABI based on probe result
  const badgeContracts = useMemo(
    () =>
      count > 0 && abiMode !== undefined
        ? Array.from({ length: count }, (_, i) => ({
            // TODO(cleanup): Remove conditional ABI selection after V2 full deployment
            abi: abiMode === "v2" ? badgeRegistryAbiV2 : badgeRegistryAbiV1,
            address,
            functionName: "getBadgeAt" as const,
            args: [BigInt(i)],
          }))
        : [],
    [address, count, abiMode]
  );

  // Execute multicall with detected ABI
  const badgesQuery = useReadContracts({
    contracts: badgeContracts,
    allowFailure: false,
    query: {
      enabled: Boolean(address) && count > 0 && abiMode !== undefined,
      retry: 0,
      staleTime: Infinity,
      refetchOnWindowFocus: false,
      refetchOnReconnect: false,
      refetchOnMount: false,
    },
  });

  // Decode descriptions based on detected ABI
  // If count === 0, return empty array
  const data: Badge[] | undefined = useMemo(() => {
    if (count === 0) return [];
    const results = badgesQuery.data as
      | [`0x${string}`, `0x${string}`, `0x${string}`][]
      | undefined;
    if (!results) return undefined;

    return results.map((item) => {
      if (!Array.isArray(item) || item.length < 2) {
        throw new Error(`Unexpected item shape: ${JSON.stringify(item)}`);
      }

      const [nameBytes, descriptionBytes] = item as [`0x${string}`, `0x${string}`, `0x${string}`];
      const name = bytes32ToString(nameBytes);
      // TODO(cleanup): Remove conditional decoding after V2 full deployment
      const description =
        abiMode === "v2"
          ? bytesToString(descriptionBytes) // V2: bytes (variable length)
          : bytes32ToString(descriptionBytes); // V1: bytes32 (fixed 32 bytes)

      return { name, description };
    });
  }, [count, badgesQuery.data, abiMode]);

  const isLoading =
    totalBadgesQuery.isLoading ||
    (count > 0 && abiMode === undefined ? versionProbeQuery.isLoading : false) ||
    (count > 0 && badgesQuery.isLoading);

  const error =
    (totalBadgesQuery.error as Error | null) ||
    // Only propagate probe error if it's NOT a decode error (decode errors are expected for V1)
    (count > 0 &&
    versionProbeQuery.error &&
    !isDecodeError(versionProbeQuery.error)
      ? (versionProbeQuery.error as Error | null)
      : null) ||
    (count > 0 ? (badgesQuery.error as Error | null) : null) ||
    null;

  return { data, isLoading, error, refetch: totalBadgesQuery.refetch };
}
