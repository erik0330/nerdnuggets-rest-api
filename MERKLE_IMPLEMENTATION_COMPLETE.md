# Merkle Tree Implementation - Status

## ✅ Completed

1. **Contract Changes** (`frontend/contracts/NerdToken.sol`)
   - ✅ Added `merkleRoot`, `merkleRootLastUpdated`, `hasClaimedWithMerkle`
   - ✅ Added `setMerkleRoot()` function (onlyOwner)
   - ✅ Added `claimWithMerkleProof()` function with payment logic
   - ✅ Added `verifyMerkleProof()` internal function
   - ✅ Added events: `MerkleRootUpdated`, `ClaimedWithMerkle`
   - ✅ All existing functionality preserved

2. **Backend Library** (`backend/libraries/merkle`)
   - ✅ Created MerkleTree library
   - ✅ Implemented build, create_leaf, get_proof
   - ✅ Leaf format: `sha256(telegram_id, token_amount)` (Note: Contract uses keccak256 - this needs to match)

3. **Backend Service** (`backend/database/src/service/merkle_service.rs`)
   - ✅ Created MerkleService
   - ✅ Added to AppService
   - ✅ Implemented generate_tree(), get_proof(), get_root()

4. **EVM Client** (`backend/libraries/evm/src/lib.rs`)
   - ✅ Added `set_merkle_root()` method
   - ✅ Updated contract ABI

5. **API Endpoint** (`backend/server/src/handler/merkle_handler.rs`)
   - ✅ Created `get_merkle_proof` handler
   - ✅ Added route `/merkle-proof`
   - ✅ Returns proof, root, telegramId, tokenAmount

6. **DTOs** (`backend/libraries/types/src/dto/holder_dto.rs`)
   - ✅ Added `MerkleProofResponse`

## ⚠️ IMPORTANT: Hash Function Mismatch

**Current Issue:**
- Backend uses **SHA256** for Merkle tree leaves
- Solidity contract uses **keccak256** for verification

**Fix Required:**
The backend Merkle tree library must use **Keccak256** (same as Solidity's keccak256) instead of SHA256.

**Solution:**
1. Add `tiny-keccak` or `sha3` crate to `backend/libraries/merkle/Cargo.toml`
2. Replace SHA256 with Keccak256 in `create_leaf()` and `hash_pair()`

## ⏳ TODO: Scheduled Job

**Daily Merkle Root Update:**
- Add cron job that runs at 2 AM UTC daily
- Calls `generate_tree()` and `set_merkle_root()` on contract
- Should be added to `backend/server/src/lib.rs` or `main.rs`

**Implementation:**
```rust
use tokio_cron_scheduler::{Job, JobScheduler};

async fn setup_merkle_root_updater(state: Arc<AppState>) {
    let sched = JobScheduler::new().await.unwrap();
    
    sched.add(
        Job::new_async("0 0 2 * * *", move |_uuid, _l| {
            let state = state.clone();
            Box::pin(async move {
                update_merkle_root_daily(state).await;
            })
        }).unwrap()
    ).await.unwrap();
    
    sched.start().await.unwrap();
}
```

## Testing Checklist

- [ ] Fix hash function (SHA256 → Keccak256)
- [ ] Test Merkle tree generation with real data
- [ ] Test proof generation
- [ ] Test contract verification
- [ ] Test scheduled job
- [ ] Verify existing functionality still works (setOwedAmount, claim, etc.)

## Deployment Notes

1. **Before deploying:**
   - Fix hash function mismatch
   - Test thoroughly
   - Generate initial Merkle tree
   - Set initial root in contract

2. **Environment variables:**
   - No new env vars needed (uses existing wallet for setMerkleRoot)

3. **Funding:**
   - Pre-fund wallet with ~$20-30 for daily updates (2-3 weeks)
   - Wallet must be contract owner

4. **Initial setup:**
   - Generate Merkle tree for all current holders
   - Call `setMerkleRoot()` on contract
   - Start scheduled job for daily updates

## Next Steps

1. Fix hash function mismatch (CRITICAL)
2. Add scheduled job for daily updates
3. Test end-to-end
4. Deploy

