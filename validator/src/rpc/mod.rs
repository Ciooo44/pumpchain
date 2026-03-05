use jsonrpc_core::{IoHandler, Params, Value};
use jsonrpc_http_server::{ServerBuilder, AccessControlAllowOrigin, DomainsValidation};
use std::sync::Arc;
use tracing::{info, debug};

use crate::state::StateManager;
use crate::execution::SvmExecutor;

/// JSON-RPC Server for PumpChain
pub struct JsonRpcServer {
    handler: IoHandler,
    state_manager: StateManager,
    executor: SvmExecutor,
}

impl JsonRpcServer {
    pub fn new(
        bind_addr: &str,
        state_manager: StateManager,
        executor: SvmExecutor,
    ) -> anyhow::Result<Self> {
        let mut handler = IoHandler::new();
        
        // Add Solana-compatible RPC methods
        Self::register_methods(&mut handler, state_manager.clone(), executor.clone());
        
        Ok(Self {
            handler,
            state_manager,
            executor,
        })
    }
    
    pub async fn run(&self) -> anyhow::Result<()> {
        info!("📡 Starting JSON-RPC server");
        
        let server = ServerBuilder::new(self.handler.clone())
            .cors(DomainsValidation::AllowOnly(vec![
                AccessControlAllowOrigin::Any,
            ]))
            .start_http(&"0.0.0.0:8899".parse()?)?;
        
        info!("📡 RPC server running on http://0.0.0.0:8899");
        
        server.wait();
        
        Ok(())
    }
    
    fn register_methods(
        handler: &mut IoHandler,
        state_manager: StateManager,
        _executor: SvmExecutor,
    ) {
        // Get latest blockhash
        handler.add_method("getLatestBlockhash", move |_params: Params| {
            debug!("RPC: getLatestBlockhash");
            
            // Return mock blockhash
            Ok(Value::Object({
                let mut map = serde_json::Map::new();
                map.insert("blockhash".to_string(), Value::String("PumpChainBlockhash123".to_string()));
                map.insert("lastValidBlockHeight".to_string(), Value::Number(serde_json::Number::from(1000)));
                map
            }))
        });
        
        // Get balance
        let state = state_manager.clone();
        handler.add_method("getBalance", move |params: Params| {
            let state = state.clone();
            
            async move {
                let pubkey_str = params.parse::<Vec<String>>()?.get(0).cloned().unwrap_or_default();
                let pubkey = bs58::decode(&pubkey_str).into_vec().unwrap_or_default();
                
                debug!("RPC: getBalance for {}", pubkey_str);
                
                let balance = if pubkey.len() == 32 {
                    match state.get_account(&pubkey.try_into().unwrap()).await {
                        Ok(Some(account)) => account.lamports,
                        _ => 0,
                    }
                } else {
                    0
                };
                
                Ok(Value::Object({
                    let mut map = serde_json::Map::new();
                    map.insert("context".to_string(), Value::Object({
                        let mut ctx = serde_json::Map::new();
                        ctx.insert("slot".to_string(), Value::Number(serde_json::Number::from(100)));
                        ctx
                    }));
                    map.insert("value".to_string(), Value::Number(serde_json::Number::from(balance)));
                    map
                }))
            }
        });
        
        // Get slot
        handler.add_method("getSlot", move |_params: Params| {
            debug!("RPC: getSlot");
            Ok(Value::Number(serde_json::Number::from(100)))
        });
        
        // Get version
        handler.add_method("getVersion", move |_params: Params| {
            debug!("RPC: getVersion");
            Ok(Value::Object({
                let mut map = serde_json::Map::new();
                map.insert("solana-core".to_string(), Value::String("pumpchain-0.1.0".to_string()));
                map.insert("feature-set".to_string(), Value::Number(serde_json::Number::from(0)));
                map
            }))
        });
        
        // Send transaction
        handler.add_method("sendTransaction", move |params: Params| {
            debug!("RPC: sendTransaction: {:?}", params);
            
            // Return mock signature
            Ok(Value::String("PumpChainTxSignature123".to_string()))
        });
        
        // Get account info
        let state = state_manager.clone();
        handler.add_method("getAccountInfo", move |params: Params| {
            let state = state.clone();
            
            async move {
                let pubkey_str = params.parse::<Vec<String>>()?.get(0).cloned().unwrap_or_default();
                let pubkey = bs58::decode(&pubkey_str).into_vec().unwrap_or_default();
                
                debug!("RPC: getAccountInfo for {}", pubkey_str);
                
                let result = if pubkey.len() == 32 {
                    match state.get_account(&pubkey.try_into().unwrap()).await {
                        Ok(Some(account)) => {
                            Value::Object({
                                let mut map = serde_json::Map::new();
                                map.insert("lamports".to_string(), Value::Number(serde_json::Number::from(account.lamports)));
                                map.insert("owner".to_string(), Value::String(bs58::encode(&account.owner).into_string()));
                                map.insert("executable".to_string(), Value::Bool(account.executable));
                                map.insert("data".to_string(), Value::Array(
                                    account.data.iter().map(|b| Value::Number(serde_json::Number::from(*b))).collect()
                                ));
                                map
                            })
                        }
                        _ => Value::Null,
                    }
                } else {
                    Value::Null
                };
                
                Ok(Value::Object({
                    let mut map = serde_json::Map::new();
                    map.insert("context".to_string(), Value::Object({
                        let mut ctx = serde_json::Map::new();
                        ctx.insert("slot".to_string(), Value::Number(serde_json::Number::from(100)));
                        ctx
                    }));
                    map.insert("value".to_string(), result);
                    map
                }))
            }
        });
    }
}