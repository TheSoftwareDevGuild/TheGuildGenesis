import { render, screen } from "@testing-library/react";
import { describe, it, expect, vi } from "vitest";
import { ProfileCard } from "../components/profiles/list/ProfileCard";
import React from "react";

// Mock window.matchMedia
Object.defineProperty(window, "matchMedia", {
  writable: true,
  value: vi.fn().mockImplementation((query) => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: vi.fn(), // deprecated
    removeListener: vi.fn(), // deprecated
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    dispatchEvent: vi.fn(),
  })),
});

// Mock wagmi
vi.mock("wagmi", () => ({
  WagmiProvider: ({ children }: { children: React.ReactNode }) => (
    <div>{children}</div>
  ),
  useAccount: () => ({ address: undefined }),
  useSignMessage: () => ({ signMessageAsync: vi.fn() }),
  useWriteContract: () => ({ writeContractAsync: vi.fn() }),
  useWaitForTransactionReceipt: () => ({ data: vi.fn() }),
  useReadContract: () => ({ data: undefined }),
  useReadContracts: () => ({ data: undefined }),
  useConfig: () => ({}),
}));

// Mock RainbowKit
vi.mock("@rainbow-me/rainbowkit", () => ({
  ConnectButton: () => <div data-testid="connect-button">Connect Wallet</div>,
  RainbowKitProvider: ({ children }: { children: React.ReactNode }) => (
    <div>{children}</div>
  ),
  getDefaultConfig: () => ({}),
}));

// Mock the wagmi config getter
vi.mock("@/lib/wagmi", () => ({
  getWagmiConfig: () => ({}),
}));

describe("ProfileCard", () => {
  it("renders minimal profile and badge count", () => {
    render(
      <ProfileCard
        address="0x1234567890123456789012345678901234567890"
        attestationCount={2}
        attestations={[
          {
            id: "1",
            badgeName: "Rust",
            justification: "Rust programming",
            issuer: "0xabcd",
          },
          {
            id: "2",
            badgeName: "React",
            justification: "React development",
            issuer: "0xefgh",
          },
        ]}
      />
    );

    // Name falls back to truncated address, and the address appears in description line too
    expect(screen.getAllByText("0x1234...7890").length).toBeGreaterThan(0);
    expect(screen.getByText("2 attestations")).toBeInTheDocument();
    expect(screen.getByText("Rust")).toBeInTheDocument();
    expect(screen.getByText("React")).toBeInTheDocument();
  });
});
