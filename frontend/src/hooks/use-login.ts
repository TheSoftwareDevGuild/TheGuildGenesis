import { useAccount, useSignMessage } from "wagmi";
import { useGetNonce } from "./profiles/use-get-nonce";
import { generateSiweMessage } from "@/lib/utils/siwe";
import { saveToken } from "@/lib/utils/jwt";
import { API_BASE_URL } from "@/lib/constants/apiConstants";
import { useCallback, useState } from "react";

interface LoginResponse {
    token: string;
    address: string;
}

export function useLogin() {
    const { address } = useAccount();
    const { signMessageAsync } = useSignMessage();
    const { data: nonceData } = useGetNonce(address);
    const [isLoading, setIsLoading] = useState(false);
    const [error, setError] = useState<Error | null>(null);

    const login = useCallback(async () => {
        try {
            setIsLoading(true);
            setError(null);

            if (!address) {
                throw new Error("Wallet not connected");
            }

            if (!nonceData) {
                throw new Error("Nonce not available");
            }

            // Generate SIWE message and sign it
            const siweMessage = generateSiweMessage(nonceData);
            const signature = await signMessageAsync({ message: siweMessage });

            // Send signature to backend to get JWT token
            const response = await fetch(`${API_BASE_URL}/auth/login`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                    "x-eth-address": address,
                    "x-eth-signature": signature,
                },
            });

            if (!response.ok) {
                const text = await response.text().catch(() => "");
                throw new Error(
                    `Login failed: ${response.status} ${response.statusText}${text ? ` - ${text}` : ""
                    }`
                );
            }

            const data = (await response.json()) as LoginResponse;
            saveToken(data);

            return data;
        } catch (err) {
            const error = err instanceof Error ? err : new Error("Unknown error");
            setError(error);
            throw error;
        } finally {
            setIsLoading(false);
        }
    }, [address, nonceData, signMessageAsync]);

    return { login, isLoading, error };
}
