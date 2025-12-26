import { describe, it, expect } from "vitest";
import {
  buildCreateBadgeArgs,
  stringToBytes32,
  stringToBytes,
} from "@/lib/utils/blockchainUtils";

describe("buildCreateBadgeArgs", () => {
  const nameBytes = stringToBytes32("TestBadge");

  it("builds V2 args with bytes description", () => {
    const args = buildCreateBadgeArgs(nameBytes, "Long description", "v2");
    expect(args).toHaveLength(2);
    expect(args[0]).toBe(nameBytes);
    // V2 uses stringToBytes (variable length)
    expect(args[1]).toBe(stringToBytes("Long description"));
  });

  it("builds V1 args with bytes32 description", () => {
    const args = buildCreateBadgeArgs(nameBytes, "Short", "v1");
    expect(args).toHaveLength(2);
    expect(args[0]).toBe(nameBytes);
    // V1 uses stringToBytes32 (fixed 32 bytes)
    expect(args[1]).toBe(stringToBytes32("Short"));
  });

  it("handles empty description for V2", () => {
    const args = buildCreateBadgeArgs(nameBytes, "", "v2");
    expect(args[1]).toBe(stringToBytes(""));
  });

  it("handles empty description for V1", () => {
    const args = buildCreateBadgeArgs(nameBytes, "", "v1");
    expect(args[1]).toBe(stringToBytes32(""));
  });
});

