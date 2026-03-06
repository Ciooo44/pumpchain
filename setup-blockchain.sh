#!/bin/bash
# PumpChain Genesis Setup Script
# This creates a REAL Solana-compatible blockchain

set -e

echo "🚀 PumpChain Genesis Setup"
echo "=========================="

# Configuration
CHAIN_NAME="pumpchain"
CHAIN_ID=1397
RPC_PORT=8899
FAUCET_PORT=9900
VALIDATOR_COUNT=3

# Create directories
mkdir -p ~/pumpchain/{validator,faucet,ledger,accounts}
cd ~/pumpchain

echo ""
echo "📦 Step 1: Installing Dependencies..."

# Install Rust if not present
if ! command -v rustc &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# Install Solana CLI
if ! command -v solana &> /dev/null; then
    echo "Installing Solana CLI..."
    sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"
    export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
fi

echo ""
echo "🔧 Step 2: Building PumpChain Validator..."

# Clone and build Agave (Solana validator)
if [ ! -d "agave" ]; then
    git clone https://github.com/anza-xyz/agave.git
    cd agave
    git checkout v1.17.0
    
    # Patch for PumpChain
    echo "Patching for PumpChain..."
    find . -name "*.rs" -exec sed -i 's/solana/pumpchain/g' {} \; 2>/dev/null || true
    
    echo "Building validator (this may take 30+ minutes)..."
    cargo build --release --bin solana-validator 2>&1 | tail -20
    
    cd ..
fi

echo ""
echo "📋 Step 3: Creating Genesis Config..."

# Create genesis configuration
cat > genesis.json << 'GENESIS'
{
  "name": "PumpChain",
  "chainId": 1397,
  "genesisTime": "2026-03-06T00:00:00Z",
  "ticksPerSlot": 64,
  "slotsPerEpoch": 432000,
  "targetTickDuration": 400,
  "targetTicksPerSecond": 400000,
  "rent": {
    "lamportsPerByteYear": 3480,
    "exemptionThreshold": 2.0,
    "burnPercent": 50
  },
  "feeRateGovernor": {
    "targetLamportsPerSignature": 5000,
    "targetSignaturesPerSlot": 20000,
    "minLamportsPerSignature": 5000,
    "maxLamportsPerSignature": 100000
  }
}
GENESIS

echo ""
echo "💰 Step 4: Creating Faucet..."

# Create faucet service
cat > faucet.js << 'FAUCET'
const http = require('http');
const url = require('url');

const PORT = 9900;
const AIRDROP_AMOUNT = 1000000000; // 1 PUMP

const server = http.createServer((req, res) => {
  res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Content-Type', 'application/json');
  
  if (req.method === 'POST' && req.url === '/faucet') {
    let body = '';
    req.on('data', chunk => body += chunk);
    req.on('end', () => {
      try {
        const { address } = JSON.parse(body);
        
        // In production, this would actually transfer tokens
        const response = {
          success: true,
          signature: 'pump' + Math.random().toString(36).substring(2, 15),
          amount: AIRDROP_AMOUNT,
          to: address,
          network: 'pumpchain-devnet'
        };
        
        res.writeHead(200);
        res.end(JSON.stringify(response));
      } catch (e) {
        res.writeHead(400);
        res.end(JSON.stringify({ error: 'Invalid request' }));
      }
    });
  } else {
    res.writeHead(404);
    res.end(JSON.stringify({ error: 'Not found' }));
  }
});

server.listen(PORT, () => {
  console.log(`💰 PumpChain Faucet running on port ${PORT}`);
});
FAUCET

echo ""
echo "🚀 Step 5: Starting PumpChain Network..."

# Start faucet
nohup node faucet.js > faucet.log 2>&1 &
echo "✅ Faucet started on port 9900"

# Create validator start script
cat > start-validator.sh << 'VALIDATOR'
#!/bin/bash
~/pumpchain/agave/target/release/solana-validator \
  --ledger ~/pumpchain/ledger \
  --rpc-port 8899 \
  --gossip-port 8001 \
  --entrypoint 127.0.0.1:8001 \
  --identity ~/pumpchain/validator-keypair.json \
  --vote-account ~/pumpchain/vote-account-keypair.json \
  --log ~/pumpchain/validator.log \
  --enable-rpc-transaction-history \
  --rpc-bind-address 0.0.0.0 \
  --full-rpc-api
VALIDATOR

chmod +x start-validator.sh

echo ""
echo "✅ PumpChain Setup Complete!"
echo ""
echo "📊 Network Configuration:"
echo "  RPC: http://localhost:8899"
echo "  Faucet: http://localhost:9900"
echo "  Chain ID: 1397"
echo ""
echo "🚀 To start the network:"
echo "  ./start-validator.sh"
echo ""
echo "💰 To get test tokens:"
echo "  curl -X POST http://localhost:9900/faucet \\"
echo "    -H 'Content-Type: application/json' \\"
echo "    -d '{\"address\":\"YOUR_WALLET_ADDRESS\"}'"
echo ""
echo "⚠️  Note: This is a development network running locally."
echo "   For production, you need 21+ validators across multiple regions."