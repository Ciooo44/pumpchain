# PumpChain Testnet Configuration

## 🧪 Testnet Details

| Parameter | Value |
|-----------|-------|
| **Network Name** | PumpChain Testnet |
| **Chain ID** | 1397 |
| **RPC URL** | https://testnet.pumpchain.io |
| **WS URL** | wss://testnet.pumpchain.io |
| **Explorer** | https://testnet.pumpscan.app |
| **Faucet** | https://faucet.pumpchain.io |

---

## 🔗 Wallet Configuration

### Backpack Wallet Setup

1. Open Backpack → Settings → Solana
2. Click "RPC Connection"
3. Select "Custom"
4. Enter: `https://testnet.pumpchain.io`
5. Save

### Phantom Wallet Setup

1. Open Phantom → Settings → Developer Settings
2. Toggle "Testnet Mode"
3. Add Custom RPC:
   - Name: PumpChain Testnet
   - URL: `https://testnet.pumpchain.io`

---

## 💰 Faucet

Get free test PUMP tokens:

```bash
curl -X POST https://faucet.pumpchain.io \
  -H "Content-Type: application/json" \
  -d '{
    "address": "YOUR_WALLET_ADDRESS",
    "amount": 1000
  }'
```

Or visit: https://faucet.pumpchain.io

---

## 📦 Deployed Programs (Testnet)

| Program | Address | Description |
|---------|---------|-------------|
| PumpLaunchpad | `pumpLchPad1111111111111111111111111111111` | Token launchpad |
| PumpProof | `pumpProof11111111111111111111111111111111` | State anchoring |
| PumpToken | `pumpToken11111111111111111111111111111111` | Token program |

---

## 🔧 Testing

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
curl https://testnet.pumpchain.io \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getBalance",
    "params": ["YOUR_ADDRESS"]
  }'
```

### 3. Bridge from Solana Devnet
```bash
pumpchain bridge devnet-to-testnet \
  --amount 100 \
  --token SOL
```

---

## 🚨 Common Issues

**RPC Not Responding:**
- Check network connection
- Verify RPC URL: `https://testnet.pumpchain.io`

**Transaction Failed:**
- Ensure you have test PUMP tokens
- Check gas limit (default: 2M units)

**Contract Not Found:**
- Verify program ID
- Check testnet explorer

---

## 📊 Testnet Status

Check status: https://status.pumpchain.io

- ✅ RPC: Operational
- ✅ Faucet: Operational
- ✅ Explorer: Operational
- ✅ Bridge: Operational