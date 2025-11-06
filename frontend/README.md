# Guild Genesis Frontend

Astro + React frontend with Web3 integration for The Guild Genesis project.

## Prerequisites

- [Node.js](https://nodejs.org/) (v18 or higher)
- [Astro CLI](https://docs.astro.build/en/getting-started/)

Install Astro CLI globally:
```bash
npm install -g @astrojs/cli
```

## 🚀 Project Structure

```text
frontend/
├── public/
│   ├── favicon.svg
│   └── logo_animated.svg
├── src/
│   ├── components/     # React components
│   ├── hooks/         # Custom React hooks
│   ├── layouts/       # Astro layouts
│   ├── lib/           # Utility functions
│   ├── pages/         # Astro pages
│   └── styles/        # Global styles
├── astro.config.mjs
└── package.json
```

## 🧞 Commands

All commands are run from the `frontend/` directory:

| Command                   | Action                                           |
| :------------------------ | :----------------------------------------------- |
| `npm install`             | Installs dependencies                            |
| `npm run dev`             | Starts local dev server at `localhost:4321`      |
| `npm run build`           | Build your production site to `./dist/`          |
| `npm run preview`         | Preview your build locally, before deploying     |
| `npm run astro ...`       | Run CLI commands like `astro add`, `astro check` |

## Environment Variables

Make sure to set up your `.env` file in the project root with:

```bash
PUBLIC_WALLET_CONNECT_PROJECT_ID=your_wallet_connect_project_id
PUBLIC_API_URL=http://localhost:3001
PUBLIC_BADGE_REGISTRY_ADDRESS=0x...
PUBLIC_EAS_CONTRACT_ADDRESS=0x...
# ERC20 token used for balances/transfers
PUBLIC_ACTIVITY_TOKEN_ADDRESS=0x...
# EAS resolver used to read attestation list
PUBLIC_ATTESTATION_RESOLVER_ADDRESS=0x...
PUBLIC_SCHEMA_ID=0x...
```

## Tech Stack

- **Astro** - Fast static site generator with React islands
- **React** - For interactive Web3 components
- **Tailwind CSS** - Utility-first CSS framework
- **wagmi** - React hooks for Ethereum
- **viem** - TypeScript interface for Ethereum
- **RainbowKit** - Wallet connection UI
- **TanStack Query** - Data fetching and caching
- **TanStack Router** - Type-safe routing

## Development

1. Install dependencies:
   ```bash
   npm install
   ```

2. Start the development server:
   ```bash
   npm run dev
   ```

3. Open [http://localhost:4321](http://localhost:4321) in your browser

## Learn More

- [Astro Documentation](https://docs.astro.build)
- [React Documentation](https://react.dev)
- [Tailwind CSS Documentation](https://tailwindcss.com/docs)
