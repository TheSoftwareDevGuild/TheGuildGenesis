import { render, screen } from "@testing-library/react";
import { describe, it, expect, vi } from "vitest";
import { AppWrapper } from "@/components/AppWrapper";
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
}));

// Mock RainbowKit
vi.mock("@rainbow-me/rainbowkit", () => ({
  ConnectButton: () => <div data-testid="connect-button">Connect Wallet</div>,
  RainbowKitProvider: ({ children }: { children: React.ReactNode }) => (
    <div>{children}</div>
  ),
  getDefaultConfig: () => ({}),
}));

// Mock the wagmi config
vi.mock("@/lib/wagmi", () => ({
  config: {},
}));

describe("AppWrapper", () => {
  it("renders title and connect button", () => {
    render(
      <AppWrapper>
        <div>Test</div>
      </AppWrapper>
    );
    expect(screen.getByText("The Guild Genesis")).toBeInTheDocument();
    expect(screen.getByTestId("connect-button")).toBeInTheDocument();
  });
});
