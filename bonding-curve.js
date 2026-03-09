// Bonding Curve AMM for PumpChain
// Constant product formula: x * y = k

class BondingCurve {
  constructor(params) {
    this.tokenMint = params.tokenMint;
    this.virtualSolReserve = params.virtualSolReserve || 30; // 30 SOL virtual
    this.virtualTokenReserve = params.virtualTokenReserve || 1073000000; // 1.073B tokens
    this.realSolReserve = 0;
    this.realTokenReserve = 0;
    this.totalSupply = params.totalSupply || 1000000000;
    this.migrated = false;
  }

  // Calculate token amount for given SOL
  getTokenAmount(solAmount) {
    const newSolReserve = this.virtualSolReserve + solAmount;
    const newTokenReserve = (this.virtualSolReserve * this.virtualTokenReserve) / newSolReserve;
    const tokensOut = this.virtualTokenReserve - newTokenReserve;
    return Math.floor(tokensOut);
  }

  // Calculate SOL amount for given tokens
  getSolAmount(tokenAmount) {
    const newTokenReserve = this.virtualTokenReserve + tokenAmount;
    const newSolReserve = (this.virtualSolReserve * this.virtualTokenReserve) / newTokenReserve;
    const solOut = this.virtualSolReserve - newSolReserve;
    return Math.floor(solOut);
  }

  // Buy tokens with SOL
  buy(solAmount) {
    const tokensOut = this.getTokenAmount(solAmount);
    
    this.virtualSolReserve += solAmount;
    this.virtualTokenReserve -= tokensOut;
    this.realSolReserve += solAmount;
    this.realTokenReserve -= tokensOut;

    // Check if should migrate to Raydium (69k market cap)
    if (this.virtualSolReserve >= 69 && !this.migrated) {
      this.migrated = true;
      console.log('🎯 Migration threshold reached! Ready for Raydium.');
    }

    return {
      tokensReceived: tokensOut,
      price: solAmount / tokensOut,
      marketCap: this.getMarketCap()
    };
  }

  // Sell tokens for SOL
  sell(tokenAmount) {
    const solOut = this.getSolAmount(tokenAmount);
    
    this.virtualSolReserve -= solOut;
    this.virtualTokenReserve += tokenAmount;
    this.realSolReserve -= solOut;
    this.realTokenReserve += tokenAmount;

    return {
      solReceived: solOut,
      price: solOut / tokenAmount,
      marketCap: this.getMarketCap()
    };
  }

  getMarketCap() {
    // Simplified market cap calculation
    const pricePerToken = this.virtualSolReserve / this.virtualTokenReserve;
    return pricePerToken * this.totalSupply;
  }

  getPrice() {
    return this.virtualSolReserve / this.virtualTokenReserve;
  }
}

// Export for use in other files
module.exports = { BondingCurve };

// Example usage
if (require.main === module) {
  const curve = new BondingCurve({
    tokenMint: 'pumpTest123',
    totalSupply: 1000000000
  });

  console.log('🚀 Bonding Curve Simulation\n');
  console.log('Initial state:');
  console.log('  Virtual SOL:', curve.virtualSolReserve);
  console.log('  Virtual Tokens:', curve.virtualTokenReserve);
  console.log('  Price:', curve.getPrice().toFixed(10), 'SOL/token');
  console.log('  Market Cap:', curve.getMarketCap().toFixed(2), 'SOL\n');

  // Simulate buying with 1 SOL
  console.log('📈 Buying with 1 SOL...');
  const buy1 = curve.buy(1);
  console.log('  Tokens received:', buy1.tokensReceived.toLocaleString());
  console.log('  New price:', curve.getPrice().toFixed(10), 'SOL/token');
  console.log('  Market Cap:', buy1.marketCap.toFixed(2), 'SOL\n');

  // Simulate buying with 10 SOL
  console.log('📈 Buying with 10 SOL...');
  const buy10 = curve.buy(10);
  console.log('  Tokens received:', buy10.tokensReceived.toLocaleString());
  console.log('  New price:', curve.getPrice().toFixed(10), 'SOL/token');
  console.log('  Market Cap:', buy10.marketCap.toFixed(2), 'SOL\n');

  // Show progression to migration
  console.log('🎯 Migration at 69 SOL market cap...');
  let totalSol = 11;
  while (totalSol < 70) {
    const result = curve.buy(5);
    totalSol += 5;
    console.log(`  ${totalSol} SOL → Price: ${curve.getPrice().toFixed(10)} | Migrated: ${curve.migrated}`);
  }
}