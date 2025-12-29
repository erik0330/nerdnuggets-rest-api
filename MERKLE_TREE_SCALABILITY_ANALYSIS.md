# Merkle Tree Scalability for 1.3M+ Dynamic Users

## The Challenge

You have:
- **150k initial users** (need immediate solution)
- **Growing to 1.3M+ users** (constantly adding new users)
- **Each user can claim multiple times** ($3.5 per claim, then $7 after 5% threshold)
- **Price increases** when 5% of supply is claimed
- **Users constantly added** from game play

## Problem with Static Merkle Tree

A **static Merkle tree** (one root for all users) won't work because:
- ❌ New users can't be added (would need new root = new contract transaction)
- ❌ Can't track multiple claims per user
- ❌ Would need to regenerate tree every time a new user is added
- ❌ Very expensive and impractical

## Solution: Hybrid Approach

### Phase 1: Merkle Tree for Initial 150k Users (One-Time Claims)

**For Telegram points claims** (one-time based on points):

1. Generate Merkle tree for initial 150k users
2. Each leaf = `keccak256(telegram_id, wallet_address, token_amount)`
3. Set root once in contract
4. Users claim their allocated amount once (based on Telegram points)

**Benefits:**
- ✅ Handles initial 150k users efficiently (~$0.50 backend cost)
- ✅ Users pay $3.5 + gas (you get $3.5, they pay gas)
- ✅ One-time claim per user (their points allocation)

### Phase 2: setOwedAmount for New Users (After Initial Batch)

**For new users joining after initial launch:**

1. New users play games → get points in database
2. When they want to claim → Backend calls `setOwedAmount()` for that specific user
3. User claims their allocated amount (pays $3.5 or $7 depending on threshold)
4. Only called for NEW users (much smaller number)

**Why this works:**
- New users trickle in (maybe 100-1000 per day)
- Much cheaper than 150k upfront transactions
- Only pay gas for users who actually claim
- Still efficient for smaller numbers

### Phase 3: Regular Claims (Multiple Claims Per User)

**For regular claims** (4B tokens per claim, multiple times):

1. Users can claim multiple times (not based on Telegram points)
2. Each claim = $3.5 (or $7 after 5% threshold)
3. Uses existing `claim()` function (no Merkle tree needed)
4. This is separate from Telegram points claims

## Implementation Strategy

### Option A: Hybrid Merkle Tree + setOwedAmount (Recommended)

```
Initial 150k users (Telegram points):
  → Merkle tree (one-time, saves $50k)
  → One claim per user (their points allocation)
  → Users pay $3.5 + gas

New users (after launch):
  → setOwedAmount (only for users who claim)
  → Much smaller number (e.g., 1000 new claims/day)
  → Gas cost: ~$500/day (much cheaper than $50k upfront)

Regular claims (multiple times):
  → Existing claim() function
  → $3.5 or $7 per claim
  → No Merkle tree needed
```

**Cost Breakdown:**
- Initial 150k: ~$0.50 (Merkle tree)
- New users (1000/day): ~$500/day (setOwedAmount)
- After 30 days: ~$15,000 (vs $50k upfront)
- **Still much cheaper than $50k upfront for all 150k**

### Option B: Periodic Merkle Root Updates

Update Merkle root periodically (e.g., weekly):

1. Week 1: Generate tree for users 1-150k
2. Week 2: Generate new tree for users 1-160k (adds 10k new)
3. Week 3: Generate new tree for users 1-170k
4. etc.

**Problems:**
- ❌ Need contract transaction to update root each week
- ❌ All users need new proofs (inconvenient)
- ❌ More complex implementation
- ❌ Not worth it for incremental growth

### Option C: Keep setOwedAmount for All (Not Recommended)

Keep current approach for all users:
- 150k × $0.50 = $75k upfront (very expensive)
- 1.3M × $0.50 = $650k (extremely expensive)
- Not scalable

## Recommended Approach: Hybrid

### Contract Changes

```solidity
// Merkle tree for initial batch
bytes32 public merkleRoot;
mapping(address => bool) public hasClaimedWithMerkle;

// Keep existing setOwedAmount for new users
mapping(address => uint256) public owedAmounts; // Still exists
mapping(address => bool) public hasClaimedOwedAmount; // Still exists

function claimWithMerkleProof(
    bytes32[] calldata proof,
    uint256 telegramId,
    uint256 tokenAmount,
    address paymentToken
) external payable nonReentrant whenNotPaused {
    require(!hasClaimedWithMerkle[msg.sender], "Already claimed");
    require(merkleRoot != bytes32(0), "Merkle root not set");
    
    // Verify proof
    bytes32 leaf = keccak256(abi.encodePacked(telegramId, msg.sender, tokenAmount));
    require(verifyMerkleProof(proof, merkleRoot, leaf), "Invalid proof");
    
    // Handle payment (same as regular claim)
    require(acceptedPaymentTokens[paymentToken], "Payment token not accepted");
    uint256 pricePerClaim = claimPricePerToken[paymentToken]; // $3.5 or $7
    require(pricePerClaim > 0, "Price not set");
    
    // Process payment (same as _processClaim)
    if (paymentToken == address(0)) {
        require(msg.value >= pricePerClaim, "Insufficient ETH");
        (bool success, ) = payable(PAYMENT_RECIPIENT).call{value: pricePerClaim}("");
        require(success, "Payment failed");
        if (msg.value > pricePerClaim) {
            (bool refundSuccess, ) = payable(msg.sender).call{value: msg.value - pricePerClaim}("");
            require(refundSuccess, "Refund failed");
        }
    } else {
        IERC20 token = IERC20(paymentToken);
        token.safeTransferFrom(msg.sender, PAYMENT_RECIPIENT, pricePerClaim);
    }
    
    // Mark as claimed
    hasClaimedWithMerkle[msg.sender] = true;
    
    // Transfer tokens
    require(balanceOf(address(this)) >= tokenAmount, "Insufficient tokens");
    _transfer(address(this), msg.sender, tokenAmount);
    totalClaimed += tokenAmount;
    
    emit ClaimedWithMerkle(msg.sender, telegramId, tokenAmount, paymentToken, pricePerClaim);
}

// Keep existing setOwedAmount for new users
function setOwedAmount(address _user, uint256 _amount) external onlyOwner {
    // Existing implementation - for new users after initial batch
}
```

### Backend Logic

```rust
// Check if user is in initial Merkle tree
if user.telegram_id in initial_merkle_tree_users {
    // Return Merkle proof (no backend transaction needed)
    return get_merkle_proof(user);
} else {
    // New user - use setOwedAmount
    // Backend calls setOwedAmount (only for users who actually claim)
    evm.set_owed_amount(wallet, tokens).await?;
}
```

## Cost Analysis

### Scenario: 150k initial + 1.3M total over 6 months

**Initial 150k users (Merkle tree):**
- Backend cost: ~$0.50 (one root transaction)
- Users pay: $3.5 + gas each

**New users (1,150,000 users over 6 months = ~6,400/day):**
- Only users who claim need setOwedAmount
- Assuming 50% claim rate = ~3,200 claims/day
- Gas cost: ~3,200 × $0.50 = **~$1,600/day**
- Over 6 months: ~$290k

**Total cost:**
- Initial: $0.50
- New users: ~$290k
- **Total: ~$290k** (vs $650k if all use setOwedAmount)

**Savings: ~$360k** vs processing all 1.3M upfront

## Price Changes ($3.5 → $7)

The price change logic already exists in the contract:

```solidity
uint256 pricePerClaim = claimPricePerToken[paymentToken]; // Automatically $7 after threshold
```

**Both Merkle tree claims and setOwedAmount claims use the same payment logic**, so:
- ✅ Price automatically increases to $7 after 5% threshold
- ✅ Users pay correct price (whether using Merkle or setOwedAmount)
- ✅ No changes needed - existing logic handles it

## Summary

**Recommended Approach: Hybrid**

1. ✅ **Initial 150k users**: Merkle tree (saves $50k, one-time)
2. ✅ **New users**: setOwedAmount (only for users who claim, much cheaper)
3. ✅ **Regular claims**: Existing claim() function (unchanged)
4. ✅ **Price changes**: Existing logic works for both (no changes needed)
5. ✅ **Database**: Handles all cases (Merkle proofs generated on-demand)

**You still get:**
- ✅ $3.5 (or $7) per claim (same revenue)
- ✅ Users pay their own gas
- ✅ Price increases work automatically
- ✅ Can handle unlimited users

**Cost savings:**
- Initial 150k: $50k → $0.50 (99.999% savings)
- New users: Only pay for users who actually claim
- Much more scalable and cost-effective

