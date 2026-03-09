# PumpChain

The fastest SVM-powered L2 blockchain.

50,000+ TPS. Sub-400ms finality. Built for DeFi and gaming.

---

## Quick Start

```bash
npm install @pumpchain/sdk
```

```typescript
import { Connection, PublicKey } from '@pumpchain/web3'

const connection = new Connection('https://pumpchain.io')
const slot = await connection.getSlot()

console.log(`Connected to slot ${slot}`)
```

---

## RPC Endpoints

| Network | URL |
|---------|-----|
| Mainnet | `https://mainnet.pumpchain.io` |
| Devnet | `https://devnet.pumpchain.io` |
| Local | `http://localhost:8899` |

---

## Features

- **50,000+ TPS** — Parallel transaction execution
- **Sub-400ms Finality** — Near-instant confirmation
- **SVM Compatible** — Run Solana programs natively
- **Token Launchpad** — Create and mint tokens instantly
- **Cross-Chain Bridge** — Solana ↔ PumpChain transfers

---

## Architecture

```
┌─────────────────────────────────────┐
│           Application Layer          │
│    (DeFi, Gaming, NFT Marketplaces)  │
├─────────────────────────────────────┤
│           Smart Contracts            │
│       (Rust/Anchor Programs)         │
├─────────────────────────────────────┤
│          PumpChain Runtime           │
│    (SVM Compatible, Parallel Exec)   │
├─────────────────────────────────────┤
│            Consensus                 │
│     (Tower BFT + Proof of History)   │
├─────────────────────────────────────┤
│           Network Layer              │
│       (Gossip + Turbine)             │
└─────────────────────────────────────┘
```

---

## Repositories

| Repo | Description |
|------|-------------|
| `validator` | Rust validator implementation |
| `sdk` | TypeScript/JavaScript SDK |
| `programs` | Smart contract examples |
| `web` | Frontend applications |

---

## Token Economics

| Parameter | Value |
|-----------|-------|
| Symbol | PUMPC |
| Total Supply | 1,000,000,000 |
| Decimals | 9 |
| Inflation | 8% → 1.5% |

---

## License

MIT © 2025 PumpChain Labs

---

[Website](https://pumpchain.vercel.app) / [Twitter](https://twitter.com/0Xmizuno) / [Docs](https://docs.pumpchain.io)