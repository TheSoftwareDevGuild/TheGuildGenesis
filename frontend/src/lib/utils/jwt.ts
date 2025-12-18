const JWT_TOKEN_KEY = "jwt_token";
const JWT_ADDRESS_KEY = "jwt_address";

export interface JwtToken {
    token: string;
    address: string;
}

export function saveToken(token: JwtToken): void {
    localStorage.setItem(JWT_TOKEN_KEY, token.token);
    localStorage.setItem(JWT_ADDRESS_KEY, token.address);
}

export function getToken(): string | null {
    return localStorage.getItem(JWT_TOKEN_KEY);
}

export function getAddress(): string | null {
    return localStorage.getItem(JWT_ADDRESS_KEY);
}

export function clearToken(): void {
    localStorage.removeItem(JWT_TOKEN_KEY);
    localStorage.removeItem(JWT_ADDRESS_KEY);
}

export function isTokenValid(): boolean {
    return getToken() !== null;
}
