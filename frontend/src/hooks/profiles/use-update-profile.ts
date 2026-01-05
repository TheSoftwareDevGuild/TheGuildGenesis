import {
  useMutation,
  type UseMutationResult,
  useQueryClient,
} from "@tanstack/react-query";
import { useAccount } from "wagmi";
import type {
  UpdateProfileInput,
  UpdateProfileResponse,
} from "@/lib/types/api";
import { API_BASE_URL } from "@/lib/constants/apiConstants";
import { isTokenValid, getToken } from "@/lib/utils/jwt";
import { useLogin } from "@/hooks/use-login";

async function putUpdateProfile(
  address: string,
  body: UpdateProfileInput,
  token: string
): Promise<UpdateProfileResponse> {
  const response = await fetch(`${API_BASE_URL}/profiles/${address}`, {
    method: "PUT",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${token}`,
    },
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
};

export function useUpdateProfile(): UseMutationResult<
  UpdateProfileResponse,
  Error,
  MutationVariables
> {
  const { address } = useAccount();
  const queryClient = useQueryClient();
  const { login } = useLogin();

  return useMutation<UpdateProfileResponse, Error, MutationVariables>({
    mutationKey: ["update-profile"],
    mutationFn: async ({ input }) => {
      if (!address) throw new Error("Wallet not connected");

      // Check if token is valid, if not trigger login
      if (!isTokenValid()) {
        await login();
      }

      const token = getToken();
      if (!token) {
        throw new Error("Authentication required. Please sign in.");
      }

      return putUpdateProfile(address, input, token);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["profiles"] });
    },
  });
}
