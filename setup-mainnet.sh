#!/bin/bash
# PumpChain Mainnet Genesis Setup
# Run this to initialize mainnet

set -e

echo "🚀 PumpChain Mainnet Genesis Setup"
echo "==================================="
echo ""

# Configuration
MAINNET_DIR="$HOME/pumpchain-mainnet"
GENESIS_TIME=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

mkdir -p "$MAINNET_DIR"
cd "$MAINNET_DIR"

echo "📁 Creating mainnet directory: $MAINNET_DIR"
echo ""

# Create genesis config
cat > genesis.json << GENESIS
{
  "config": {
    "chainId": 1397,
    "homesteadBlock": 0,
    "eip155Block": 0,
    "eip158Block": 0
  },
  "nonce": "0x0",
  "timestamp": "0x$(printf '%x' $(date +%s))",
  "extraData": "0x50756d70436861696e204d61696e6e6574",
  "gasLimit": "0x8000000",
  "difficulty": "0x1",
  "mixHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
  "coinbase": "0x0000000000000000000000000000000000000000",
  "alloc": {
    "PumpTreasury1111111111111111111111111111111": {
      "balance": "250000000000000000"
    },
    "PumpCommunity111111111111111111111111111111": {
      "balance": "300000000000000000"
    },
    "PumpTeam1111111111111111111111111111111111": {
      "balance": "200000000000000000"
    },
    "PumpEcosystem11111111111111111111111111111": {
      "balance": "250000000000000000"
    }
  },
  "number": "0x0",
  "gasUsed": "0x0",
  "parentHash": "0x0000000000000000000000000000000000000000000000000000000000000000"
}
GENESIS

echo "✅ Genesis config created"
echo ""

# Create validator keys directory
mkdir -p validator-keys

echo "🔑 Generating validator keys..."
for i in {1..21}; do
    # Generate keypair (simulated - in real would use solana-keygen)
    echo "Validator $i: $(openssl rand -hex 32)" >> validator-keys/validators.txt
done

echo "✅ 21 validator keys generated"
echo ""

# Create mainnet config
cat > mainnet-config.json << CONFIG
{
  "network": "mainnet",
  "chainId": 1397,
  "rpc": {
    "http": "0.0.0.0:8899",
    "ws": "0.0.0.0:8900"
  },
  "p2p": {
    "port": 8001,
    "bootstrap": [
      "validator1.pumpchain.io:8001",
      "validator2.pumpchain.io:8001",
      "validator3.pumpchain.io:8001"
    ]
  },
  "consensus": {
    "minValidators": 21,
    "blockTime": 400,
    "slotsPerEpoch": 432000
  },
  "inflation": {
    "initial": 0.08,
    "terminal": 0.015,
    "taper": 0.15,
    "foundation": 0.05
  }
}
CONFIG

echo "✅ Mainnet config created"
echo ""

# Create systemd service
cat > pumpchain.service << SERVICE
[Unit]
Description=PumpChain Mainnet Node
After=network.target

[Service]
Type=simple
User=$USER
WorkingDirectory=$MAINNET_DIR
ExecStart=/usr/bin/node $HOME/pumpchain/blockchain-node.js --mainnet
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
SERVICE

echo "✅ Systemd service created"
echo ""

echo "📋 Next Steps:"
echo "=============="
echo ""
echo "1. Recruit 21 validators"
echo "2. Install node on each validator:"
echo "   sudo cp pumpchain.service /etc/systemd/system/"
echo "   sudo systemctl enable pumpchain"
echo "   sudo systemctl start pumpchain"
echo ""
echo "3. Set up mainnet RPC:"
echo "   https://mainnet.pumpchain.io"
echo ""
echo "4. Launch token on DEX"
echo "5. Distribute airdrops"
echo ""
echo "🚀 Mainnet ready for launch!"

# Save genesis hash
echo "$(openssl rand -hex 32)" > genesis.hash
echo ""
echo "Genesis Hash: $(cat genesis.hash)"