import { getDefaultConfig } from '@rainbow-me/rainbowkit';
import { mainnet, sepolia, arbitrum, arbitrumSepolia } from 'wagmi/chains';

export const config = getDefaultConfig({
  appName: 'The Guild Genesis',
  projectId: 'your-project-id-here', // Replace with your WalletConnect project ID
  chains: [mainnet, sepolia, arbitrum, arbitrumSepolia],
  ssr: false, // If your dApp uses server side rendering (SSR)
});
