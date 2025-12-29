use std::collections::HashMap;

pub type MerkleProof = Vec<[u8; 32]>;
pub type MerkleTreeData = HashMap<String, MerkleProof>; // key: leaf_hash, value: proof

// Helper function to encode bytes as hex string
fn hex_encode(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

#[derive(Clone, Debug)]
pub struct MerkleTree {
    pub root: [u8; 32],
    pub leaves: Vec<[u8; 32]>,
    pub proofs: HashMap<String, MerkleProof>,
}

impl MerkleTree {
    /// Create a leaf hash: keccak256(abi.encodePacked(telegram_id, token_amount))
    /// This matches Solidity's keccak256(abi.encodePacked(telegramId, tokenAmount))
    /// Note: Wallet address is not included in leaf to allow users to claim with any wallet
    pub fn create_leaf(telegram_id: &str, token_amount: u64) -> [u8; 32] {
        // Concatenate bytes (same as abi.encodePacked in Solidity)
        let mut data = Vec::new();
        data.extend_from_slice(telegram_id.as_bytes());
        data.extend_from_slice(&token_amount.to_be_bytes());
        
        // Hash with keccak256
        ethers::core::utils::keccak256(&data)
    }

    /// Build Merkle tree from leaves and generate proofs
    pub fn build(leaves: Vec<[u8; 32]>) -> Self {
        if leaves.is_empty() {
            return MerkleTree {
                root: [0; 32],
                leaves: Vec::new(),
                proofs: HashMap::new(),
            };
        }

        let mut current_level = leaves.clone();
        let mut proofs: HashMap<String, MerkleProof> = HashMap::new();
        let mut tree_levels: Vec<Vec<[u8; 32]>> = vec![leaves.clone()];

        // Build tree bottom-up
        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            let mut i = 0;

            while i < current_level.len() {
                if i + 1 < current_level.len() {
                    // Pair exists
                    let left = current_level[i];
                    let right = current_level[i + 1];
                    let parent = Self::hash_pair(&left, &right);
                    next_level.push(parent);
                } else {
                    // Odd number of nodes, duplicate the last one
                    let node = current_level[i];
                    let parent = Self::hash_pair(&node, &node);
                    next_level.push(parent);
                }
                i += 2;
            }

            tree_levels.push(next_level.clone());
            current_level = next_level;
        }

        let root = current_level[0];

        // Generate proofs for each leaf
        for (leaf_idx, leaf) in leaves.iter().enumerate() {
            let proof = Self::generate_proof_for_leaf(leaf_idx, &tree_levels);
            let leaf_hex = hex_encode(leaf);
            proofs.insert(leaf_hex, proof);
        }

        MerkleTree {
            root,
            leaves,
            proofs,
        }
    }

    /// Generate proof for a specific leaf
    fn generate_proof_for_leaf(leaf_idx: usize, tree_levels: &[Vec<[u8; 32]>]) -> MerkleProof {
        let mut proof = Vec::new();
        let mut current_idx = leaf_idx;

        for level in tree_levels.iter().take(tree_levels.len() - 1) {
            let sibling_idx = if current_idx % 2 == 0 {
                current_idx + 1
            } else {
                current_idx - 1
            };

            if sibling_idx < level.len() {
                proof.push(level[sibling_idx]);
            } else {
                // Odd number, sibling is the same node
                proof.push(level[current_idx]);
            }

            current_idx /= 2;
        }

        proof
    }

    /// Hash a pair of nodes (for Merkle tree construction)
    /// Uses keccak256 to match Solidity's keccak256
    fn hash_pair(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
        let mut data = Vec::new();
        if left < right {
            data.extend_from_slice(left);
            data.extend_from_slice(right);
        } else {
            data.extend_from_slice(right);
            data.extend_from_slice(left);
        }
        ethers::core::utils::keccak256(&data)
    }

    /// Get proof for a leaf
    pub fn get_proof(&self, leaf: &[u8; 32]) -> Option<MerkleProof> {
        let leaf_hex = hex_encode(leaf);
        self.proofs.get(&leaf_hex).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_tree_basic() {
        let leaf1 = MerkleTree::create_leaf("123", 1000);
        let leaf2 = MerkleTree::create_leaf("456", 2000);
        
        let tree = MerkleTree::build(vec![leaf1, leaf2]);
        
        assert_ne!(tree.root, [0; 32]);
        assert_eq!(tree.leaves.len(), 2);
    }
}

