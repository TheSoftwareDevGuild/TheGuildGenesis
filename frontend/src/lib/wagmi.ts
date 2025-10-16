import { getDefaultConfig } from "@rainbow-me/rainbowkit";
import { http } from "wagmi";
import { polygonAmoy } from "wagmi/chains";

const projectId = import.meta.env.PUBLIC_WALLET_CONNECT_PROJECT_ID as
  | string
  | undefined;

export const config = getDefaultConfig({
  appName: "The Guild Genesis",
  projectId: projectId ?? "",
  chains: [polygonAmoy],
  ssr: false,
  syncConnectedChain: true,
  transports: {
    [polygonAmoy.id]: http(),
  },
});
