#!/bin/bash
# PumpChain Testnet Setup Script

echo "🚀 PumpChain Testnet Setup"
echo "=========================="

# Configuration
TESTNET_RPC="https://testnet.pumpchain.io"
CHAIN_ID=1397

echo ""
echo "📋 Testnet Configuration:"
echo "  RPC URL: $TESTNET_RPC"
echo "  Chain ID: $CHAIN_ID"
echo "  Network: testnet"

echo ""
echo "🔧 Setting up environment..."

# Create .env file for frontend
cat > .env << EOF
REACT_APP_NETWORK=testnet
REACT_APP_RPC_URL=$TESTNET_RPC
REACT_APP_CHAIN_ID=$CHAIN_ID
REACT_APP_EXPLORER=https://testnet.pumpscan.app
REACT_APP_FAUCET=https://faucet.pumpchain.io
EOF

echo "✅ Environment file created"

echo ""
echo "📦 Installing dependencies..."
npm install

echo ""
echo "🏗️ Building for testnet..."
npm run build

echo ""
echo "🚀 Deploying to testnet..."
echo "  Frontend: https://pumpchain.vercel.app"
echo "  RPC: $TESTNET_RPC"
echo "  Explorer: https://testnet.pumpscan.app"

echo ""
echo "✅ Testnet setup complete!"
echo ""
echo "📖 Next steps:"
echo "  1. Get test tokens from faucet"
echo "  2. Configure wallet with custom RPC"
echo "  3. Start building!"