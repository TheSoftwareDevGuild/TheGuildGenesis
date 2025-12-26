import { describe, it, expect } from "vitest";
import {
  isDecodeError,
  isFunctionSelectorError,
} from "@/lib/utils/abiDetection";

describe("isDecodeError", () => {
  it("returns false for null/undefined", () => {
    expect(isDecodeError(null)).toBe(false);
  });

  it("detects PositionOutOfBounds errors", () => {
    const error = new Error("Something went wrong");
    error.name = "PositionOutOfBoundsError";
    expect(isDecodeError(error)).toBe(true);
  });

  it("detects DecodeFunctionResult errors", () => {
    const error = new Error("Decode failed");
    error.name = "DecodeFunctionResultError";
    expect(isDecodeError(error)).toBe(true);
  });

  it("detects decode errors in message", () => {
    const error = new Error("Failed to decode function result");
    expect(isDecodeError(error)).toBe(true);
  });

  it("detects position out of bounds in message", () => {
    const error = new Error("Position out of bounds");
    expect(isDecodeError(error)).toBe(true);
  });

  it("does not classify RPC errors as decode errors", () => {
    const error = new Error("Internal JSON-RPC error");
    expect(isDecodeError(error)).toBe(false);
  });

  it("does not classify network errors as decode errors", () => {
    const error = new Error("Network request failed");
    expect(isDecodeError(error)).toBe(false);
  });

  it("detects contract function execution decode errors", () => {
    const error = new Error("Contract function execution failed: decode error");
    error.name = "ContractFunctionExecutionError";
    expect(isDecodeError(error)).toBe(true);
  });
});

describe("isFunctionSelectorError", () => {
  it("returns false for null/undefined", () => {
    expect(isFunctionSelectorError(null)).toBe(false);
  });

  it("detects FunctionNotFound errors", () => {
    const error = new Error("Function not found");
    error.name = "FunctionNotFoundError";
    expect(isFunctionSelectorError(error)).toBe(true);
  });

  it("detects FunctionSelector errors", () => {
    const error = new Error("Selector mismatch");
    error.name = "FunctionSelectorError";
    expect(isFunctionSelectorError(error)).toBe(true);
  });

  it("detects 'function not found' in message", () => {
    const error = new Error("Function not found in ABI");
    expect(isFunctionSelectorError(error)).toBe(true);
  });

  it("detects 'selector not found' in message", () => {
    const error = new Error("Selector not found");
    expect(isFunctionSelectorError(error)).toBe(true);
  });

  it("does not classify revert reasons as selector errors", () => {
    const error = new Error("Contract function reverted: EMPTY_NAME");
    expect(isFunctionSelectorError(error)).toBe(false);
  });

  it("does not classify duplicate name errors as selector errors", () => {
    const error = new Error("Contract function reverted: DUPLICATE_NAME");
    expect(isFunctionSelectorError(error)).toBe(false);
  });
});

