import { useMemo } from "react";
import { useReadContract, useReadContracts } from "wagmi";
import { decodeAbiParameters } from "viem";
import { activityTokenAbi } from "@/lib/abis/activityTokenAbi";
import { ACTIVITY_TOKEN_ADDRESS } from "@/lib/constants/blockchainConstants";
import { bytes32ToString } from "@/lib/utils/blockchainUtils";
import type { AttestationItem } from "@/lib/types/attestation";

function tryDecodeBadgeData(data: `0x${string}` | null | undefined): {
  badgeName: string;
  justification: string;
} {
  if (!data) return { badgeName: "", justification: "" };
  try {
    const [nameBytes, justificationBytes] = decodeAbiParameters(
      [{ type: "bytes32" }, { type: "bytes" }],
      data
    ) as [`0x${string}`, `0x${string}`];
    return {
      badgeName: bytes32ToString(nameBytes),
      justification: justificationBytes === "0x" ? "" : Buffer.from(justificationBytes.slice(2), "hex").toString("utf-8"),
    };
  } catch {
    return { badgeName: "", justification: "" };
  }
}

export function useGetAttestations(): {
  data: AttestationItem[] | undefined;
  isLoading: boolean;
  error: Error | null;
  refetch: () => void;
} {
  const address = ACTIVITY_TOKEN_ADDRESS;

  const countQuery = useReadContract({
    abi: activityTokenAbi,
    address,
    functionName: "getAttestationCount",
    query: { enabled: Boolean(address) },
  });

  const count = Number((countQuery.data as bigint | undefined) ?? 0n);

  const attestationCalls = useMemo(
    () =>
      count > 0
        ? Array.from({ length: count }, (_, i) => ({
            abi: activityTokenAbi,
            address,
            functionName: "getAttestationAtIndex" as const,
            args: [BigInt(i)],
          }))
        : [],
    [address, count]
  );

  const listQuery = useReadContracts({
    contracts: attestationCalls,
    allowFailure: false,
    query: { enabled: Boolean(address) && count > 0 },
  });

  const data: AttestationItem[] | undefined = useMemo(() => {
    const results = listQuery.data as
      | Array<{
          attester: `0x${string}`;
          bump: number;
          data: `0x${string}`;
          expirationTime: bigint;
          recipient: `0x${string}`;
          refUID: `0x${string}`;
          revocable: boolean;
          revocationTime: bigint;
          schema: `0x${string}`;
          time: bigint;
          uid: `0x${string}`;
        }>
      | undefined;
    if (!results) return undefined;
    return results.map((r) => {
      const recipient = r.recipient as string;
      const issuer = r.attester as string;
      const bytesData = r.data as `0x${string}`;
      const { badgeName, justification } = tryDecodeBadgeData(bytesData);
      return {
        issuer,
        recipient,
        badgeName,
        attestationJustification: justification,
      };
    });
  }, [listQuery.data]);

  const isLoading = countQuery.isLoading || listQuery.isLoading;
  const error =
    (countQuery.error as Error | null) ||
    (listQuery.error as Error | null) ||
    null;

  return { data, isLoading, error, refetch: countQuery.refetch };
}
