/**
 * ABI version detection utilities for BadgeRegistry contracts.
 * 
 * These functions detect contract version (V1 vs V2) by analyzing error messages
 * when attempting to call with the wrong ABI. This is a temporary solution during
 * the V1→V2 migration period.
 * 
 * Limitations:
 * - Error-based detection is fragile: error message formats may vary across RPC providers
 * - Network/RPC errors must be carefully filtered to avoid false positives
 * - This approach will be removed after V2 full deployment (see TODO comments)
 * 
 * Why error-based detection:
 * - No version getter exists on-chain
 * - On-demand detection avoids refetch storms from version probes in hooks
 * - Works for both read (decode errors) and write (selector errors) operations
 */

/**
 * TODO(cleanup): Remove decode error detection after V2 full deployment
 * Check if error is a decode error (ABI mismatch).
 * V2 ABI will fail to decode V1 contract responses with decode errors.
 * Must NOT classify RPC/network errors as decode errors.
 * 
 * NOTE: Matching logic is copied exactly from use-create-badge.ts and use-get-badges.ts.
 * Do not modify matching patterns in this PR - only relocate and document.
 */
export function isDecodeError(error: Error | null): boolean {
  if (!error) return false;
  const message = error.message.toLowerCase();
  const name = error.name.toLowerCase();
  // Only detect actual decode errors, not RPC/network issues
  return (
    name.includes("positionoutofbounds") ||
    name.includes("decodefunctionresult") ||
    (name.includes("contractfunctionexecution") &&
      (message.includes("decode") || message.includes("position"))) ||
    (message.includes("decode") &&
      (message.includes("function") || message.includes("abi"))) ||
    (message.includes("position") && message.includes("out of bounds"))
  );
}

/**
 * TODO(cleanup): Remove function selector error detection after V2 full deployment
 * Check if error indicates function selector not found (ABI mismatch for write operations).
 * When simulating a write with wrong ABI, we get "function selector not found" errors.
 * Must NOT classify other revert reasons (EMPTY_NAME, DUPLICATE_NAME, etc.) as selector errors.
 * 
 * NOTE: Matching logic is copied exactly from use-create-badge.ts.
 * Do not modify matching patterns in this PR - only relocate and document.
 */
export function isFunctionSelectorError(error: Error | null): boolean {
  if (!error) return false;
  const message = error.message.toLowerCase();
  const name = error.name.toLowerCase();
  // Detect function selector not found errors
  return (
    name.includes("functionnotfound") ||
    name.includes("functionselector") ||
    (message.includes("function") &&
      (message.includes("not found") || message.includes("selector"))) ||
    (message.includes("selector") && message.includes("not found"))
  );
}

