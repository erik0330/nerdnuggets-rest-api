# Backend Implementation: Points-to-Tokens Claim System

## Overview

This document outlines what needs to be implemented in the backend to support the points-to-tokens claim system. Users earn points through Telegram activities, and these points determine how many tokens they can claim. The backend must calculate token amounts from points and set them in the smart contract.

## Current Status

The backend currently:
- ✅ Has Telegram verification endpoint (`/api/v2/verify-telegram`)
- ✅ Has claim endpoint (`/api/v2/claim`)
- ✅ Verifies Telegram authentication hash
- ✅ Verifies wallet signatures
- ✅ Marks holders as claimed in database
- ❌ **Missing**: Points-to-tokens conversion logic
- ❌ **Missing**: Contract interaction to set owed amounts
- ❌ **Missing**: NERD token contract integration in EVM client

## Required Changes

### 1. Add NERD Token Contract to EVM Client

**File:** `backend/libraries/evm/src/lib.rs`

**Changes Needed:**
1. Generate contract bindings for the NERD token contract
2. Add NERD token contract to `EVMClient` struct
3. Initialize NERD token contract in `init()` method
4. Add method to call `setOwedAmount()` on the contract

**Step-by-Step:**

#### Step 1.1: Create Contract ABI File

Create `abis/nerd_token_contract_abi.json` (in the root `abis/` folder, same location as other contract ABIs) with the ABI for the NERD token contract.

**Note:** You can extract the ABI from the compiled contract or get it from the deployed contract on a block explorer. The ABI should include at minimum:
- `function setOwedAmount(address _user, uint256 _amount) external onlyOwner`

**Location:** `/Users/bblist/dev/nerdnuggets/nerdnuggets-api/abis/nerd_token_contract_abi.json`

You can also compile the Solidity contract and extract just the needed function(s) in a minimal ABI array, or include the full contract ABI.

#### Step 1.2: Update EVM Client

Add to `backend/libraries/evm/src/lib.rs`:

```rust
// Add after existing abigen! macros
// NOTE: The existing code uses "./abis/" but abis are at workspace root.
// You may need to either:
// Option 1: Create backend/libraries/evm/abis/ and copy ABI file there (matches existing pattern)
// Option 2: Use path "../../abis/nerd_token_contract_abi.json" (relative to evm crate root)
// Check how the existing abigen! paths work first - they may be resolved at build time
abigen!(NERD_TOKEN_CONTRACT, "./abis/nerd_token_contract_abi.json"); // Try this first, if it doesn't work use ../../abis/

// Update EVMClient struct to include nerd_token_contract
#[derive(Clone)]
pub struct EVMClient {
    dao_contract: DAO_CONTRACT<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    funding_contract: FUNDING_CONTRACT<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    prediction_contract: PREDICTION_CONTRACT<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    nerd_token_contract: NERD_TOKEN_CONTRACT<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>, // ADD THIS
    provider: Provider<Http>,
}

// Update init() method to initialize NERD token contract
impl EVMClient {
    pub fn init(
        dao_contract_address: &str,
        funding_contract_address: &str,
        prediction_contract_address: &str,
        nerd_token_contract_address: &str, // ADD THIS PARAMETER
        wallet_private_key: &str,
        rpc_url: &str,
        chain_id: u64,
    ) -> EVMClient {
        // ... existing code ...
        
        let nerd_token_contract_address = nerd_token_contract_address
            .parse::<Address>()
            .expect("Invalid NERD_TOKEN_CONTRACT_ADDRESS");
        
        // ... existing code for other contracts ...
        
        let nerd_token_contract = NERD_TOKEN_CONTRACT::new(
            nerd_token_contract_address,
            Arc::new(provider_with_wallet.clone()),
        );

        EVMClient {
            dao_contract,
            funding_contract,
            prediction_contract,
            nerd_token_contract, // ADD THIS
            provider,
        }
    }

    // ADD THIS NEW METHOD
    /// Set owed amount for a user in the NERD token contract
    /// @param user_address: Wallet address of the user
    /// @param amount_wei: Amount of tokens in wei (18 decimals, e.g., 1000 tokens = 1000 * 10^18)
    pub async fn set_owed_amount(
        &self,
        user_address: &str,
        amount_wei: U256,
    ) -> Result<String, anyhow::Error> {
        let user_address = user_address.parse::<Address>()?;
        
        if let Some(tx) = self
            .nerd_token_contract
            .set_owed_amount(user_address, amount_wei)
            .send()
            .await?
            .await?
        {
            Ok(format!("{:?}", tx.transaction_hash))
        } else {
            Err(anyhow!("Transaction failed"))
        }
    }
}
```

#### Step 1.3: Update Environment Variables

**File:** `backend/libraries/utils/src/env.rs`

Add to `Env` struct:
```rust
pub nerd_token_contract_address: String,
```

Add to `init()` method:
```rust
let nerd_token_contract_address = std::env::var("NERD_TOKEN_CONTRACT_ADDRESS")
    .expect("NERD_TOKEN_CONTRACT_ADDRESS must be set");
```

Add to return statement in `init()`:
```rust
nerd_token_contract_address, // ADD THIS
```

#### Step 1.4: Update AppState

**File:** `backend/server/src/state.rs`

Update `AppState::init()` to pass the NERD token contract address:
```rust
let evm = EVMClient::init(
    &env.dao_contract_address,
    &env.funding_contract_address,
    &env.prediction_contract_address,
    &env.nerd_token_contract_address, // ADD THIS
    &env.wallet_private_key,
    &env.rpc_url,
    env.chain_id,
);
```

### 2. Implement Points-to-Tokens Conversion

**Decision Required:** You need to determine the conversion rate between points and tokens.

**Options:**
1. **Fixed rate**: `tokens = points * CONVERSION_RATE` (e.g., 1 point = 1000 tokens)
2. **Scaled rate**: Different rates based on point ranges
3. **Formula-based**: Complex calculation based on leaderboard position or other factors

**Recommendation:** Start with a simple fixed rate, stored as an environment variable for easy adjustment.

#### Step 2.1: Add Conversion Rate to Environment

**File:** `backend/libraries/utils/src/env.rs`

Add to `Env` struct:
```rust
pub points_to_tokens_conversion_rate: u64, // Tokens per point (e.g., 1000 means 1 point = 1000 tokens)
```

Add to `init()` method:
```rust
let points_to_tokens_conversion_rate = std::env::var("POINTS_TO_TOKENS_CONVERSION_RATE")
    .ok()
    .and_then(|p| p.parse().ok())
    .unwrap_or(1000); // Default: 1 point = 1000 tokens
```

#### Step 2.2: Create Conversion Function

**File:** `backend/server/src/handler/telegram_handler.rs` (or create a new utility module)

Add conversion function:
```rust
/// Convert points to tokens (in wei, 18 decimals)
/// @param points: Number of points
/// @param conversion_rate: Tokens per point
/// @return: Tokens in wei (18 decimals)
fn calculate_tokens_from_points(points: u64, conversion_rate: u64) -> U256 {
    // tokens = points * conversion_rate
    // Convert to wei (18 decimals): multiply by 10^18
    let tokens = U256::from(points) * U256::from(conversion_rate);
    tokens * U256::from(10).pow(U256::from(18))
}
```

### 3. Update Claim Handler

**File:** `backend/server/src/handler/telegram_handler.rs`

**Update the `claim` function to:**

1. Calculate tokens from points
2. Call `setOwedAmount` on the contract
3. Then mark as claimed in database

**Modified `claim` function:**

```rust
pub async fn claim(
    State(state): State<AppState>,
    Json(payload): Json<ClaimRequest>,
) -> Result<Json<ClaimResponse>, ApiError> {
    // ... existing validation code (lines 149-262) ...

    // After signature verification and before marking as claimed:

    // Calculate tokens from points
    let conversion_rate = state.env.points_to_tokens_conversion_rate;
    let tokens_wei = calculate_tokens_from_points(holder.current_coins as u64, conversion_rate);

    // Set owed amount in contract BEFORE marking as claimed in DB
    // This ensures atomicity: if contract call fails, DB isn't updated
    match state.evm.set_owed_amount(&payload.wallet_address, tokens_wei).await {
        Ok(tx_hash) => {
            // Log transaction hash for reference
            println!("Set owed amount transaction: {}", tx_hash);
        }
        Err(e) => {
            // Contract call failed - don't mark as claimed
            return Ok(Json(ClaimResponse {
                success: false,
                points_claimed: None,
                wallet: None,
                error: Some(format!("Failed to set owed amount in contract: {}", e)),
            }));
        }
    }

    // Now mark as claimed in DB (existing code)
    let updated = state
        .service
        .holder
        .update_holder_claim(&telegram_id_str, &payload.wallet_address)
        .await?;

    // ... rest of existing code ...
}
```

### 4. Environment Variables Required

Add these to your `.env` file:

```env
# NERD Token Contract Address
NERD_TOKEN_CONTRACT_ADDRESS=0x...  # Your deployed NERD token contract address

# Points to Tokens Conversion Rate
# Example: 1000 means 1 point = 1000 tokens
POINTS_TO_TOKENS_CONVERSION_RATE=1000
```

### 5. Database Migration (If Needed)

**Check if `holder` table has `wallet_address` and `claimed_at` columns:**

If not, create migration:

```sql
-- Add wallet_address column if not exists
ALTER TABLE holder 
ADD COLUMN IF NOT EXISTS wallet_address VARCHAR(255);

-- Add claimed_at column if not exists  
ALTER TABLE holder
ADD COLUMN IF NOT EXISTS claimed_at TIMESTAMPTZ;

-- Add index for wallet_address
CREATE INDEX IF NOT EXISTS idx_holder_wallet_address ON holder(wallet_address);

-- Add index for claimed_at
CREATE INDEX IF NOT EXISTS idx_holder_claimed_at ON holder(claimed_at);
```

Based on the code review, these columns likely already exist, but verify.

## Implementation Order

1. ✅ **Add NERD token contract ABI** to `backend/libraries/evm/abis/`
2. ✅ **Update EVM client** to include NERD token contract
3. ✅ **Add environment variables** for contract address and conversion rate
4. ✅ **Implement conversion function** (points → tokens)
5. ✅ **Update claim handler** to calculate and set owed amounts
6. ✅ **Test thoroughly** before deploying

## Testing

After implementation, test:

1. **Unit Tests:**
   - Test conversion function with various point values
   - Test edge cases (0 points, very large points)

2. **Integration Tests:**
   - Test full claim flow: verify Telegram → claim → check contract
   - Test contract call failures (what happens if contract call fails?)
   - Test duplicate claims (should be prevented)

3. **Manual Testing:**
   - Use test wallet to claim tokens
   - Verify `owedAmounts[wallet]` in contract matches expected value
   - Verify user can then call `claim()` on contract to receive tokens

## Security Considerations

1. **Atomicity**: Set owed amount in contract BEFORE marking as claimed in DB
   - If contract call fails, user can retry
   - If DB update fails, contract state can be corrected manually

2. **Idempotency**: Check if user already has owed amount set before setting
   - Contract's `setOwedAmount` already checks `!hasClaimedOwedAmount[user]`
   - But you may want to check contract state before calling

3. **Conversion Rate**: Store conversion rate securely (env var, not hardcoded)
   - Allows adjustment without code changes
   - Consider making it configurable per user/range if needed

4. **Error Handling**: Handle contract call failures gracefully
   - Don't mark as claimed if contract call fails
   - Return meaningful error messages
   - Log transaction hashes for debugging

## Contract Function Reference

From `contracts/NerdToken.sol`:

```solidity
function setOwedAmount(address _user, uint256 _amount) external onlyOwner {
    require(_user != address(0), "Cannot set owed amount for zero address");
    require(!hasClaimedOwedAmount[_user], "User has already claimed their owed amount");
    
    // Update total owed amounts
    uint256 oldAmount = owedAmounts[_user];
    totalOwedAmounts = totalOwedAmounts - oldAmount + _amount;
    
    // Validate supply limit
    require(totalOwedAmounts + totalClaimed + totalContributed + totalReferralRewardsOwed + totalReferralRewardsClaimed <= CLAIMABLE_SUPPLY, "Would exceed claimable supply");
    
    owedAmounts[_user] = _amount;
    emit OwedAmountSet(_user, _amount);
}
```

**Important Notes:**
- `_amount` must be in wei (18 decimals)
- Example: 1000 tokens = `1000 * 10^18` = `1000000000000000000000`
- Contract validates that total doesn't exceed `CLAIMABLE_SUPPLY`
- Only contract owner can call this (backend wallet must be owner)

## Questions to Answer Before Implementation

1. **What is the conversion rate?** (e.g., 1 point = ? tokens)
2. **Is the conversion rate fixed or variable?**
3. **What should happen if contract call succeeds but DB update fails?**
4. **Should we check contract state before setting (idempotency)?**
5. **How to handle partial failures?** (contract succeeds, DB fails, or vice versa)

## Next Steps

1. Review and approve this implementation plan
2. Determine conversion rate
3. Get NERD token contract ABI (extract from compiled contract or get from deployed contract)
4. Implement changes in order listed above
5. Test thoroughly
6. Deploy to staging/testnet
7. Verify end-to-end flow works
8. Deploy to production

## Support

For questions or issues:
- Review `POINTS_TO_TOKENS_LOGIC.md` in frontend folder for system overview
- Check contract code in `contracts/NerdToken.sol`
- Review existing EVM client code for patterns

