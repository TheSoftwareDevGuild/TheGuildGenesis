import { useMutation, type UseMutationResult, useQueryClient } from "@tanstack/react-query";
import { useAccount } from "wagmi";
import type {
  UpdateProfileInput,
  UpdateProfileResponse,
} from "@/lib/types/api";
import { API_BASE_URL } from "@/lib/constants/apiConstants";

async function putUpdateProfile(
  address: string,
  body: UpdateProfileInput,
  signerAddress: string,
  signature: string
): Promise<UpdateProfileResponse> {
  const response = await fetch(`${API_BASE_URL}/profiles/${address}`, {
    method: "PUT",
    headers: {
      "Content-Type": "application/json",
      "x-eth-address": signerAddress,
      "x-eth-signature": signature,
    },
    body: JSON.stringify(body),
  });

  if (!response.ok) {
    const text = await response.text().catch(() => "");
    throw new Error(
      `Failed to update profile: ${response.status} ${response.statusText}${
        text ? ` - ${text}` : ""
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
      return putUpdateProfile(address, input, address, signature);
    },
    onSuccess: () => {
      // Invalidate nonce query since it was incremented
      queryClient.invalidateQueries({ queryKey: ["nonce", address] });
    },
  });
}
