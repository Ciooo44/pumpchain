// Pump.fun Fork Frontend Integration
// Connects to deployed PumpChain bonding curve program

const PROGRAM_ID = 'Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS'; // Will update after deploy
const PUMPCHAIN_RPC = 'https://databases-chief-patent-del.trycloudflare.com';

// Bonding curve configuration
const CONFIG = {
  VIRTUAL_SOL_RESERVE: 30, // 30 SOL
  VIRTUAL_TOKEN_RESERVE: 1073000000, // 1.073B tokens
  TOTAL_SUPPLY: 1000000000, // 1B tokens
  MIGRATION_THRESHOLD: 69, // 69 SOL
  CREATOR_FEE_BPS: 100, // 1%
};

// Calculate token amount for given SOL input
function calculateTokensOut(solAmount, virtualSol, virtualTokens) {
  const newSolReserve = virtualSol + solAmount;
  const newTokenReserve = (virtualSol * virtualTokens) / newSolReserve;
  return Math.floor(virtualTokens - newTokenReserve);
}

// Calculate SOL amount for given token input  
function calculateSolOut(tokenAmount, virtualSol, virtualTokens) {
  const newTokenReserve = virtualTokens + tokenAmount;
  const newSolReserve = (virtualSol * virtualTokens) / newTokenReserve;
  return Math.floor(virtualSol - newSolReserve);
}

// Get current price
function getCurrentPrice(virtualSol, virtualTokens) {
  return virtualSol / virtualTokens;
}

// Get market cap
function getMarketCap(virtualSol, virtualTokens, totalSupply) {
  const price = getCurrentPrice(virtualSol, virtualTokens);
  return price * totalSupply;
}

// Get migration progress %
function getMigrationProgress(virtualSol) {
  return Math.min((virtualSol / CONFIG.MIGRATION_THRESHOLD) * 100, 100);
}

// Format number with commas
function formatNumber(num) {
  return num.toLocaleString('en-US', { maximumFractionDigits: 4 });
}

// Format SOL to human readable
function formatSol(lamports) {
  return (lamports / 1e9).toFixed(4) + ' SOL';
}

// Format tokens to human readable
function formatTokens(amount) {
  if (amount >= 1e9) return (amount / 1e9).toFixed(2) + 'B';
  if (amount >= 1e6) return (amount / 1e6).toFixed(2) + 'M';
  if (amount >= 1e3) return (amount / 1e3).toFixed(2) + 'K';
  return amount.toString();
}

// Simulate bonding curve for display
class BondingCurveSimulator {
  constructor() {
    this.virtualSol = CONFIG.VIRTUAL_SOL_RESERVE;
    this.virtualTokens = CONFIG.VIRTUAL_TOKEN_RESERVE;
    this.realSol = 0;
    this.realTokens = CONFIG.TOTAL_SUPPLY;
    this.migrated = false;
    this.trades = [];
  }

  buy(solAmount) {
    const tokensOut = calculateTokensOut(solAmount, this.virtualSol, this.virtualTokens);
    const fee = solAmount * 0.01; // 1% fee
    const solAfterFee = solAmount - fee;

    this.virtualSol += solAfterFee;
    this.virtualTokens -= tokensOut;
    this.realSol += solAfterFee;
    this.realTokens -= tokensOut;

    const progress = getMigrationProgress(this.virtualSol);
    if (progress >= 100 && !this.migrated) {
      this.migrated = true;
    }

    const trade = {
      type: 'buy',
      solAmount,
      tokensOut,
      price: solAfterFee / tokensOut,
      marketCap: getMarketCap(this.virtualSol, this.virtualTokens, CONFIG.TOTAL_SUPPLY),
      progress,
      timestamp: Date.now(),
    };
    this.trades.push(trade);
    return trade;
  }

  sell(tokenAmount) {
    const solOut = calculateSolOut(tokenAmount, this.virtualSol, this.virtualTokens);
    const fee = solOut * 0.01; // 1% fee
    const solAfterFee = solOut - fee;

    this.virtualSol -= solOut;
    this.virtualTokens += tokenAmount;
    this.realSol -= solOut;
    this.realTokens += tokenAmount;

    const trade = {
      type: 'sell',
      tokenAmount,
      solOut: solAfterFee,
      price: solAfterFee / tokenAmount,
      marketCap: getMarketCap(this.virtualSol, this.virtualTokens, CONFIG.TOTAL_SUPPLY),
      progress: getMigrationProgress(this.virtualSol),
      timestamp: Date.now(),
    };
    this.trades.push(trade);
    return trade;
  }

  getState() {
    return {
      virtualSol: this.virtualSol,
      virtualTokens: this.virtualTokens,
      realSol: this.realSol,
      realTokens: this.realTokens,
      price: getCurrentPrice(this.virtualSol, this.virtualTokens),
      marketCap: getMarketCap(this.virtualSol, this.virtualTokens, CONFIG.TOTAL_SUPPLY),
      progress: getMigrationProgress(this.virtualSol),
      migrated: this.migrated,
    };
  }
}

// Export for use
if (typeof module !== 'undefined' && module.exports) {
  module.exports = {
    CONFIG,
    calculateTokensOut,
    calculateSolOut,
    getCurrentPrice,
    getMarketCap,
    getMigrationProgress,
    formatNumber,
    formatSol,
    formatTokens,
    BondingCurveSimulator,
  };
}