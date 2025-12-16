import { useState, useEffect } from "react";
import { useReadContract } from "wagmi";
import { badgeRegistryAbiV2 } from "@/lib/abis/badgeRegistryAbi";

export function useBadgeRegistryVersion(address?: `0x${string}`): {
  version: "v1" | "v2" | null;
  isLoading: boolean;
  error: Error | null;
} {
  const [version, setVersion] = useState<"v1" | "v2" | null>(null);

  // Reset version when address changes
  useEffect(() => {
    setVersion(null);
  }, [address]);

  // Read totalBadges using V2 ABI (same signature in both versions)
  const totalBadgesQuery = useReadContract({
    abi: badgeRegistryAbiV2,
    address,
    functionName: "totalBadges",
    query: {
      enabled: Boolean(address),
    },
  });

  const count = Number((totalBadgesQuery.data as bigint | undefined) ?? 0n);

  // Test version by calling getBadgeAt(0) with V2 ABI
  const versionTestQuery = useReadContract({
    abi: badgeRegistryAbiV2,
    address,
    functionName: "getBadgeAt",
    args: [0n],
    query: {
      enabled: Boolean(address) && count > 0 && version === null,
    },
  });

  // Detection logic
  useEffect(() => {
    if (!address) return;

    if (totalBadgesQuery.isSuccess && count === 0) {
      // Empty registry defaults to V2
      setVersion("v2");
      return;
    }

    if (versionTestQuery.isSuccess) {
      // V2 call succeeded
      setVersion("v2");
    } else if (versionTestQuery.isError) {
      // V2 call failed, assume V1
      setVersion("v1");
    }
  }, [
    address,
    count,
    totalBadgesQuery.isSuccess,
    versionTestQuery.isSuccess,
    versionTestQuery.isError,
  ]);

  // Calculate isLoading
  const isLoading =
    Boolean(address) &&
    (totalBadgesQuery.isLoading ||
      (count > 0 && versionTestQuery.isLoading) ||
      version === null);

  // If address is undefined, return early state
  if (!address) {
    return { version: null, isLoading: false, error: null };
  }

  // Note: versionTestQuery.error is expected for V1, not a real error to propagate
  const error = (totalBadgesQuery.error as Error | null) || null;

  return { version, isLoading, error };
}

