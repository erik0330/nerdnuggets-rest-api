# Merkle Tree Implementation Plan for 150k Users

## The Problem

With **150,000 users**, calling `setOwedAmount` 150,000 times would be:
- **Gas Cost**: 150,000 × ~50,000 gas = **7.5 billion gas** = **~$50,000+** (at current gas prices)
- **Time**: Would take weeks/months to process all transactions
- **Not Practical**: This approach doesn't scale

## Solution: Merkle Tree

### How It Works

1. **Generate Merkle Tree** (one-time, off-chain):
   - Create Merkle tree from all 150k users
   - Each leaf = `keccak256(telegram_id, wallet_address, token_amount)`
   - Store Merkle root in contract (32 bytes, one transaction)

2. **User Claims** (on-demand):
   - User gets Merkle proof from backend API
   - User calls `claimWithMerkleProof()` directly on contract
   - Contract verifies proof against stored root
   - If valid, transfers tokens

### Gas Cost Comparison

**Current Approach (setOwedAmount 150k times)**:
- 150,000 × 50,000 gas = **7.5 billion gas** = **~$50,000+**
- Backend pays for all transactions
- Takes weeks to complete

**Merkle Tree Approach**:
- Set root: 1 × 50,000 gas = **50,000 gas** = **~$0.50**
- Each claim: ~150,000 gas (user pays)
- **Total backend cost: ~$0.50** (just setting the root)
- Users pay their own gas when they claim

**Savings: ~$49,999.50** 🎉

## Implementation Steps

### 1. Contract Changes

Add Merkle tree verification to contract:

```solidity
bytes32 public merkleRoot;
mapping(address => bool) public hasClaimedWithMerkle;

function setMerkleRoot(bytes32 _root) external onlyOwner {
    merkleRoot = _root;
    emit MerkleRootUpdated(_root);
}

function claimWithMerkleProof(
    bytes32[] calldata proof,
    uint256 telegramId,
    uint256 tokenAmount
) external nonReentrant whenNotPaused {
    require(!hasClaimedWithMerkle[msg.sender], "Already claimed");
    require(merkleRoot != bytes32(0), "Merkle root not set");
    
    // Create leaf: keccak256(telegramId, msg.sender, tokenAmount)
    bytes32 leaf = keccak256(abi.encodePacked(telegramId, msg.sender, tokenAmount));
    
    // Verify Merkle proof
    require(verifyMerkleProof(proof, merkleRoot, leaf), "Invalid proof");
    
    // Mark as claimed
    hasClaimedWithMerkle[msg.sender] = true;
    
    // Transfer tokens
    require(balanceOf(address(this)) >= tokenAmount, "Insufficient tokens");
    _transfer(address(this), msg.sender, tokenAmount);
    totalClaimed += tokenAmount;
    
    emit ClaimedWithMerkle(msg.sender, telegramId, tokenAmount);
}

function verifyMerkleProof(
    bytes32[] calldata proof,
    bytes32 root,
    bytes32 leaf
) internal pure returns (bool) {
    bytes32 computedHash = leaf;
    
    for (uint256 i = 0; i < proof.length; i++) {
        bytes32 proofElement = proof[i];
        
        if (computedHash < proofElement) {
            computedHash = keccak256(abi.encodePacked(computedHash, proofElement));
        } else {
            computedHash = keccak256(abi.encodePacked(proofElement, computedHash));
        }
    }
    
    return computedHash == root;
}
```

### 2. Backend: Merkle Tree Generation

**New endpoint**: `/api/v2/merkle-proof?telegramId=X&walletAddress=Y`

**Implementation** (Rust):

```rust
use sha2::{Sha256, Digest};
use std::collections::HashMap;

// Generate Merkle tree from all holders
pub fn generate_merkle_tree(holders: Vec<Holder>) -> (bytes32, HashMap<String, Vec<bytes32>>) {
    let mut leaves = Vec::new();
    let mut proofs = HashMap::new();
    
    for holder in holders {
        // Create leaf: keccak256(telegram_id, wallet_address, token_amount)
        let leaf = create_leaf(
            &holder.telegram_id,
            &holder.wallet_address.unwrap_or_default(),
            holder.current_coins as u64
        );
        leaves.push(leaf);
    }
    
    // Build Merkle tree
    let root = build_merkle_tree(&leaves, &mut proofs);
    
    (root, proofs)
}

fn create_leaf(telegram_id: &str, wallet: &str, amount: u64) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(telegram_id.as_bytes());
    hasher.update(wallet.as_bytes());
    hasher.update(&amount.to_be_bytes());
    hasher.finalize().into()
}

// API endpoint to get proof
pub async fn get_merkle_proof(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<MerkleProofResponse>, ApiError> {
    let telegram_id = params.get("telegramId").ok_or_else(|| {
        ApiError::UserError(UserError::Str("telegramId required".to_string()))
    })?;
    
    let wallet = params.get("walletAddress").ok_or_else(|| {
        ApiError::UserError(UserError::Str("walletAddress required".to_string()))
    })?;
    
    // Get holder from DB
    let holder = state.service.holder.get_holder_by_telegram_id(telegram_id).await
        .ok_or_else(|| ApiError::DbError(DbError::Str("Holder not found".to_string())))?;
    
    // Generate proof (from pre-computed Merkle tree)
    let proof = state.merkle_tree.get_proof(telegram_id, wallet)?;
    
    Ok(Json(MerkleProofResponse {
        telegram_id: holder.telegram_id,
        wallet_address: wallet.to_string(),
        token_amount: holder.current_coins as u64,
        proof: proof,
        merkle_root: state.merkle_tree.root,
    }))
}
```

### 3. Frontend Changes

Update claim flow to use Merkle proof:

```typescript
// 1. Get Merkle proof from backend
const proofResponse = await fetch(
  `${API_URL}/api/v2/merkle-proof?telegramId=${telegramId}&walletAddress=${walletAddress}`
);
const { proof, tokenAmount, merkleRoot } = await proofResponse.json();

// 2. Call contract with proof
const contract = new ethers.Contract(CONTRACT_ADDRESS, ABI, signer);
const tx = await contract.claimWithMerkleProof(
  proof,           // bytes32[] proof
  telegramId,      // uint256 telegramId
  tokenAmount      // uint256 tokenAmount
);
await tx.wait();
```

## Implementation Priority

### Phase 1: Contract (Critical)
1. Add Merkle tree functions to contract
2. Deploy updated contract
3. Set Merkle root (one transaction for all 150k users)

### Phase 2: Backend (High Priority)
1. Generate Merkle tree from database
2. Store tree structure (for proof generation)
3. Add `/api/v2/merkle-proof` endpoint
4. Add `/api/v2/merkle-root` endpoint (for frontend to verify)

### Phase 3: Frontend (Medium Priority)
1. Update claim flow to fetch proof
2. Call `claimWithMerkleProof` instead of backend claim
3. Handle Merkle proof errors

## Migration Strategy

### Option 1: Clean Slate (Recommended)
- Generate Merkle tree for all 150k users
- Set root in contract
- Users claim with Merkle proof going forward
- No `setOwedAmount` needed

### Option 2: Hybrid
- Keep `setOwedAmount` for new users (if any)
- Use Merkle tree for existing 150k users
- More complex but allows flexibility

## Gas Cost Breakdown

**Setting Merkle Root** (one-time):
- Transaction: ~50,000 gas
- Cost: ~$0.50

**Each User Claim**:
- `claimWithMerkleProof`: ~150,000 gas
- User pays their own gas
- Backend pays nothing

**Total Backend Cost**: ~$0.50 (vs $50,000+ with setOwedAmount)

## Libraries Needed

**Rust** (Backend):
- `sha2` - Already in dependencies
- `merkletree` crate or custom implementation

**Solidity** (Contract):
- Standard library (no external dependencies needed)

## Timeline Estimate

- Contract changes: 2-3 hours
- Backend Merkle tree generation: 4-6 hours
- Backend API endpoints: 2-3 hours
- Frontend integration: 2-3 hours
- Testing: 2-3 hours

**Total: 1-2 days**

## Critical Notes

1. **Merkle Root Must Match**: The root stored in contract must match the tree generated from database
2. **Proof Generation**: Must be fast enough for real-time API responses
3. **Tree Updates**: If points change, need to regenerate tree and update root
4. **Security**: Verify proof correctly in contract (double-check implementation)

## Next Steps

1. ✅ Implement Merkle tree in contract
2. ✅ Generate Merkle tree from database
3. ✅ Create proof generation API
4. ✅ Update frontend to use Merkle proofs
5. ✅ Test with small subset first
6. ✅ Deploy and set root for all 150k users

