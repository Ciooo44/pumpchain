use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use rocksdb::DB;
use tracing::info;

/// State Manager - Handles persistent storage
pub struct StateManager {
    db: Arc<RwLock<DB>>,
}

impl StateManager {
    pub fn new(data_dir: &str) -> anyhow::Result<Self> {
        info!("💾 Initializing state manager at {}", data_dir);
        
        let path = shellexpand::tilde(data_dir);
        let db_path = Path::new(path.as_ref()).join("db");
        
        // Create directory if not exists
        std::fs::create_dir_all(&db_path)?;
        
        let db = DB::open_default(&db_path)?;
        
        Ok(Self {
            db: Arc::new(RwLock::new(db)),
        })
    }
    
    pub async fn get(&self, key: &[u8]) -> anyhow::Result<Option<Vec<u8>>> {
        let db = self.db.read().await;
        Ok(db.get(key)?)
    }
    
    pub async fn put(&self, key: &[u8], value: &[u8]) -> anyhow::Result<()> {
        let db = self.db.write().await;
        db.put(key, value)?;
        Ok(())
    }
    
    pub async fn get_account(&self, pubkey: &[u8; 32]) -> anyhow::Result<Option<Account>> {
        let key = format!("account:{}", bs58::encode(pubkey).into_string());
        let data = self.get(key.as_bytes()).await?;
        
        match data {
            Some(bytes) => {
                let account: Account = bincode::deserialize(&bytes)?;
                Ok(Some(account))
            }
            None => Ok(None),
        }
    }
    
    pub async fn put_account(&self, pubkey: &[u8; 32], account: &Account) -> anyhow::Result<()> {
        let key = format!("account:{}", bs58::encode(pubkey).into_string());
        let value = bincode::serialize(account)?;
        self.put(key.as_bytes(), &value).await
    }
}

impl Clone for StateManager {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Account {
    pub owner: [u8; 32],
    pub lamports: u64,
    pub data: Vec<u8>,
    pub executable: bool,
    pub rent_epoch: u64,
}

// Add bs58 dependency
use bs58;