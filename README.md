# The Guild Genesis

A simple Web3 dapp for awarding badges to users via smart contracts. Built with Astro and vanilla JavaScript.

## Features

- **Wallet Connection** - Connect MetaMask or other Web3 wallets
- **Badge Awarding** - Award badges to other users by calling smart contracts
- **Clean UI** - Simple, responsive interface
- **No Database** - All data stored on-chain via smart contracts

## Quick Start

1. **Install dependencies:**
   ```bash
   npm install
   ```

2. **Start development server:**
   ```bash
   npm run dev
   ```

3. **Open in browser:**
   ```
   http://localhost:4321
   ```

4. **Connect your wallet** and start awarding badges!

## Available Scripts

- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm run preview` - Preview production build
- `npm run test` - Run tests
- `npm run lint` - Lint code
- `npm run format` - Format code
- `npm run clean` - Clean build artifacts

## Project Structure

```
/
├── src/
│   ├── components/     # React components (if needed)
│   ├── layouts/        # Astro layouts
│   ├── pages/          # Astro pages
│   │   └── index.astro # Main dapp page
│   └── styles/         # Global styles
├── public/             # Static assets
└── package.json        # Dependencies and scripts
```

## How It Works

1. **Connect Wallet** - Click "Connect Wallet" to connect MetaMask
2. **Award Badges** - Fill in recipient address, select badge type, add message
3. **Smart Contract** - Badge data is stored on-chain (currently simulated)

## Badge Types

- 👨‍💻 **Contributor** - For active code contributions
- 🎓 **Mentor** - For helping others learn
- 💡 **Innovator** - For creative solutions
- 👑 **Leader** - For project leadership

## Development

This is a simple Astro project with vanilla JavaScript for Web3 functionality. No complex build tools or databases needed.

### Adding Smart Contract Integration

To integrate with real smart contracts:

1. Add contract ABI and address
2. Replace the simulation in the form submission handler
3. Use libraries like ethers.js or web3.js for contract calls

### Customization

- Modify badge types in the HTML
- Update styling in the `<style>` section
- Add more Web3 functionality in the JavaScript section

## Requirements

- Node.js 18+
- MetaMask or Web3 wallet
- Modern browser with Web3 support

## License

MIT