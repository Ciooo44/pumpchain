#!/bin/bash
# PumpChain Real Validator Node Setup
# This sets up an ACTUAL Solana validator for PumpChain

set -e

echo "🔧 PumpChain Validator Node Setup"
echo "=================================="
echo ""

# Install Solana CLI
echo "📦 Installing Solana CLI..."
sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"

# Create directories
mkdir -p ~/pumpchain-validator/{ledger,accounts,logs}
cd ~/pumpchain-validator

echo ""
echo "🔑 Generating Validator Keys..."

# Generate identity keypair
solana-keygen new --no-passphrase -o identity.json
solana-keygen new --no-passphrase -o vote.json
solana-keygen new --no-passphrase -o withdrawer.json

echo ""
echo "📋 Validator Configuration:"
echo "  Identity: $(solana-keygen pubkey identity.json)"
echo "  Vote: $(solana-keygen pubkey vote.json)"
echo ""

# Create validator startup script
cat > start-validator.sh << 'EOF'
#!/bin/bash

# PumpChain Validator Startup Script
# IMPORTANT: This creates a NEW local network, NOT connecting to Solana mainnet

cd ~/pumpchain-validator

# Set Solana config for local development
solana config set --url localhost

# Start the validator with devnet settings
solana-test-validator \
  --ledger ./ledger \
  --rpc-port 8899 \
  --faucet-port 9900 \
  --bind-address 0.0.0.0 \
  --rpc-bind-address 0.0.0.0 \
  --gossip-host 0.0.0.0 \
  --enable-rpc-transaction-history \
  --enable-extended-tx-metadata-storage \
  --limit-ledger-size 100000000 \
  --max-genesis-archive-unpacked-size 1073741824 \
  --identity ./identity.json \
  --vote-account ./vote.json \
  --mint $(solana-keygen pubkey ./identity.json) \
  --slots-per-epoch 432000 \
  --ticks-per-slot 64 \
  --fde 1000000000 \
  --dynamic-port-range 8000-8020 \
  --log ./logs/validator.log

# Alternative: Run as background service with nohup
# nohup solana-test-validator [options above] > ./logs/validator.log 2>&1 &
EOF

chmod +x start-validator.sh

echo ""
echo "✅ Validator Setup Complete!"
echo ""
echo "🚀 To start the validator:"
echo "  cd ~/pumpchain-validator"
echo "  ./start-validator.sh"
echo ""
echo "📡 RPC will be available at:"
echo "  http://localhost:8899"
echo ""
echo "💰 Faucet will be available at:"
echo "  http://localhost:9900"
echo ""
echo "⚠️  This creates a LOCAL development network."
echo "   It does NOT connect to Solana mainnet or devnet."
echo ""
echo "🔍 To check status:"
echo "  solana config set --url http://localhost:8899"
echo "  solana cluster-version"
echo "  solana block-height"
echo "  solana slot"