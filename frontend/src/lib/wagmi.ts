import { getDefaultConfig } from "@rainbow-me/rainbowkit";
import { http } from "wagmi";
import { polygonAmoy } from "wagmi/chains";

export function getWagmiConfig() {
  const projectId = import.meta.env.PUBLIC_WALLET_CONNECT_PROJECT_ID as
    | string
    | undefined;

  // In tests or dev without a project id, the consumer can handle an empty string.
  return getDefaultConfig({
    appName: "The Guild Genesis",
    projectId: projectId ?? "",
    chains: [polygonAmoy],
    ssr: false,
    syncConnectedChain: true,
    transports: {
      [polygonAmoy.id]: http(),
    },
  });
}
