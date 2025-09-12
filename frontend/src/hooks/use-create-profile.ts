import { useMutation, type UseMutationResult } from "@tanstack/react-query";
import { useAccount, useSignMessage } from "wagmi";

export type CreateProfileInput = {
  name: string;
  description?: string;
  avatar_url?: string;
  siweMessage: string; // message to sign for auth
};

// Unknown response shape from backend; expose as unknown for consumers to refine
export type CreateProfileResponse = unknown;

const API_BASE_URL: string =
  import.meta.env.PUBLIC_API_URL || "http://0.0.0.0:3001";

async function postCreateProfile(
  input: CreateProfileInput,
  address: string,
  signature: string
): Promise<CreateProfileResponse> {
  const response = await fetch(`${API_BASE_URL}/profiles/`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "x-eth-address": address,
      "x-eth-signature": signature,
      "x-siwe-message": input.siweMessage,
    },
    body: JSON.stringify(input),
  });

  if (!response.ok) {
    const text = await response.text().catch(() => "");
    throw new Error(
      `Failed to create profile: ${response.status} ${response.statusText}${
        text ? ` - ${text}` : ""
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
};

export function useCreateProfile(): UseMutationResult<
  CreateProfileResponse,
  Error,
  MutationVariables
> {
  const { address } = useAccount();
  const { signMessageAsync } = useSignMessage();

  return useMutation<CreateProfileResponse, Error, MutationVariables>({
    mutationKey: ["create-profile"],
    mutationFn: async ({ input }) => {
      if (!address) {
        throw new Error("Wallet not connected");
      }
      const signature = await signMessageAsync({ message: input.siweMessage });
      return postCreateProfile(input, address, signature);
    },
  });
}
