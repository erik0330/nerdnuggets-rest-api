use crate::pool::DatabasePool;
use crate::repository::holder_repository::HolderRepository;
use merkle::MerkleTree;
use std::sync::Arc;
use types::error::ApiError;

/// Service for managing Merkle tree for Telegram points claims
#[derive(Clone)]
pub struct MerkleService {
    db_conn: Arc<DatabasePool>,
    current_tree: Arc<tokio::sync::RwLock<Option<MerkleTree>>>,
}

impl MerkleService {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
            current_tree: Arc::new(tokio::sync::RwLock::new(None)),
        }
    }

    /// Generate Merkle tree from all holders in database
    /// This should be called periodically (e.g., daily) to update the tree
    pub async fn generate_tree(&self) -> Result<MerkleTree, ApiError> {
        let holder_repo = HolderRepository::new(&self.db_conn);
        
        // Get all holders from database
        let holders = holder_repo.get_holders(None, None).await
            .map_err(|e| ApiError::DbError(types::error::DbError::Str(e.to_string())))?;

        // Filter holders with points who haven't claimed yet
        let mut leaves = Vec::new();

        for holder in holders {
            // Only include holders with points who haven't claimed yet
            if holder.current_coins > 0 && holder.claimed_at.is_none() {
                // Create leaf: keccak256(telegram_id, token_amount)
                // Wallet address is not part of leaf (users can claim with any wallet)
                let leaf = MerkleTree::create_leaf(
                    &holder.telegram_id,
                    holder.current_coins as u64,
                );
                leaves.push(leaf);
            }
        }

        // Build Merkle tree
        let tree = MerkleTree::build(leaves);

        // Store tree in memory
        let mut current = self.current_tree.write().await;
        *current = Some(tree.clone());

        Ok(tree)
    }

    /// Get Merkle proof for a specific holder
    pub async fn get_proof(
        &self,
        telegram_id: &str,
        token_amount: u64,
    ) -> Result<Vec<[u8; 32]>, ApiError> {
        // Check if tree exists
        let tree_guard = self.current_tree.read().await;
        let tree = tree_guard.as_ref()
            .ok_or_else(|| ApiError::UserError(types::error::UserError::Str("Merkle tree not initialized".to_string())))?;

        // Create leaf hash: keccak256(telegram_id, token_amount)
        let leaf = MerkleTree::create_leaf(telegram_id, token_amount);

        // Get proof
        tree.get_proof(&leaf)
            .ok_or_else(|| ApiError::UserError(types::error::UserError::Str("Proof not found for this user".to_string())))
    }

    /// Get current Merkle root
    pub async fn get_root(&self) -> Option<[u8; 32]> {
        let tree_guard = self.current_tree.read().await;
        tree_guard.as_ref().map(|t| t.root)
    }
}
