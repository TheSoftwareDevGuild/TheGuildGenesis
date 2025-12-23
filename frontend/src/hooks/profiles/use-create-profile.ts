import {
  useMutation,
  type UseMutationResult,
  useQueryClient,
} from "@tanstack/react-query";
import { useAccount } from "wagmi";
import type {
  CreateProfileInput,
  CreateProfileResponse,
} from "@/lib/types/api";
import { API_BASE_URL } from "@/lib/constants/apiConstants";
import { isTokenValid, getToken } from "@/lib/utils/jwt";
import { useLogin } from "@/hooks/use-login";

async function postCreateProfile(
  input: CreateProfileInput,
  token: string
): Promise<CreateProfileResponse> {
  const response = await fetch(`${API_BASE_URL}/profiles/`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify(input),
  });

  if (!response.ok) {
    const text = await response.text().catch(() => "");
    throw new Error(
      `Failed to create profile: ${response.status} ${response.statusText}${text ? ` - ${text}` : ""
      }`
    );
  }

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
  const queryClient = useQueryClient();
  const { login } = useLogin();

  return useMutation<CreateProfileResponse, Error, MutationVariables>({
    mutationKey: ["create-profile"],
    mutationFn: async ({ input }) => {
      if (!address) {
        throw new Error("Wallet not connected");
      }

      // Check if token is valid, if not trigger login
      if (!isTokenValid()) {
        await login();
      }

      const token = getToken();
      if (!token) {
        throw new Error("Authentication required. Please sign in.");
      }

      return postCreateProfile(input, token);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["profiles"] });
    },
  });
}
