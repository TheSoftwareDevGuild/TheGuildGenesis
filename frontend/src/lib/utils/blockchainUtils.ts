export function bytes32ToString(value: `0x${string}`): string {
  try {
    // viem utils: to strip null bytes and decode
    const bytes = new TextDecoder();
    // fallback simple decode by removing trailing zeros and interpreting as utf8
    // Convert hex to Uint8Array
    const hex = value.startsWith("0x") ? value.slice(2) : value;
    const arr = new Uint8Array(hex.length / 2);
    for (let i = 0; i < arr.length; i++) {
      arr[i] = parseInt(hex.slice(i * 2, i * 2 + 2), 16);
    }
    // Trim trailing zeros
    let end = arr.length;
    while (end > 0 && arr[end - 1] === 0) end--;
    return bytes.decode(arr.subarray(0, end));
  } catch (_e) {
    return "";
  }
}

export function bytesToString(value: `0x${string}`): string {
  try {
    const hex = value.startsWith("0x") ? value.slice(2) : value;
    const arr = new Uint8Array(hex.length / 2);
    for (let i = 0; i < arr.length; i++) {
      arr[i] = parseInt(hex.slice(i * 2, i * 2 + 2), 16);
    }
    return new TextDecoder().decode(arr);
  } catch (_e) {
    return "";
  }
}

export function stringToBytes32(value: string): `0x${string}` {
  // Encode to utf8, pad/truncate to 32 bytes, return as hex
  const encoder = new TextEncoder();
  const bytes = encoder.encode(value);
  const out = new Uint8Array(32);
  const len = Math.min(32, bytes.length);
  for (let i = 0; i < len; i++) out[i] = bytes[i];
  // Convert to hex
  let hex = "0x";
  for (let i = 0; i < out.length; i++) {
    const h = out[i].toString(16).padStart(2, "0");
    hex += h;
  }
  return hex as `0x${string}`;
}
