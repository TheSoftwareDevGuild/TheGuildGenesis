import {
  useMutation,
  type UseMutationResult,
  useQueryClient,
} from "@tanstack/react-query";
import { useAccount, useSignMessage } from "wagmi";
import type {
  CreateProfileInput,
  CreateProfileResponse,
} from "@/lib/types/api";
import { API_BASE_URL } from "@/lib/constants/apiConstants";
import { getToken } from "@/lib/utils/jwt";

async function postCreateProfile(
  input: CreateProfileInput,
  address: string,
  signature: string,
  token?: string
): Promise<CreateProfileResponse> {
  const headers: HeadersInit = {
    "Content-Type": "application/json",
  };

  // Use JWT token if available, otherwise use SIWE signature
  if (token) {
    headers["Authorization"] = `Bearer ${token}`;
  } else {
    headers["x-eth-address"] = address;
    headers["x-eth-signature"] = signature;
  }

  const response = await fetch(`${API_BASE_URL}/profiles/`, {
    method: "POST",
    headers,
    body: JSON.stringify(input),
  });

  if (!response.ok) {
    const text = await response.text().catch(() => "");
    throw new Error(
      `Failed to create profile: ${response.status} ${response.statusText}${text ? ` - ${text}` : ""
      }`
    );
  }

  // Attempt to parse JSON, but allow empty body
  try {
    return (await response.json()) as CreateProfileResponse;
  } catch {
    return {} as CreateProfileResponse;
  }
}

type MutationVariables = {
  input: CreateProfileInput;
  signature: string;
};

export function useCreateProfile(): UseMutationResult<
  CreateProfileResponse,
  Error,
  MutationVariables
> {
  const { address } = useAccount();
  const queryClient = useQueryClient();

  return useMutation<CreateProfileResponse, Error, MutationVariables>({
    mutationKey: ["create-profile"],
    mutationFn: async ({ input, signature }) => {
      if (!address) {
        throw new Error("Wallet not connected");
      }
      const token = getToken();
      return postCreateProfile(input, address, signature, token || undefined);
    },
    onSuccess: () => {
      // Invalidate nonce query since it was incremented
      queryClient.invalidateQueries({ queryKey: ["nonce", address] });
    },
  });
}
