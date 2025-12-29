# Merkle Tree: Hourly/Daily Updates for 2-3 Week Game Period

## Timeline & Update Frequency

### Game Duration: 2-3 Weeks

**Update Frequency Options:**

**Option 1: Hourly Updates** (Most Responsive)
- Updates every hour
- New users can claim within 1 hour of earning points
- 2 weeks = 336 updates
- 3 weeks = 504 updates
- Cost: 336 × $0.50 = **$168** (2 weeks) or 504 × $0.50 = **$252** (3 weeks)

**Option 2: Daily Updates** (Recommended Balance)
- Updates once per day (e.g., 2 AM UTC)
- New users can claim within 24 hours
- 2 weeks = 14 updates
- 3 weeks = 21 updates
- Cost: 14 × $0.50 = **$7** (2 weeks) or 21 × $0.50 = **$10.50** (3 weeks)

**Option 3: Every 6 Hours** (Middle Ground)
- Updates 4 times per day
- New users can claim within 6 hours
- 2 weeks = 56 updates
- 3 weeks = 84 updates
- Cost: 56 × $0.50 = **$28** (2 weeks) or 84 × $0.50 = **$42** (3 weeks)

## Recommended: Daily Updates

**Why Daily:**
- ✅ **Cost-effective**: Only $7-10.50 for entire game period
- ✅ **Reasonable delay**: 24 hours is acceptable for most users
- ✅ **Simple**: One update per day is easy to manage
- ✅ **Still very cheap**: $0.50/day is negligible

**If you need faster updates:**
- Every 6 hours: $28-42 (still very reasonable)
- Hourly: $168-252 (more expensive but still much cheaper than setOwedAmount)

## Funding Strategy

### When to Fund Merkle Tree Updates

**Option 1: Pre-fund Wallet** (Recommended)
- Before game starts: Fund backend wallet with enough ETH for all updates
- 2 weeks (daily): ~$10-15 (includes buffer)
- 3 weeks (daily): ~$15-20 (includes buffer)
- 2 weeks (hourly): ~$200-250 (includes buffer)
- 3 weeks (hourly): ~$300-350 (includes buffer)

**Option 2: Fund as Needed**
- Monitor wallet balance
- Auto-fund when balance drops below threshold
- More complex but flexible

**Option 3: Use Revenue to Fund**
- Use $3.5 payments to fund future updates
- Set aside portion of revenue for gas costs
- Self-sustaining approach

### Recommended: Pre-fund with Buffer

**For Daily Updates (2-3 weeks):**
- Pre-fund: **$20-30** (includes 2x buffer for safety)
- Covers: 21-30 days of updates
- Very affordable

**For Hourly Updates (2-3 weeks):**
- Pre-fund: **$400-500** (includes 2x buffer)
- Covers: 2-3 weeks of hourly updates
- Still much cheaper than setOwedAmount

## Cost Breakdown

### Scenario: 150k initial + 1.15M new users over 2-3 weeks

**Daily Updates:**
- Initial tree: $0.50
- Daily updates (21 days): 21 × $0.50 = $10.50
- **Total: ~$11** 🎉

**Hourly Updates:**
- Initial tree: $0.50
- Hourly updates (504 hours): 504 × $0.50 = $252
- **Total: ~$252.50**

**vs setOwedAmount:**
- 1.3M users: ~$650,000
- **Savings: ~$649,989** (daily) or **~$649,747** (hourly)

## Implementation: Daily Updates

### Backend: Scheduled Job

```rust
// Cargo.toml - Add tokio-cron-scheduler
// tokio-cron-scheduler = "0.13.0"

use tokio_cron_scheduler::{Job, JobScheduler};

// Schedule daily Merkle root update at 2 AM UTC
async fn setup_merkle_root_updater(evm_client: Arc<EVMClient>, db: Arc<DatabasePool>) {
    let sched = JobScheduler::new().await.unwrap();
    
    // Run daily at 2 AM UTC
    sched.add(
        Job::new_async("0 0 2 * * *", move |_uuid, _l| {
            let evm = evm_client.clone();
            let db_pool = db.clone();
            
            Box::pin(async move {
                update_merkle_root(evm, db_pool).await;
            })
        }).unwrap()
    ).await.unwrap();
    
    sched.start().await.unwrap();
}

async fn update_merkle_root(evm: Arc<EVMClient>, db: Arc<DatabasePool>) {
    tracing::info!("Starting daily Merkle root update...");
    
    // 1. Query ALL users from database
    let all_users = db.get_all_holders().await?;
    tracing::info!("Found {} users in database", all_users.len());
    
    // 2. Generate Merkle tree
    let start = std::time::Instant::now();
    let (root, tree_structure) = generate_merkle_tree(all_users)?;
    let duration = start.elapsed();
    tracing::info!("Merkle tree generated in {:?} ({} users)", duration, all_users.len());
    
    // 3. Store tree structure (for proof generation)
    cache.set_merkle_tree(tree_structure, root).await?;
    
    // 4. Update root in contract
    match evm.set_merkle_root(root).await {
        Ok(tx_hash) => {
            tracing::info!("Merkle root updated successfully: {} (tx: {})", root, tx_hash);
        }
        Err(e) => {
            tracing::error!("Failed to update Merkle root: {}", e);
            // Don't update cache if contract update failed
            // Will retry next day
        }
    }
}
```

### Contract: Root Update Function

```solidity
bytes32 public merkleRoot;
uint256 public merkleRootLastUpdated; // Timestamp of last update
mapping(address => bool) public hasClaimedWithMerkle; // Prevent double claims

event MerkleRootUpdated(bytes32 indexed newRoot, uint256 timestamp);

function setMerkleRoot(bytes32 _newRoot) external onlyOwner {
    merkleRoot = _newRoot;
    merkleRootLastUpdated = block.timestamp;
    emit MerkleRootUpdated(_newRoot, block.timestamp);
}

function claimWithMerkleProof(
    bytes32[] calldata proof,
    uint256 telegramId,
    uint256 tokenAmount,
    address paymentToken
) external payable nonReentrant whenNotPaused {
    require(merkleRoot != bytes32(0), "Merkle root not set");
    require(!hasClaimedWithMerkle[msg.sender], "Already claimed");
    
    // Verify Merkle proof
    bytes32 leaf = keccak256(abi.encodePacked(telegramId, msg.sender, tokenAmount));
    require(verifyMerkleProof(proof, merkleRoot, leaf), "Invalid proof");
    
    // Mark as claimed
    hasClaimedWithMerkle[msg.sender] = true;
    
    // Process payment (same as regular claim - $3.5 or $7)
    require(acceptedPaymentTokens[paymentToken], "Payment token not accepted");
    uint256 pricePerClaim = claimPricePerToken[paymentToken];
    require(pricePerClaim > 0, "Price not set");
    
    // Handle payment
    if (paymentToken == address(0)) {
        require(msg.value >= pricePerClaim, "Insufficient ETH sent");
        (bool success, ) = payable(PAYMENT_RECIPIENT).call{value: pricePerClaim}("");
        require(success, "Payment failed");
        if (msg.value > pricePerClaim) {
            (bool refundSuccess, ) = payable(msg.sender).call{value: msg.value - pricePerClaim}("");
            require(refundSuccess, "Refund failed");
        }
    } else {
        require(msg.value == 0, "Do not send ETH when paying with ERC20");
        IERC20 token = IERC20(paymentToken);
        token.safeTransferFrom(msg.sender, PAYMENT_RECIPIENT, pricePerClaim);
    }
    
    // Transfer tokens
    require(balanceOf(address(this)) >= tokenAmount, "Insufficient tokens");
    _transfer(address(this), msg.sender, tokenAmount);
    totalClaimed += tokenAmount;
    
    emit ClaimedWithMerkle(msg.sender, telegramId, tokenAmount, paymentToken, pricePerClaim);
}
```

## Funding Setup

### Step 1: Calculate Required Funds

**For Daily Updates (2-3 weeks):**
```
21 days × $0.50 = $10.50
+ 50% buffer = $15.75
+ Network fees = ~$20-25 total
```

**For Hourly Updates (2-3 weeks):**
```
504 hours × $0.50 = $252
+ 50% buffer = $378
+ Network fees = ~$400-500 total
```

### Step 2: Fund Backend Wallet

**Before game starts:**
1. Send ETH to backend wallet (the one that calls `setMerkleRoot`)
2. Amount: $20-30 (daily) or $400-500 (hourly)
3. This wallet is the contract owner (can call `setMerkleRoot`)

**Environment Variable:**
```env
# Backend wallet private key (for Merkle root updates)
MERKLE_ROOT_UPDATE_WALLET_PRIVATE_KEY=0x...
```

### Step 3: Monitor & Auto-Fund (Optional)

```rust
// Check wallet balance before each update
async fn check_and_fund_wallet(evm: &EVMClient) -> Result<()> {
    let balance = evm.get_wallet_balance().await?;
    let min_balance = U256::from(10_000_000_000_000_000); // 0.01 ETH
    
    if balance < min_balance {
        tracing::warn!("Wallet balance low: {:?}, consider funding", balance);
        // Could trigger alert or auto-funding
    }
    
    Ok(())
}
```

## Recommended Setup

### For 2-3 Week Game Period:

**Update Frequency: Daily** (Recommended)
- ✅ Cost: ~$10-15 total
- ✅ Simple: One update per day
- ✅ Reasonable: 24-hour delay for new users
- ✅ Pre-fund: $20-30 before game starts

**Update Frequency: Every 6 Hours** (If faster needed)
- ✅ Cost: ~$28-42 total
- ✅ Faster: 6-hour delay for new users
- ✅ Pre-fund: $50-60 before game starts

**Update Frequency: Hourly** (If real-time needed)
- ✅ Cost: ~$250-350 total
- ✅ Fastest: 1-hour delay
- ✅ Pre-fund: $400-500 before game starts

## Summary

**Recommended Approach:**
1. ✅ **Daily updates** at 2 AM UTC
2. ✅ **Pre-fund $20-30** before game starts
3. ✅ **Monitor balance** (optional alerts)
4. ✅ **Total cost: ~$10-15** for entire 2-3 week period

**Benefits:**
- ✅ Extremely cheap ($10-15 vs $650k)
- ✅ Simple to implement
- ✅ Handles unlimited users
- ✅ Users can claim within 24 hours
- ✅ All existing logic stays the same

**When to Fund:**
- **Before game starts**: Fund backend wallet with $20-30
- **Monitor**: Check balance weekly
- **Refund**: After game ends, any unused ETH can be withdrawn

