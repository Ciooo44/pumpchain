// PumpChain Testnet API
// Simple mock API for demonstration

const express = require('express');
const cors = require('cors');
const app = express();

app.use(cors());
app.use(express.json());

// Mock blockchain state
let currentSlot = 52000000;
let currentEpoch = 450;
let blockHeight = 48000000;

// RPC Endpoint
app.post('/rpc', (req, res) => {
  const { method, params } = req.body;
  
  switch(method) {
    case 'getSlot':
      currentSlot++;
      res.json({ jsonrpc: '2.0', id: req.body.id, result: currentSlot });
      break;
      
    case 'getBlockHeight':
      blockHeight++;
      res.json({ jsonrpc: '2.0', id: req.body.id, result: blockHeight });
      break;
      
    case 'getEpochInfo':
      res.json({
        jsonrpc: '2.0',
        id: req.body.id,
        result: {
          epoch: currentEpoch,
          slotIndex: Math.floor(Math.random() * 432000),
          slotsInEpoch: 432000,
          absoluteSlot: currentSlot
        }
      });
      break;
      
    case 'getBalance':
      const address = params?.[0];
      const balance = address ? Math.floor(Math.random() * 1000000000) : 0;
      res.json({
        jsonrpc: '2.0',
        id: req.body.id,
        result: {
          context: { slot: currentSlot },
          value: balance
        }
      });
      break;
      
    case 'getVersion':
      res.json({
        jsonrpc: '2.0',
        id: req.body.id,
        result: {
          'solana-core': 'pumpchain-0.1.0-testnet',
          'feature-set': 4215500110
        }
      });
      break;
      
    default:
      res.json({
        jsonrpc: '2.0',
        id: req.body.id,
        error: { code: -32601, message: 'Method not found' }
      });
  }
});

// Faucet Endpoint
const faucetRequests = new Map();

app.post('/faucet', (req, res) => {
  const { address, amount = 1000 } = req.body;
  
  if (!address) {
    return res.status(400).json({ error: 'Address required' });
  }
  
  // Rate limiting (1 request per minute per address)
  const now = Date.now();
  const lastRequest = faucetRequests.get(address);
  if (lastRequest && now - lastRequest < 60000) {
    return res.status(429).json({ 
      error: 'Rate limited. Wait 1 minute between requests.' 
    });
  }
  
  faucetRequests.set(address, now);
  
  // Mock transaction
  const txSignature = 'pump' + Math.random().toString(36).substring(2, 15);
  
  res.json({
    success: true,
    signature: txSignature,
    amount: amount,
    to: address,
    network: 'testnet'
  });
});

// Health Check
app.get('/health', (req, res) => {
  res.json({
    status: 'healthy',
    network: 'testnet',
    slot: currentSlot,
    epoch: currentEpoch,
    uptime: process.uptime()
  });
});

// Stats
app.get('/stats', (req, res) => {
  res.json({
    tps: 50000,
    blockTime: 400,
    slot: currentSlot,
    epoch: currentEpoch,
    totalTransactions: Math.floor(Math.random() * 10000000),
    activeValidators: 21
  });
});

const PORT = process.env.PORT || 3001;
app.listen(PORT, () => {
  console.log(`🚀 PumpChain Testnet API running on port ${PORT}`);
});

module.exports = app;