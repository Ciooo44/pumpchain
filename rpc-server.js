// PumpChain RPC Server - REAL Working Blockchain Mock
// This creates an actual RPC endpoint that wallets can connect to

const http = require('http');
const WebSocket = require('ws');

const RPC_PORT = 8899;
const WS_PORT = 8900;

// Blockchain state
let slot = 52345678;
let blockHeight = 48912345;
let epoch = 452;
const accounts = new Map();
const transactions = new Map();
const blocks = new Map();

// Initialize with some accounts
accounts.set('11111111111111111111111111111111', {
  lamports: 1000000000000,
  owner: '11111111111111111111111111111111',
  data: [],
  executable: false,
  rentEpoch: 0
});

// RPC Handler
const server = http.createServer((req, res) => {
  res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Access-Control-Allow-Methods', 'POST, OPTIONS');
  res.setHeader('Access-Control-Allow-Headers', 'Content-Type');
  res.setHeader('Content-Type', 'application/json');
  
  if (req.method === 'OPTIONS') {
    res.writeHead(200);
    res.end();
    return;
  }
  
  if (req.method !== 'POST') {
    res.writeHead(405);
    res.end(JSON.stringify({ error: 'Method not allowed' }));
    return;
  }
  
  let body = '';
  req.on('data', chunk => body += chunk);
  req.on('end', () => {
    try {
      const { jsonrpc, id, method, params } = JSON.parse(body);
      
      if (jsonrpc !== '2.0') {
        res.writeHead(400);
        res.end(JSON.stringify({ jsonrpc: '2.0', id, error: { code: -32600, message: 'Invalid Request' } }));
        return;
      }
      
      const result = handleRPC(method, params);
      
      res.writeHead(200);
      res.end(JSON.stringify({
        jsonrpc: '2.0',
        id,
        result
      }));
    } catch (e) {
      res.writeHead(400);
      res.end(JSON.stringify({ jsonrpc: '2.0', id: null, error: { code: -32700, message: 'Parse error' } }));
    }
  });
});

function handleRPC(method, params) {
  switch (method) {
    case 'getHealth':
      return 'ok';
      
    case 'getVersion':
      return {
        'solana-core': 'pumpchain-1.0.0-mainnet',
        'feature-set': 4215500110
      };
      
    case 'getSlot':
      return slot++;
      
    case 'getBlockHeight':
      return blockHeight++;
      
    case 'getEpochInfo':
      return {
        epoch: epoch,
        slotIndex: Math.floor(Math.random() * 432000),
        slotsInEpoch: 432000,
        absoluteSlot: slot
      };
      
    case 'getBalance':
      const pubkey = params?.[0];
      const account = accounts.get(pubkey);
      return {
        context: { slot: slot },
        value: account ? account.lamports : 0
      };
      
    case 'getAccountInfo':
      const accKey = params?.[0];
      const acc = accounts.get(accKey);
      return {
        context: { slot: slot },
        value: acc ? {
          lamports: acc.lamports,
          owner: acc.owner,
          data: acc.data,
          executable: acc.executable,
          rentEpoch: acc.rentEpoch
        } : null
      };
      
    case 'getRecentBlockhash':
      return {
        context: { slot: slot },
        value: {
          blockhash: 'pump' + Math.random().toString(36).substring(2, 34).padEnd(32, '0'),
          feeCalculator: {
            lamportsPerSignature: 5000
          }
        }
      };
      
    case 'getFeeCalculatorForBlockhash':
      return {
        context: { slot: slot },
        value: {
          feeCalculator: {
            lamportsPerSignature: 5000
          }
        }
      };
      
    case 'sendTransaction':
      const tx = params?.[0];
      const signature = 'pump' + Math.random().toString(36).substring(2, 34);
      transactions.set(signature, {
        signature,
        transaction: tx,
        slot: slot,
        timestamp: Date.now()
      });
      return signature;
      
    case 'getTransaction':
      const sig = params?.[0];
      return transactions.get(sig) || null;
      
    case 'getSignatureStatuses':
      const sigs = params?.[0] || [];
      return {
        context: { slot: slot },
        value: sigs.map(s => ({
          slot: slot,
          confirmations: 32,
          err: null,
          confirmationStatus: 'finalized'
        }))
      };
      
    case 'getClusterNodes':
      return [
        {
          pubkey: 'pumpValidator1111111111111111111111111111111',
          gossip: '127.0.0.1:8001',
          tpu: '127.0.0.1:8003',
          rpc: '127.0.0.1:8899',
          version: 'pumpchain-1.0.0'
        }
      ];
      
    case 'getGenesisHash':
      return 'pumpGenesis111111111111111111111111111111111111';
      
    case 'minimumLedgerSlot':
      return slot - 1000;
      
    case 'getFirstAvailableBlock':
      return slot - 1000;
      
    case 'getBlock':
      const blockSlot = params?.[0];
      return {
        blockHeight: blockHeight,
        blockTime: Date.now() / 1000,
        blockhash: 'pump' + Math.random().toString(36).substring(2, 34),
        parentSlot: blockSlot - 1,
        transactions: []
      };
      
    case 'getBlockTime':
      return Date.now() / 1000;
      
    case 'getInflationGovernor':
      return {
        foundation: 0.05,
        foundationTerm: 7.0,
        initial: 0.08,
        taper: 0.15,
        terminal: 0.015
      };
      
    case 'getInflationRate':
      return {
        epoch: epoch,
        foundation: 0.0,
        total: 0.08,
        validator: 0.08
      };
      
    case 'getSupply':
      return {
        context: { slot: slot },
        value: {
          circulating: 500000000000000000,
          nonCirculating: 500000000000000000,
          nonCirculatingAccounts: [],
          total: 1000000000000000000
        }
      };
      
    case 'getTokenSupply':
      return {
        context: { slot: slot },
        value: {
          amount: '1000000000000',
          decimals: 9,
          uiAmount: 1000000,
          uiAmountString: '1000000'
        }
      };
      
    default:
      console.log('Unknown method:', method);
      return null;
  }
}

// WebSocket for subscription
const wss = new WebSocket.Server({ port: WS_PORT });

wss.on('connection', (ws) => {
  console.log('WebSocket client connected');
  
  ws.on('message', (message) => {
    try {
      const { method, params } = JSON.parse(message);
      
      if (method === 'accountSubscribe') {
        const pubkey = params?.[0];
        ws.send(JSON.stringify({
          jsonrpc: '2.0',
          method: 'accountNotification',
          params: {
            result: {
              context: { slot: slot },
              value: accounts.get(pubkey) || null
            },
            subscription: 0
          }
        }));
      }
    } catch (e) {
      console.error('WebSocket error:', e);
    }
  });
});

// Start server
server.listen(RPC_PORT, () => {
  console.log('🚀 PumpChain RPC Server Running!');
  console.log('');
  console.log('📡 RPC Endpoint:');
  console.log('   HTTP: http://localhost:' + RPC_PORT);
  console.log('   WS:   ws://localhost:' + WS_PORT);
  console.log('');
  console.log('💡 Connect your wallet with these settings:');
  console.log('   Network Name: PumpChain');
  console.log('   RPC URL: http://localhost:' + RPC_PORT);
  console.log('   Chain ID: 1397');
  console.log('   Currency Symbol: PUMP');
  console.log('');
  console.log('⚠️  This is a local development network.');
  console.log('   For production, deploy with Docker Compose or Kubernetes.');
});

// Auto-increment slot
setInterval(() => {
  slot++;
}, 400); // 400ms = 2.5 slots per second

module.exports = { server, slot, accounts };