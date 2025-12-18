import { useMutation, type UseMutationResult, useQueryClient } from "@tanstack/react-query";
import { useAccount } from "wagmi";
import type {
  UpdateProfileInput,
  UpdateProfileResponse,
} from "@/lib/types/api";
import { API_BASE_URL } from "@/lib/constants/apiConstants";
import { getToken } from "@/lib/utils/jwt";

async function putUpdateProfile(
  address: string,
  body: UpdateProfileInput,
  signerAddress: string,
  signature: string,
  token?: string
): Promise<UpdateProfileResponse> {
  const headers: HeadersInit = {
    "Content-Type": "application/json",
  };

  // Use JWT token if available, otherwise use SIWE signature
  if (token) {
    headers["Authorization"] = `Bearer ${token}`;
  } else {
    headers["x-eth-address"] = signerAddress;
    headers["x-eth-signature"] = signature;
  }

  const response = await fetch(`${API_BASE_URL}/profiles/${address}`, {
    method: "PUT",
    headers,
    body: JSON.stringify(body),
  });

  if (!response.ok) {
    const text = await response.text().catch(() => "");
    throw new Error(
      `Failed to update profile: ${response.status} ${response.statusText}${text ? ` - ${text}` : ""
      }`
    );
  }

  try {
    return (await response.json()) as UpdateProfileResponse;
  } catch {
    return {} as UpdateProfileResponse;
  }
}

type MutationVariables = {
  input: UpdateProfileInput;
  signature: string;
};

export function useUpdateProfile(): UseMutationResult<
  UpdateProfileResponse,
  Error,
  MutationVariables
> {
  const { address } = useAccount();
  const queryClient = useQueryClient();

  return useMutation<UpdateProfileResponse, Error, MutationVariables>({
    mutationKey: ["update-profile"],
    mutationFn: async ({ input, signature }) => {
      if (!address) throw new Error("Wallet not connected");
      const token = getToken();
      return putUpdateProfile(address, input, address, signature, token || undefined);
    },
    onSuccess: () => {
      // Invalidate nonce query since it was incremented
      queryClient.invalidateQueries({ queryKey: ["nonce", address] });
    },
  });
}
