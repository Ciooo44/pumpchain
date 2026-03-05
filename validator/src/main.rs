use clap::Parser;
use tracing::{info, error};

mod consensus;
mod execution;
mod network;
mod state;
mod rpc;

use consensus::PumpBFT;
use execution::SvmExecutor;
use network::P2PNetwork;
use state::StateManager;
use rpc::JsonRpcServer;

/// PumpChain Validator Node
#[derive(Parser, Debug)]
#[command(name = "pumpchain-validator")]
#[command(about = "PumpChain SVM-based validator node")]
struct Args {
    /// Validator identity keypair
    #[arg(short, long)]
    identity: String,
    
    /// RPC bind address
    #[arg(long, default_value = "0.0.0.0:8899")]
    rpc_bind: String,
    
    /// P2P listen address
    #[arg(long, default_value = "/ip4/0.0.0.0/tcp/8001")]
    p2p_listen: String,
    
    /// Data directory
    #[arg(short, long, default_value = "~/.pumpchain")]
    data_dir: String,
    
    /// Bootstrap nodes (comma-separated multiaddrs)
    #[arg(long)]
    bootstrap: Option<String>,
    
    /// Solana L1 RPC URL for state anchoring
    #[arg(long, default_value = "https://api.mainnet-beta.solana.com")]
    solana_rpc: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    let args = Args::parse();
    
    info!("🚀 Starting PumpChain Validator v{}", env!("CARGO_PKG_VERSION"));
    info!("Chain ID: pumpchain-mainnet-beta");
    
    // Initialize state manager
    let state_manager = StateManager::new(&args.data_dir)?;
    info!("✅ State manager initialized");
    
    // Initialize SVM executor
    let executor = SvmExecutor::new(state_manager.clone())?;
    info!("✅ SVM executor initialized");
    
    // Initialize consensus
    let consensus = PumpBFT::new(
        &args.identity,
        state_manager.clone(),
        executor.clone(),
    )?;
    info!("✅ Consensus engine initialized");
    
    // Initialize P2P network
    let mut network = P2PNetwork::new(
        &args.p2p_listen,
        args.bootstrap.as_deref(),
        consensus.clone(),
    ).await?;
    info!("✅ P2P network initialized on {}", args.p2p_listen);
    
    // Initialize RPC server
    let rpc = JsonRpcServer::new(
        &args.rpc_bind,
        state_manager.clone(),
        executor.clone(),
    )?;
    info!("✅ RPC server initialized on http://{}", args.rpc_bind);
    
    // Start components
    let network_handle = tokio::spawn(async move {
        if let Err(e) = network.run().await {
            error!("P2P network error: {}", e);
        }
    });
    
    let consensus_handle = tokio::spawn(async move {
        if let Err(e) = consensus.run().await {
            error!("Consensus error: {}", e);
        }
    });
    
    let rpc_handle = tokio::spawn(async move {
        if let Err(e) = rpc.run().await {
            error!("RPC server error: {}", e);
        }
    });
    
    info!("✅ PumpChain validator is running!");
    info!("📡 RPC: http://{}", args.rpc_bind);
    info!("🌐 P2P: {}", args.p2p_listen);
    
    // Wait for all components
    tokio::try_join!(network_handle, consensus_handle, rpc_handle)?;
    
    Ok(())
}