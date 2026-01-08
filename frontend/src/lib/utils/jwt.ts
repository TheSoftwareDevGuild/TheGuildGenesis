const JWT_TOKEN_KEY = "jwt_token";
const JWT_ADDRESS_KEY = "jwt_address";
const JWT_EXPIRY_KEY = "jwt_expiry";

export interface JwtToken {
    token: string;
    address: string;
}

export function saveToken(token: JwtToken, expiresInSeconds = 86400): void {
    const expiryTime = Date.now() + expiresInSeconds * 1000;
    localStorage.setItem(JWT_TOKEN_KEY, token.token);
    localStorage.setItem(JWT_ADDRESS_KEY, token.address);
    localStorage.setItem(JWT_EXPIRY_KEY, expiryTime.toString());
}

export function getToken(): string | null {
    if (!isTokenValid()) {
        clearToken();
        return null;
    }
    return localStorage.getItem(JWT_TOKEN_KEY);
}

export function getAddress(): string | null {
    return localStorage.getItem(JWT_ADDRESS_KEY);
}

export function clearToken(): void {
    localStorage.removeItem(JWT_TOKEN_KEY);
    localStorage.removeItem(JWT_ADDRESS_KEY);
    localStorage.removeItem(JWT_EXPIRY_KEY);
}

export function isTokenValid(): boolean {
    const token = localStorage.getItem(JWT_TOKEN_KEY);
    const expiryStr = localStorage.getItem(JWT_EXPIRY_KEY);

    if (!token || !expiryStr) {
        return false;
    }

    const expiryTime = parseInt(expiryStr, 10);
    const isExpired = Date.now() >= expiryTime;

    if (isExpired) {
        clearToken();
        return false;
    }

    return true;
}