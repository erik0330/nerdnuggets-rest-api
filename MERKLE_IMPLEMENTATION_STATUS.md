# Merkle Tree Implementation Status

## ✅ Completed

1. **Contract Changes**
   - ✅ Added `merkleRoot` and `merkleRootLastUpdated` state variables
   - ✅ Added `hasClaimedWithMerkle` mapping
   - ✅ Added `setMerkleRoot()` function (onlyOwner)
   - ✅ Added `claimWithMerkleProof()` function with payment logic
   - ✅ Added `verifyMerkleProof()` internal function
   - ✅ Added events: `MerkleRootUpdated`, `ClaimedWithMerkle`
   - ✅ All existing functionality preserved (setOwedAmount still works)

2. **Backend Library**
   - ✅ Created `backend/libraries/merkle` crate
   - ✅ Implemented MerkleTree with build, create_leaf, get_proof
   - ✅ Leaf format: `keccak256(telegram_id, token_amount)` (wallet not in leaf)

3. **Backend Service**
   - ✅ Created `MerkleService` in `backend/database/src/service/merkle_service.rs`
   - ✅ Added to `AppService`
   - ✅ Implemented `generate_tree()`, `get_proof()`, `get_root()`

4. **EVM Client**
   - ✅ Added `set_merkle_root()` method to EVMClient
   - ✅ Updated contract ABI to include `setMerkleRoot`

## ⏳ In Progress / TODO

5. **API Endpoint** (Next)
   - ⏳ Add `/api/v2/merkle-proof` endpoint
   - ⏳ Return proof, root, telegramId, tokenAmount

6. **Scheduled Job** (Next)
   - ⏳ Add daily cron job to update Merkle root
   - ⏳ Runs at 2 AM UTC
   - ⏳ Generates tree and calls `set_merkle_root()`

7. **Frontend Integration**
   - ⏳ Update claim flow to use Merkle proofs
   - ⏳ Add API call to get proof
   - ⏳ Call `claimWithMerkleProof()` on contract

8. **Testing**
   - ⏳ Test Merkle tree generation
   - ⏳ Test proof generation
   - ⏳ Test contract verification
   - ⏳ Verify existing functionality still works

## Important Notes

### Leaf Format
- **Leaf = keccak256(telegram_id, token_amount)**
- Wallet address is NOT in leaf (allows users to claim with any wallet)
- This means one Telegram ID can only claim once (prevented by `hasClaimedWithMerkle`)

### Contract Function Signature
```solidity
function claimWithMerkleProof(
    bytes32[] calldata proof,
    uint256 telegramId,
    uint256 tokenAmount,
    address paymentToken,
    address referrer  // optional
) external payable
```

### Backend Flow
1. User requests proof via API: `/api/v2/merkle-proof?telegramId=X`
2. Backend queries database for holder
3. Backend generates proof from current Merkle tree
4. Returns proof, root, telegramId, tokenAmount
5. User calls contract with proof

### Daily Update Flow
1. Cron job runs at 2 AM UTC
2. Queries all holders from database (unclaimed only)
3. Generates new Merkle tree
4. Calls `set_merkle_root()` on contract
5. Stores tree in memory for proof generation

## Next Steps

1. Add DTO for Merkle proof response
2. Add API endpoint handler
3. Add route to auth.rs
4. Add scheduled job to main.rs or lib.rs
5. Test compilation
6. Update frontend

