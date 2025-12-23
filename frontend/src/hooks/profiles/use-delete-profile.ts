import {
  useMutation,
  type UseMutationResult,
  useQueryClient,
} from "@tanstack/react-query";
import { useAccount } from "wagmi";
import type {
  DeleteProfileInput,
  DeleteProfileResponse,
} from "@/lib/types/api";
import { API_BASE_URL } from "@/lib/constants/apiConstants";
import { isTokenValid, getToken } from "@/lib/utils/jwt";
import { useLogin } from "@/hooks/use-login";

async function deleteProfile(
  address: string,
  token: string
): Promise<DeleteProfileResponse> {
  const response = await fetch(`${API_BASE_URL}/profiles/${address}`, {
    method: "DELETE",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${token}`,
    },
  });

  if (!response.ok) {
    const text = await response.text().catch(() => "");
    throw new Error(
      `Failed to delete profile: ${response.status} ${response.statusText}${text ? ` - ${text}` : ""
      }`
    );
  }

  try {
    return (await response.json()) as DeleteProfileResponse;
  } catch {
    return {} as DeleteProfileResponse;
  }
}

export function useDeleteProfile(): UseMutationResult<
  DeleteProfileResponse,
  Error,
  void
> {
  const { address } = useAccount();
  const queryClient = useQueryClient();
  const { login } = useLogin();

  return useMutation<DeleteProfileResponse, Error, void>({
    mutationKey: ["delete-profile"],
    mutationFn: async () => {
      if (!address) throw new Error("Wallet not connected");

      // Check if token is valid, if not trigger login
      if (!isTokenValid()) {
        await login();
      }

      const token = getToken();
      if (!token) {
        throw new Error("Authentication required. Please sign in.");
      }

      return deleteProfile(address, token);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["profiles"] });
    },
  });
}
