# Merkle Tree: Payment & Gas Fee Explanation

## Current Implementation (setOwedAmount)

### Payment Flow:
1. User wants to claim tokens (has points in database)
2. User pays **$3.5** → Goes to `PAYMENT_RECIPIENT` (you get the money) ✅
3. Backend calls `setOwedAmount(wallet, tokens)` → **Backend pays gas** (~50k gas)
4. User calls `claim()` → **User pays gas** (~100k gas)
5. User receives tokens

**Total Gas:**
- Backend pays: ~50k gas per user
- User pays: ~100k gas per user
- **Total cost to backend for 150k users: ~$50,000** ❌

## Merkle Tree Implementation

### Payment Flow:
1. User wants to claim tokens (has points in database)
2. User gets Merkle proof from backend API (free, no transaction)
3. User calls `claimWithMerkleProof()` directly → **User pays $3.5 + gas** (~150k gas)
4. Contract verifies proof and transfers tokens
5. **$3.5 goes to `PAYMENT_RECIPIENT`** (you still get it!) ✅

**Total Gas:**
- Backend pays: **$0** (only sets root once for all users) ✅
- User pays: ~150k gas per user
- **Total cost to backend: ~$0.50** (one root transaction) ✅

## Key Points

### ✅ You Still Get $3.5 Per Claim
- The $3.5 payment is **separate from gas fees**
- It's the claim price that goes to `PAYMENT_RECIPIENT`
- Users pay this when they call `claimWithMerkleProof()`
- **You receive the payment, not the gas fees**

### ✅ Users Pay Their Own Gas
- With Merkle tree, users call contract directly
- They pay their own gas fees (~$1-3 depending on network)
- This is standard - users always pay gas for their transactions
- You don't pay gas for their claims

### ✅ Database Can Handle On-Demand Claims
- Merkle proofs are generated **from the database on-demand**
- When user requests proof, backend queries database
- Generates proof in milliseconds
- No problem handling thousands of requests

## Updated Contract Function

```solidity
function claimWithMerkleProof(
    bytes32[] calldata proof,
    uint256 telegramId,
    uint256 tokenAmount,
    address paymentToken  // User still pays $3.5
) external payable nonReentrant whenNotPaused {
    require(!hasClaimedWithMerkle[msg.sender], "Already claimed");
    require(merkleRoot != bytes32(0), "Merkle root not set");
    
    // Verify Merkle proof
    bytes32 leaf = keccak256(abi.encodePacked(telegramId, msg.sender, tokenAmount));
    require(verifyMerkleProof(proof, merkleRoot, leaf), "Invalid proof");
    
    // Handle payment (same as current claim() function)
    require(acceptedPaymentTokens[paymentToken], "Payment token not accepted");
    uint256 pricePerClaim = claimPricePerToken[paymentToken];
    require(pricePerClaim > 0, "Price not set");
    
    if (paymentToken == address(0)) {
        // ETH payment
        require(msg.value >= pricePerClaim, "Insufficient ETH sent");
        (bool success, ) = payable(PAYMENT_RECIPIENT).call{value: pricePerClaim}("");
        require(success, "Payment failed");
        // Refund excess
        if (msg.value > pricePerClaim) {
            (bool refundSuccess, ) = payable(msg.sender).call{value: msg.value - pricePerClaim}("");
            require(refundSuccess, "Refund failed");
        }
    } else {
        // ERC20 payment
        require(msg.value == 0, "Do not send ETH when paying with ERC20");
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
```

## Cost Comparison for 150k Users

### Current (setOwedAmount):
- Backend gas: 150k × 50k gas = **7.5 billion gas ≈ $50,000** ❌
- Revenue: 150k × $3.5 = **$525,000** ✅
- **Net (after gas): $475,000** ✅
- But backend has to pay $50k upfront

### Merkle Tree:
- Backend gas: 1 × 50k gas = **50k gas ≈ $0.50** ✅
- Revenue: 150k × $3.5 = **$525,000** ✅ (same!)
- **Net (after gas): $524,999.50** ✅
- Backend pays almost nothing

**You make the same revenue, but save $49,999.50 in gas costs!**

## On-Demand Claims

### Database Can Handle This ✅

**Proof Generation:**
- Backend queries database: `SELECT * FROM holder WHERE telegram_id = ?`
- Generates Merkle proof from pre-computed tree
- Returns proof to user
- Takes ~10-50ms per request
- Database can easily handle 1000+ requests/second

**Merkle Tree Structure:**
- Generate tree once (takes ~5-10 minutes for 150k users)
- Store tree structure in memory/cache
- Generate proofs on-demand from stored tree
- Each proof is ~20-30 bytes (very small)

**Scalability:**
- PostgreSQL can handle thousands of queries/second
- Merkle proof generation is fast (milliseconds)
- Can add caching layer if needed
- Can generate proofs in batches

## Implementation Strategy

### Phase 1: Generate Merkle Tree
1. Query all holders from database
2. Generate Merkle tree (one-time, ~5-10 minutes)
3. Store tree structure (for proof generation)
4. Set Merkle root in contract (one transaction)

### Phase 2: On-Demand Proofs
1. User requests proof via API
2. Backend queries database for holder data
3. Backend generates proof from stored tree
4. Returns proof to user
5. User calls contract with proof

### Phase 3: User Claims
1. User calls `claimWithMerkleProof(proof, telegramId, amount, paymentToken)`
2. User pays $3.5 + gas (user pays, you get $3.5)
3. Contract verifies proof
4. Contract transfers tokens
5. Contract sends $3.5 to `PAYMENT_RECIPIENT`

## Summary

✅ **You still get $3.5 per claim** (same revenue)  
✅ **Users pay their own gas** (standard practice)  
✅ **Backend saves $50k in gas costs**  
✅ **Database can handle on-demand claims** (very efficient)  
✅ **Merkle tree is perfect for this use case**

The only difference: Backend doesn't need to call `setOwedAmount` anymore (saves you $50k), and users pay their own gas (which they do anyway).

