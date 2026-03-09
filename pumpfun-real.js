// REAL Pump.fun Integration
// Interacts with actual Pump.fun program on Solana mainnet

const solanaWeb3 = require('@solana/web3.js');

// Pump.fun constants
const PUMP_FUN_PROGRAM = new solanaWeb3.PublicKey('6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P');
const PUMP_FUN_ACCOUNT = new solanaWeb3.PublicKey('Ce6TQqeHC9p8KetsN7JsjKh7NVk9jfT8M6QTfDJ7xLg');
const GLOBAL_ACCOUNT = new solanaWeb3.PublicKey('4wTV1YmiEkRvAtNtsSGPtU7rA8mHK8Yo9gNgv1Nyijn');
const EVENT_AUTHORITY = new solanaWeb3.PublicKey('GS4R6bBWSpr1CJ9wMP9YHRpKqZkWJjSjxWpKcfVNS5o');
const RENT_SYSVAR = solanaWeb3.SYSVAR_RENT_PUBKEY;
const SYSTEM_PROGRAM = solanaWeb3.SystemProgram.programId;
const TOKEN_PROGRAM = new solanaWeb3.PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA');
const ASSOCIATED_TOKEN_PROGRAM = new solanaWeb3.PublicKey('ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL');
const METADATA_PROGRAM = new solanaWeb3.PublicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s');

class RealPumpFun {
  constructor() {
    this.connection = new solanaWeb3.Connection('https://api.mainnet-beta.solana.com', 'confirmed');
  }

  // Create token instruction data structure
  createInstructionData(name, symbol, uri) {
    // Pump.fun create instruction discriminator
    const discriminator = Buffer.from([24, 30, 200, 40, 5, 28, 7, 119]);
    
    // String lengths and data
    const nameLen = Buffer.alloc(4);
    nameLen.writeUInt32LE(name.length, 0);
    const nameBuf = Buffer.from(name);
    
    const symbolLen = Buffer.alloc(4);
    symbolLen.writeUInt32LE(symbol.length, 0);
    const symbolBuf = Buffer.from(symbol);
    
    const uriLen = Buffer.alloc(4);
    uriLen.writeUInt32LE(uri.length, 0);
    const uriBuf = Buffer.from(uri);
    
    return Buffer.concat([
      discriminator,
      nameLen, nameBuf,
      symbolLen, symbolBuf,
      uriLen, uriBuf
    ]);
  }

  async buildCreateTokenTransaction(params) {
    const { creator, name, symbol, uri } = params;
    
    // Generate mint keypair
    const mint = solanaWeb3.Keypair.generate();
    
    // Get associated bonding curve address
    const [bondingCurve] = await solanaWeb3.PublicKey.findProgramAddress(
      [Buffer.from('bonding-curve'), mint.publicKey.toBuffer()],
      PUMP_FUN_PROGRAM
    );
    
    // Get associated token account
    const [associatedToken] = await solanaWeb3.PublicKey.findProgramAddress(
      [
        bondingCurve.toBuffer(),
        TOKEN_PROGRAM.toBuffer(),
        mint.publicKey.toBuffer()
      ],
      ASSOCIATED_TOKEN_PROGRAM
    );
    
    // Get metadata address
    const [metadata] = await solanaWeb3.PublicKey.findProgramAddress(
      [
        Buffer.from('metadata'),
        METADATA_PROGRAM.toBuffer(),
        mint.publicKey.toBuffer()
      ],
      METADATA_PROGRAM
    );
    
    // Create instruction
    const data = this.createInstructionData(name, symbol, uri);
    
    const keys = [
      { pubkey: mint.publicKey, isSigner: true, isWritable: true },
      { pubkey: bondingCurve, isSigner: false, isWritable: true },
      { pubkey: associatedToken, isSigner: false, isWritable: true },
      { pubkey: GLOBAL_ACCOUNT, isSigner: false, isWritable: true },
      { pubkey: metadata, isSigner: false, isWritable: true },
      { pubkey: creator.publicKey, isSigner: true, isWritable: true },
      { pubkey: SYSTEM_PROGRAM, isSigner: false, isWritable: false },
      { pubkey: TOKEN_PROGRAM, isSigner: false, isWritable: false },
      { pubkey: ASSOCIATED_TOKEN_PROGRAM, isSigner: false, isWritable: false },
      { pubkey: METADATA_PROGRAM, isSigner: false, isWritable: false },
      { pubkey: EVENT_AUTHORITY, isSigner: false, isWritable: false },
      { pubkey: PUMP_FUN_PROGRAM, isSigner: false, isWritable: false },
      { pubkey: RENT_SYSVAR, isSigner: false, isWritable: false },
    ];
    
    return new solanaWeb3.TransactionInstruction({
      keys,
      programId: PUMP_FUN_PROGRAM,
      data
    });
  }

  // RPC endpoint for frontend
  async handleCreateRequest(req, res) {
    try {
      const { name, symbol, imageUrl, creator } = req.body;
      
      // Build transaction (unsigned)
      const creatorKeypair = new solanaWeb3.PublicKey(creator);
      const ix = await this.buildCreateTokenTransaction({
        creator: { publicKey: creatorKeypair },
        name,
        symbol,
        uri: imageUrl || ''
      });
      
      const transaction = new solanaWeb3.Transaction().add(ix);
      transaction.feePayer = creatorKeypair;
      transaction.recentBlockhash = (await this.connection.getLatestBlockhash()).blockhash;
      
      // Return serialized transaction for frontend to sign
      const serialized = transaction.serialize({
        requireAllSignatures: false,
        verifySignatures: false
      });
      
      res.json({
        success: true,
        transaction: serialized.toString('base64'),
        message: 'Sign this transaction in your wallet to create the token'
      });
      
    } catch (error) {
      res.status(500).json({ error: error.message });
    }
  }
}

// Export
module.exports = { RealPumpFun };

// Test
if (require.main === module) {
  const pump = new RealPumpFun();
  console.log('✅ Real Pump.fun integration loaded');
  console.log('Program ID:', PUMP_FUN_PROGRAM.toString());
}