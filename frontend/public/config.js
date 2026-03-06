// PumpChain Mainnet Configuration
const MAINNET_CONFIG = {
  // Network
  NETWORK: 'mainnet',
  CHAIN_ID: 1397,
  
  // RPC Endpoints
  RPC_URL: 'https://mainnet.pumpchain.io',
  WS_URL: 'wss://mainnet.pumpchain.io',
  
  // Explorer
  EXPLORER_URL: 'https://pumpscan.app',
  
  // Faucet (optional - mainnet may not have faucet)
  FAUCET_URL: 'https://mainnet.pumpchain.io/faucet',
  
  // Program IDs (Mainnet)
  PROGRAMS: {
    LAUNCHPAD: 'pumpLchPad1111111111111111111111111111111',
    PROOF: 'pumpProof11111111111111111111111111111111',
    TOKEN: 'pumpToken11111111111111111111111111111111',
  },
  
  // Token
  NATIVE_TOKEN: {
    SYMBOL: 'PUMP',
    DECIMALS: 9,
    NAME: 'PumpChain Token',
  },
  
  // Gas
  DEFAULT_GAS_LIMIT: 2000000,
  DEFAULT_GAS_PRICE: 1,
};

// Export for use
if (typeof module !== 'undefined' && module.exports) {
  module.exports = MAINNET_CONFIG;
}

// Browser global
if (typeof window !== 'undefined') {
  window.PUMPCHAIN_MAINNET = MAINNET_CONFIG;
}
