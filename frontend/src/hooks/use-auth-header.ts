import { useCallback, useState } from "react";
import { useAccount, useSignMessage } from "wagmi";
import { getToken, saveToken, clearToken, isTokenValid } from "@/lib/utils/jwt";
import { useGetNonce } from "./profiles/use-get-nonce";
import { generateSiweMessage } from "@/lib/utils/siwe";
import { API_BASE_URL } from "@/lib/constants/apiConstants";

interface AuthHeaders {
    [key: string]: string;
}

export function useAuthHeader() {
    const { address } = useAccount();
    const { signMessageAsync } = useSignMessage();
    const { data: nonceData } = useGetNonce(address);
    const [isRefreshing, setIsRefreshing] = useState(false);

    const getAuthHeaders = useCallback((): AuthHeaders => {
        // Check if token is valid and not expired
        if (isTokenValid()) {
            const token = getToken();
            if (token) {
                return {
                    Authorization: `Bearer ${token}`,
                };
            }
        }

        // Fall back to empty headers (caller will add SIWE headers)
        return {};
    }, []);

    const refreshToken = useCallback(async (): Promise<boolean> => {
        try {
            setIsRefreshing(true);

            if (!address || !nonceData) {
                return false;
            }

            // Sign a new message to get a new token
            const siweMessage = generateSiweMessage(nonceData);
            const signature = await signMessageAsync({ message: siweMessage });

            // Get new JWT token
            const response = await fetch(`${API_BASE_URL}/auth/login`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                    "x-eth-address": address,
                    "x-eth-signature": signature,
                },
            });

            if (!response.ok) {
                clearToken();
                return false;
            }

            const data = (await response.json()) as {
                token: string;
                address: string;
            };
            saveToken(data);
            return true;
        } catch {
            clearToken();
            return false;
        } finally {
            setIsRefreshing(false);
        }
    }, [address, nonceData, signMessageAsync]);

    return {
        getAuthHeaders,
        refreshToken,
        isRefreshing,
        hasToken: isTokenValid,
    };
}
