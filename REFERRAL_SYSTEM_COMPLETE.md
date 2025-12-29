# Complete Referral System Implementation

## Overview

The referral system has been fully implemented across backend, frontend, and contract. When users refer friends, both the referrer and referee receive rewards that are deducted from the marketing allocation and tracked in the database for leaderboard display.

## Implementation Summary

### ✅ Database (PostgreSQL)

**Migration**: `20251229080532_add_referral_fields_to_holder`

**New Fields**:
- `referrer_telegram_id` - Who referred this user (immutable once set)
- `referral_rewards_earned` - Total referral rewards earned (in points)
- `total_referred` - Number of users referred
- `referral_rewards_claimed` - Referral rewards that have been claimed

**Indexes**: Added for efficient lookups by referrer and rewards

### ✅ Backend (Rust + Axum)

**Repository Methods** (`holder_repository.rs`):
- `set_referrer()` - Sets referrer relationship (immutable)
- `add_referral_reward()` - Adds reward to referrer's points (updates leaderboard)
- `get_holder_by_wallet_address()` - Lookup by wallet

**Service Methods** (`holder_service.rs`):
- Wrappers for all repository methods with error handling

**Claim Handler** (`telegram_handler.rs`):
- Validates referrer if provided
- Prevents self-referral
- Calculates reward (1:1 ratio with referee's points)
- Adds reward to referrer's `total_coins` and `current_coins` (leaderboard)
- Sets referrer relationship

**DTOs Updated**:
- `ClaimRequest` - Added optional `referrer_telegram_id`
- `HolderDto` - Added referral fields

### ✅ Contract (Solidity)

**Key Changes**:
1. **Referral Rewards Source**: Changed from contract balance to `marketingWallet`
2. **Immediate Payment**: Both referrer and referee receive rewards immediately
3. **Marketing Deduction**: Each referral deducts `TOKENS_PER_CLAIM * 2` from marketing wallet
4. **New Tracking**: `totalReferralRewardsPaidFromMarketing` tracks total paid

**Updated Functions**:
- `_processClaim()` - Pays rewards immediately from marketing wallet
- `_claimOwedAmount()` - Same for owed amount claims
- `contributeWithReferrer()` - Pays rewards from marketing wallet
- `claimReferralRewards()` - Updated to pay from marketing wallet (backward compatibility)
- `setOwedAmount()` / `batchSetOwedAmounts()` - Removed referral from supply validation

**Supply Validation**:
- Referral rewards no longer count against CLAIMABLE_SUPPLY
- Validation: `totalClaimed + totalContributed + totalOwedAmounts <= CLAIMABLE_SUPPLY`
- Referral rewards come from separate marketing allocation (50 Trillion tokens)

### ✅ Frontend (Next.js + TypeScript)

**Type Updates**:
- `ClaimRequest` - Added optional `referrerTelegramId`
- `HolderDto` - Added referral stats fields

**Claim Page**:
- Reads `tgref` URL parameter for referrer
- Passes referrer to backend in claim request
- Displays referral stats (when available in HolderDto)

**Leaderboard**:
- Automatically includes referral rewards (via `total_coins`)
- No additional changes needed

## Reward Flow

### Example Scenario

1. **User A** (has 1000 points) shares referral link: `/claim?tgref=123456789`
2. **User B** (has 500 points) clicks link and claims
3. **Backend Processing**:
   - Sets User B's `referrer_telegram_id` = User A's Telegram ID
   - Adds 500 points to User A's `referral_rewards_earned`
   - Adds 500 points to User A's `total_coins` and `current_coins`
   - Increments User A's `total_referred` count
4. **Contract Processing** (if using contract claims):
   - Transfers 4 Billion tokens to User B from contract balance
   - Transfers 4 Billion tokens to User A from marketing wallet
   - Transfers 4 Billion tokens to User B (bonus) from marketing wallet
   - Deducts 8 Billion total from marketing wallet
5. **Result**:
   - User A: Leaderboard shows 1500 points (1000 + 500 referral)
   - User B: Gets their 500 points + bonus from marketing
   - Marketing wallet: Deducted 8 Billion tokens

## Database Schema

```sql
CREATE TABLE holder (
    -- ... existing fields ...
    referrer_telegram_id VARCHAR(255),
    referral_rewards_earned BIGINT NOT NULL DEFAULT 0,
    total_referred INTEGER NOT NULL DEFAULT 0,
    referral_rewards_claimed BIGINT NOT NULL DEFAULT 0,
    -- ... timestamps ...
);
```

## API Endpoints

### POST `/api/v2/claim`

**Request**:
```json
{
  "telegramId": 123456789,
  "walletAddress": "0x...",
  "message": "Claim 500 points for Telegram ID: 123456789",
  "signature": "0x...",
  "referrerTelegramId": 987654321  // Optional
}
```

**Response**:
```json
{
  "success": true,
  "pointsClaimed": 500,
  "wallet": "0x..."
}
```

## Frontend Usage

### Share Referral Link

```typescript
const referralLink = `https://yourdomain.com/claim?tgref=${myTelegramId}`;
```

### Display Referral Stats

```tsx
<div>
  <p>Total Referred: {holder.totalReferred}</p>
  <p>Referral Rewards Earned: {holder.referralRewardsEarned} points</p>
  <p>Referral Rewards Claimed: {holder.referralRewardsClaimed} points</p>
</div>
```

## Contract Functions

### For Users

- `claimWithReferrer(paymentToken, referrer)` - Claim with referrer
- `claimReferralRewards()` - Claim old pending rewards (backward compatibility)

### For Owner

- `setReferralProgram(active, rewardPerClaim)` - Enable/disable referral program
- Marketing wallet balance can be checked via `balanceOf(marketingWallet)`
- Total paid tracked in `totalReferralRewardsPaidFromMarketing`

## Security Features

1. **Immutable Referrer**: Once set, cannot be changed
2. **Self-Referral Prevention**: Users cannot refer themselves
3. **Referrer Validation**: Backend validates referrer exists
4. **Marketing Balance Check**: Contract verifies sufficient marketing balance
5. **Atomic Database Updates**: Prevents race conditions
6. **Supply Separation**: Referral rewards don't affect claimable supply limits

## Testing Checklist

- [ ] User can claim with referrer
- [ ] Referrer receives reward in database
- [ ] Referrer's leaderboard points increase
- [ ] Contract pays rewards from marketing wallet
- [ ] Marketing wallet balance decreases correctly
- [ ] Self-referral is prevented
- [ ] Invalid referrer is rejected
- [ ] Referral stats display correctly
- [ ] Leaderboard includes referral rewards

## Migration Steps

1. **Run Database Migration**:
   ```bash
   cd backend
   sqlx migrate run
   ```

2. **Deploy Updated Contract**:
   - Compile and deploy the updated `NerdToken.sol`
   - Ensure marketing wallet has sufficient balance (50 Trillion tokens)

3. **Update Environment Variables**:
   - No new env vars needed (uses existing `TELEGRAM_BOT_TOKEN`)

4. **Deploy Backend**:
   - Backend changes are backward compatible
   - Existing claims without referrers still work

5. **Deploy Frontend**:
   - Frontend changes are backward compatible
   - Referral links work immediately

## Notes

- **Marketing Allocation**: 50 Trillion tokens (10% of total supply)
- **Reward Ratio**: 1:1 (referrer gets same as referee's points)
- **Contract Rewards**: 4 Billion tokens per referral (TOKENS_PER_CLAIM)
- **Database Rewards**: Points equivalent (1 point = 1 point in leaderboard)
- **Both Tracked**: Contract tracks token rewards, database tracks point rewards for leaderboard

## Files Modified

### Backend
- `migrations/20251229080532_add_referral_fields_to_holder.up.sql`
- `migrations/20251229080532_add_referral_fields_to_holder.down.sql`
- `backend/libraries/types/src/models/holder.rs`
- `backend/libraries/types/src/dto/holder_dto.rs`
- `backend/database/src/repository/holder_repository.rs`
- `backend/database/src/service/holder_service.rs`
- `backend/server/src/handler/telegram_handler.rs`

### Frontend
- `frontend/types/telegram.ts`
- `frontend/lib/api/holders.ts`
- `frontend/app/claim/page.tsx`

### Contract
- `frontend/contracts/NerdToken.sol`

## Documentation

- `REFERRAL_SYSTEM_IMPLEMENTATION.md` - Detailed implementation guide
- `CONTRACT_REFERRAL_UPDATES.md` - Contract-specific changes

