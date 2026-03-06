# PumpChain Mainnet Configuration

## 🚀 Mainnet Details

| Parameter | Value |
|-----------|-------|
| **Network Name** | PumpChain Mainnet |
| **Chain ID** | 1397 |
| **RPC URL** | https://mainnet.pumpchain.io |
| **WS URL** | wss://mainnet.pumpchain.io |
| **Explorer** | https://pumpscan.app |
| **Faucet** | https://mainnet.pumpchain.io/faucet (optional) |

---

## 🔗 Wallet Configuration

### Backpack Wallet Setup

1. Open Backpack → Settings → Solana
2. Click "RPC Connection"
3. Select "Custom"
4. Enter: `https://mainnet.pumpchain.io`
5. Save

### Phantom Wallet Setup

1. Open Phantom → Settings → Developer Settings
2. Add Custom RPC:
   - Name: PumpChain Mainnet
   - URL: `https://mainnet.pumpchain.io`

### Solflare Wallet Setup

1. Open Solflare → Settings → Network
2. Click "+" to add custom network
3. Enter:
   - Name: PumpChain Mainnet
   - RPC: `https://mainnet.pumpchain.io`
   - Chain ID: 1397

---

## 💰 Faucet (Optional)

Mainnet tokens have real value. Faucet availability is limited:

```bash
curl -X POST https://mainnet.pumpchain.io/faucet \
  -H "Content-Type: application/json" \
  -d '{
    "address": "YOUR_WALLET_ADDRESS",
    "amount": 100
  }'
```

---

## 📦 Deployed Programs (Mainnet)

| Program | Address | Description |
|---------|---------|-------------|
| PumpLaunchpad | `pumpLchPad1111111111111111111111111111111` | Token launchpad |
| PumpProof | `pumpProof11111111111111111111111111111111` | State anchoring |
| PumpToken | `pumpToken11111111111111111111111111111111` | Token program |

---

## 🔧 Usage

### 1. Launch Token
```bash
# Install CLI
npm install -g @pumpchain/cli

# Launch token
pumpchain launch \
  --name "My Token" \
  --symbol "MYT" \
  --supply 1000000 \
  --curve exponential
```

### 2. Check Balance
```bash
curl https://mainnet.pumpchain.io \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getBalance",
    "params": ["YOUR_ADDRESS"]
  }'
```

### 3. Bridge from Solana
```bash
pumpchain bridge solana-to-pumpchain \
  --amount 100 \
  --token SOL
```

---

## 🚨 Important Notes

**Mainnet tokens have real value!**
- Double-check all transaction amounts
- Verify contract addresses before interacting
- Keep your private keys secure

**RPC Connection:**
- Mainnet RPC: `https://mainnet.pumpchain.io`
- Verify connection before sending transactions

**Transaction Fees:**
- Mainnet requires real PUMP tokens for gas
- Fees are paid to validators

---

## 📊 Mainnet Status

Check status: https://status.pumpchain.io

- ✅ RPC: Operational
- ✅ Explorer: Operational
- ✅ Bridge: Operational
