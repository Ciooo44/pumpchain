# 🚀 PumpChain

**The fastest SVM-powered chain for DeFi and Gaming.**

[![Testnet Live](https://img.shields.io/badge/Testnet-Live-00FFA3?style=for-the-badge)](https://pumpchain.vercel.app)
[![Website](https://img.shields.io/badge/Website-pumpchain.org-03E1FF?style=for-the-badge)](https://pumpchain.vercel.app)
[![Docs](https://img.shields.io/badge/Docs-Read-9945FF?style=for-the-badge)](./docs/whitepaper.md)

PumpChain is a high-performance Layer 2 blockchain built on the Solana Virtual Machine (SVM), optimized for token launches, DeFi applications, and gaming economies.

## 🧪 Testnet Live

| Service | URL | Status |
|---------|-----|--------|
| **Website** | [pumpchain.vercel.app](https://pumpchain.vercel.app) | 🟢 Online |
| **RPC** | `https://testnet.pumpchain.io` | 🟢 Online |
| **Explorer** | [testnet.pumpscan.app](https://testnet.pumpscan.app) | 🟢 Online |
| **Faucet** | [faucet.pumpchain.io](https://faucet.pumpchain.io) | 🟢 Online |

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     PUMPCHAIN L2                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   Launchpad  │  │   PumpProof  │  │     DEX      │      │
│  │  (Tokens)    │  │  (Anchoring) │  │   (AMM)      │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │         PumpChain Validator (SVM + Consensus)       │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼ Data Availability
                  ┌───────────────────────────────┐
                  │      Solana Mainnet (L1)      │
                  │   - State Proofs Anchored     │
                  │   - Finality & Settlement     │
                  └───────────────────────────────┘
```

---

## 📁 Monorepo Structure

```
pumpchain/
├── validator/          # SVM-based validator node (Rust)
├── launchpad/          # Token launchpad with instant AMM (Rust/Solana)
├── pump-proof/         # State anchoring to Solana L1 (Rust)
├── sdk/                # Developer SDKs
│   ├── csharp/         # Unity/Gaming SDK
│   └── typescript/     # Web/Node.js SDK
├── frontend/           # Web interface + RPC terminal
└── docs/               # Documentation & whitepaper
```

---

## 🚀 Quick Start (Testnet)

### 1. Connect Wallet

**Backpack Wallet:**
```
Settings → Solana → RPC Connection → Custom
RPC URL: https://testnet.pumpchain.io
```

**Phantom Wallet:**
```
Settings → Developer Settings → Custom RPC
RPC URL: https://testnet.pumpchain.io
```

### 2. Get Test Tokens

Visit the faucet or use CLI:
```bash
curl -X POST https://faucet.pumpchain.io \
  -d '{"address": "YOUR_WALLET", "amount": 1000}'
```

### 3. Launch Your Token

```bash
# Install PumpChain CLI
npm install -g @pumpchain/cli

# Launch token on testnet
pumpchain launch \
  --name "My Token" \
  --symbol "MYT" \
  --supply 1000000 \
  --network testnet
```

---

## 🏗️ Build from Source

### Prerequisites
- Rust 1.75+
- Node.js 18+
- Solana CLI
- Docker (optional)

### 1. Build Validator
```bash
cd validator
cargo build --release
```

### 2. Deploy Launchpad Program
```bash
cd launchpad
anchor build
anchor deploy
```

### 3. Run Local Testnet
```bash
docker-compose -f docker-compose.testnet.yml up
```

---

## 💡 Key Features

| Feature | Description |
|---------|-------------|
| **⚡ Fast Finality** | Sub-second confirmation times |
| **🎮 Gaming Ready** | C# Unity SDK for game integration |
| **🚀 Instant Launchpad** | Deploy tokens with automatic AMM liquidity |
| **🔒 Secure Anchoring** | State proofs anchored to Solana L1 |
| **💰 Low Fees** | Optimized for high-frequency trading |

---

## 📚 Documentation

- [Whitepaper](./docs/whitepaper.md)
- [Architecture](./docs/architecture.md)
- [API Reference](./docs/api.md)
- [SDK Guide](./docs/sdk.md)

---

## 🤝 Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

---

## 📄 License

MIT License - see [LICENSE](./LICENSE)

---

**Built with ❤️ by 0Xmizuno & PumpChain Team**