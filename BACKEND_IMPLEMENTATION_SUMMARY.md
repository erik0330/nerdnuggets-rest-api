# Backend Implementation Summary

## Quick Overview

The backend needs to be updated to calculate token amounts from points and set them in the smart contract. Here's what needs to be done:

## What's Missing

1. **NERD Token Contract Integration** - Add contract to EVM client
2. **Points-to-Tokens Conversion** - Calculate tokens from points  
3. **Contract Interaction** - Call `setOwedAmount()` when user claims
4. **Environment Variables** - Add contract address and conversion rate

## Implementation Checklist

### ✅ Step 1: Create NERD Token Contract ABI File

**Location:** The existing ABIs are in `abis/` at workspace root, but the code references `./abis/` which suggests they might be copied or the path is resolved differently.

**Options:**
1. **Create at:** `abis/nerd_token_contract_abi.json` (workspace root, alongside other ABIs)
2. **Or create at:** `backend/libraries/evm/abis/nerd_token_contract_abi.json` (to match `./abis/` pattern in code)

**How to get the ABI:**
1. Compile the Solidity contract (`frontend/contracts/NerdToken.sol`) 
2. Extract the ABI from the compiled output (look for the `setOwedAmount` function)
3. Or get it from the deployed contract on block explorer (Etherscan/Basescan)
4. Create a JSON file with at minimum the `setOwedAmount` function, or include the full contract ABI

**Minimum required function in ABI:**
```json
[
  {
    "inputs": [
      {"internalType": "address", "name": "_user", "type": "address"},
      {"internalType": "uint256", "name": "_amount", "type": "uint256"}
    ],
    "name": "setOwedAmount",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  }
]
```

### ✅ Step 2: Update EVM Client

**File:** `backend/libraries/evm/src/lib.rs`

**Changes:**
1. Add `abigen!(NERD_TOKEN_CONTRACT, "../abis/nerd_token_contract_abi.json");`
2. Add `nerd_token_contract` field to `EVMClient` struct
3. Add `nerd_token_contract_address` parameter to `init()` method
4. Initialize `nerd_token_contract` in `init()`
5. Add `set_owed_amount()` method to call the contract function

### ✅ Step 3: Update Environment Variables

**File:** `backend/libraries/utils/src/env.rs`

**Add:**
- `nerd_token_contract_address: String`
- `points_to_tokens_conversion_rate: u64` (e.g., 1000 = 1 point = 1000 tokens)

### ✅ Step 4: Update State Initialization

**File:** `backend/server/src/state.rs`

**Update:** Pass `nerd_token_contract_address` to `EVMClient::init()`

### ✅ Step 5: Update Claim Handler

**File:** `backend/server/src/handler/telegram_handler.rs`

**Add:**
1. Conversion function: `calculate_tokens_from_points(points, rate) -> U256`
2. In `claim()` function: Calculate tokens, call `evm.set_owed_amount()`, then mark as claimed

### ✅ Step 6: Add Environment Variables

**Add to `.env` file:**
```env
NERD_TOKEN_CONTRACT_ADDRESS=0x...  # Your deployed contract address
POINTS_TO_TOKENS_CONVERSION_RATE=1000  # Tokens per point (adjust as needed)
```

## Decision Needed

**What is the conversion rate between points and tokens?**
- Example: 1 point = 1000 tokens
- Example: 1 point = 1,000,000 tokens  
- This needs to be decided before implementation

## Files to Modify

1. `abis/nerd_token_contract_abi.json` (NEW FILE)
2. `backend/libraries/evm/src/lib.rs`
3. `backend/libraries/utils/src/env.rs`
4. `backend/server/src/state.rs`
5. `backend/server/src/handler/telegram_handler.rs`
6. `.env` (add new variables)

## Detailed Instructions

See `BACKEND_POINTS_TO_TOKENS_IMPLEMENTATION.md` for complete step-by-step instructions with code examples.

## Testing

After implementation:
1. Test conversion calculation with various point values
2. Test contract call succeeds and sets correct amount
3. Test user can then call `claim()` on contract
4. Test error handling (contract call failures, etc.)

