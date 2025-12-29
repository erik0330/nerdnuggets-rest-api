# Backend Changes Implemented for Points-to-Tokens Integration

## Summary

This document outlines the backend changes implemented to integrate the NERD token contract with the Telegram points claiming system. Users can now claim tokens based on their Telegram game points.

## Changes Made

### 1. Contract ABI File
**File**: `abis/nerd_token_contract_abi.json`
- Created ABI file for the NERD token contract
- Contains the `setOwedAmount` function signature required for setting owed token amounts

### 2. EVM Client Updates
**File**: `backend/libraries/evm/src/lib.rs`

**Changes**:
- Added `abigen!(NERD_TOKEN_CONTRACT, "./abis/nerd_token_contract_abi.json");` to generate contract bindings
- Added `nerd_token_contract` field to `EVMClient` struct
- Updated `EVMClient::init()` to accept `nerd_token_contract_address` parameter
- Added `set_owed_amount()` method to interact with the contract's `setOwedAmount` function

**New Method**:
```rust
pub async fn set_owed_amount(
    &self,
    user_address: &str,
    tokens_wei: U256,
) -> Result<String, anyhow::Error>
```

### 3. Environment Variables
**File**: `backend/libraries/utils/src/env.rs`

**New Fields Added**:
- `nerd_token_contract_address: String` - Contract address for NERD token
- `points_to_tokens_conversion_rate: u64` - Conversion rate from points to tokens (in wei)

**Environment Variables Required**:
- `NERD_TOKEN_CONTRACT_ADDRESS` - Must be set
- `POINTS_TO_TOKENS_CONVERSION_RATE` - Optional (defaults to 1e18 = 1 token per point)

### 4. State Initialization
**File**: `backend/server/src/state.rs`

**Changes**:
- Updated `EVMClient::init()` call to pass `nerd_token_contract_address` parameter

### 5. Telegram Claim Handler
**File**: `backend/server/src/handler/telegram_handler.rs`

**Changes**:
- Added `calculate_tokens_from_points()` helper function
- Updated `claim()` handler to:
  1. Calculate tokens from points using conversion rate
  2. Call `set_owed_amount()` on the contract
  3. Handle errors appropriately
  4. Mark user as claimed in database after successful contract call

**New Function**:
```rust
fn calculate_tokens_from_points(points: u64, conversion_rate: u64) -> U256
```

**Flow**:
1. Verify signature and message
2. Calculate `tokens_wei = points * conversion_rate`
3. Call contract `setOwedAmount(user_address, tokens_wei)`
4. On success, mark user as claimed in database
5. Return success response

## Configuration

### Required Environment Variables

Add these to your `.env` file:

```env
# NERD Token Contract Address (Base network)
NERD_TOKEN_CONTRACT_ADDRESS=0x...

# Points to Tokens Conversion Rate (in wei, 18 decimals)
# Default: 1000000000000000000 (1 token per point)
# Example: For 1000 tokens per point, use: 1000000000000000000000
POINTS_TO_TOKENS_CONVERSION_RATE=1000000000000000000
```

### Conversion Rate Explanation

The `POINTS_TO_TOKENS_CONVERSION_RATE` is in wei (18 decimals). 

- `1e18` = 1 token per point
- `1e21` = 1000 tokens per point
- `1e20` = 100 tokens per point

**Formula**: `tokens_wei = points * conversion_rate`

**Example**:
- User has 1000 points
- Conversion rate = `1e18` (1 token per point)
- `tokens_wei = 1000 * 1e18 = 1000 tokens`

## Integration Flow

1. **User connects Telegram** → Frontend calls `/api/v2/verify-telegram`
2. **Backend verifies** → Returns user's points
3. **User connects wallet** → Signs message with Telegram ID
4. **Frontend calls `/api/v2/claim`** → Sends signature and wallet address
5. **Backend**:
   - Verifies signature
   - Calculates tokens from points
   - Calls contract `setOwedAmount(wallet, tokens_wei)`
   - Marks user as claimed in database
   - Returns success
6. **User claims tokens** → Frontend calls contract `claim()` function to receive tokens

## Error Handling

The implementation includes error handling for:
- Invalid wallet addresses
- Contract transaction failures
- Database update failures
- Signature verification failures

If the contract call fails, the user is not marked as claimed in the database, allowing them to retry.

## Notes

- The contract's `setOwedAmount` function requires the caller to be the contract owner
- The backend wallet (configured via `WALLET_PRIVATE_KEY`) must be the contract owner
- Users can only claim their owed amount once (enforced by contract)
- The conversion rate is configurable via environment variable for flexibility

## Next Steps

1. Set the `NERD_TOKEN_CONTRACT_ADDRESS` environment variable to your deployed contract address
2. Configure `POINTS_TO_TOKENS_CONVERSION_RATE` based on your tokenomics
3. Ensure the backend wallet (`WALLET_PRIVATE_KEY`) is the contract owner
4. Test the integration end-to-end

