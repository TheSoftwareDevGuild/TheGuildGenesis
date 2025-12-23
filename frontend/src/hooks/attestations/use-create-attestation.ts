import { useMemo, useRef } from "react";
import {
  useWriteContract,
  useWaitForTransactionReceipt,
  useConfig,
  useAccount,
} from "wagmi";
import { simulateContract } from "@wagmi/core";
import { encodeAbiParameters } from "viem";
import { easAbi } from "@/lib/abis/easAbi";
import {
  EAS_CONTRACT_ADDRESS,
  SCHEMA_ID,
} from "@/lib/constants/blockchainConstants";
import { stringToBytes32, stringToBytes } from "@/lib/utils/blockchainUtils";

function encodeBadgeData(
  badgeName: `0x${string}`,
  justificationBytes: `0x${string}`
) {
  // Encode according to schema: bytes32 badgeName, bytes justification
  return encodeAbiParameters(
    [{ type: "bytes32" }, { type: "bytes" }],
    [badgeName, justificationBytes]
  );
}

export function useCreateAttestation() {
  const config = useConfig();
  const { address: account, chainId } = useAccount();
  const {
    writeContractAsync,
    data: hash,
    isPending,
    error,
    reset,
  } = useWriteContract();
  const isBusyRef = useRef(false);

  const createAttestation = useMemo(() => {
    return async (
      recipient: `0x${string}`,
      badgeName: string,
      justification: string
    ) => {
      if (isBusyRef.current || isPending) {
        throw new Error(
          "Previous attestation is still pending. Please wait..."
        );
      }
      isBusyRef.current = true;
      // Convert strings to bytes32
      const badgeNameBytes = stringToBytes32(badgeName);
      const justificationBytes = stringToBytes(justification);

      // Encode data according to schema
      const encodedData = encodeBadgeData(badgeNameBytes, justificationBytes);
      // 1) Simulate with exact sender/chain to catch reverts and get prepared request
      const simulation = await simulateContract(config, {
        abi: easAbi,
        address: EAS_CONTRACT_ADDRESS,
        functionName: "attest",
        args: [
          {
            schema: SCHEMA_ID,
            data: {
              recipient,
              expirationTime: 0n,
              revocable: true,
              refUID:
                "0x0000000000000000000000000000000000000000000000000000000000000000",
              data: encodedData,
              value: 0n,
            },
          },
        ],
        account: account as `0x${string}` | undefined,
        chainId,
      });

      // 2) Bump gas params slightly and send
      const bump = (v?: bigint) =>
        typeof v === "bigint" ? (v * 12n) / 10n : v;
      const req: any = { ...simulation.request };
      req.gas = bump(req.gas);
      req.maxFeePerGas = bump(req.maxFeePerGas);
      req.maxPriorityFeePerGas = bump(req.maxPriorityFeePerGas);

      try {
        const txHash = await writeContractAsync(req);
        return txHash;
      } finally {
        isBusyRef.current = false;
      }
    };
  }, [account, chainId, config, isPending, writeContractAsync]);

  const wait = useWaitForTransactionReceipt({
    hash: hash as `0x${string}` | undefined,
    confirmations: 6,
    query: { enabled: Boolean(hash) },
  });

  return {
    createAttestation,
    hash,
    isPending,
    error,
    reset,
    isConfirming: wait.isLoading,
    isConfirmed: wait.isSuccess,
    receipt: wait.data,
  };
}
