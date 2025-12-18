import { getToken, clearToken, saveToken } from "./jwt";
import { API_BASE_URL } from "@/lib/constants/apiConstants";

interface FetchOptions extends RequestInit {
    skipAuth?: boolean;
    retryOn401?: boolean;
}

/**
 * Enhanced fetch with JWT support and 401 handling
 * On 401, attempts to refresh token and retry the request
 */
export async function fetchWithJWT(
    url: string,
    options: FetchOptions = {},
    refreshTokenFn?: () => Promise<boolean>
): Promise<Response> {
    const { skipAuth = false, retryOn401 = true, ...fetchOptions } = options;

    // Add JWT token if available and not skipping auth
    if (!skipAuth) {
        const token = getToken();
        if (token) {
            const headers = new Headers(fetchOptions.headers);
            headers.set("Authorization", `Bearer ${token}`);
            fetchOptions.headers = headers;
        }
    }

    let response = await fetch(url, fetchOptions);

    // Handle 401: try to refresh token and retry
    if (response.status === 401 && retryOn401 && refreshTokenFn) {
        const refreshed = await refreshTokenFn();
        if (refreshed) {
            // Retry the request with new token
            const token = getToken();
            if (token) {
                const headers = new Headers(fetchOptions.headers);
                headers.set("Authorization", `Bearer ${token}`);
                fetchOptions.headers = headers;
            }
            response = await fetch(url, fetchOptions);
        }
    }

    return response;
}
