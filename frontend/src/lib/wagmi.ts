import { getDefaultConfig } from "@rainbow-me/rainbowkit";
import { http } from "wagmi";
import { polygonAmoy } from "wagmi/chains";

const projectId = import.meta.env.PUBLIC_WALLET_CONNECT_PROJECT_ID as
  | string
  | undefined;

// Fallback project ID for development
const fallbackProjectId = "2b3c4d5e6f7g8h9i0j1k2l3m4n5o6p7q";

console.log("WalletConnect Project ID:", projectId || fallbackProjectId);

export const config = getDefaultConfig({
  appName: "The Guild Genesis",
  projectId: projectId || fallbackProjectId,
  chains: [polygonAmoy],
  ssr: false,
  syncConnectedChain: true,
  transports: {
    [polygonAmoy.id]: http(),
  },
});
