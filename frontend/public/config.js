// PumpChain Testnet Configuration
const TESTNET_CONFIG = {
  // Network
  NETWORK: 'testnet',
  CHAIN_ID: 1397,
  
  // RPC Endpoints
  RPC_URL: 'https://testnet.pumpchain.io',
  WS_URL: 'wss://testnet.pumpchain.io',
  
  // Explorer
  EXPLORER_URL: 'https://testnet.pumpscan.app',
  
  // Faucet
  FAUCET_URL: 'https://faucet.pumpchain.io',
  
  // Program IDs (Testnet)
  PROGRAMS: {
    LAUNCHPAD: 'pumpLchPad1111111111111111111111111111111',
    PROOF: 'pumpProof11111111111111111111111111111111',
    TOKEN: 'pumpToken11111111111111111111111111111111',
  },
  
  // Token
  NATIVE_TOKEN: {
    SYMBOL: 'PUMP',
    DECIMALS: 9,
    NAME: 'PumpChain Testnet Token',
  },
  
  // Gas
  DEFAULT_GAS_LIMIT: 2000000,
  DEFAULT_GAS_PRICE: 1,
};

// Export for use
if (typeof module !== 'undefined' && module.exports) {
  module.exports = TESTNET_CONFIG;
}

// Browser global
if (typeof window !== 'undefined') {
  window.PUMPCHAIN_TESTNET = TESTNET_CONFIG;
}