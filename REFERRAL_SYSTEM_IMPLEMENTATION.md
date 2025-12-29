# Referral System Implementation

## Overview

This document describes the referral system implementation where users can refer friends and both the referrer and referee receive rewards. Referral rewards are tracked in the database, displayed on user pages, and included in leaderboard rankings.

## Key Features

1. **Referral Tracking**: When someone claims with a referrer, both parties get rewarded
2. **Reward Calculation**: Referrer receives the same amount as the referee's points (1:1 ratio)
3. **Leaderboard Integration**: Referral rewards are added to `total_coins` and `current_coins`, automatically appearing in leaderboard rankings
4. **Database Tracking**: All referral relationships and rewards are stored in the database
5. **Frontend Display**: Referral stats are shown on user pages

## Database Schema

### New Fields Added to `holder` Table

```sql
referrer_telegram_id VARCHAR(255)          -- Telegram ID of the referrer (immutable once set)
referral_rewards_earned BIGINT DEFAULT 0   -- Total referral rewards earned (in points)
total_referred INTEGER DEFAULT 0           -- Number of users referred
referral_rewards_claimed BIGINT DEFAULT 0   -- Referral rewards that have been claimed
```

### Migration Files

- `migrations/20251229080532_add_referral_fields_to_holder.up.sql` - Adds referral fields
- `migrations/20251229080532_add_referral_fields_to_holder.down.sql` - Removes referral fields

## Backend Implementation

### Repository Methods (`holder_repository.rs`)

1. **`set_referrer(telegram_id, referrer_telegram_id)`**
   - Sets the referrer relationship (only if not already set - immutable)
   - Returns `true` if referrer was set, `false` if already had a referrer

2. **`add_referral_reward(referrer_telegram_id, reward_amount)`**
   - Adds reward to referrer's `referral_rewards_earned`
   - Increments `total_referred` count
   - Adds reward to both `total_coins` and `current_coins` (for leaderboard)
   - Updates `updated_at` timestamp

3. **`get_holder_by_wallet_address(wallet_address)`**
   - Retrieves holder by wallet address (for contract-based referral tracking)

### Service Methods (`holder_service.rs`)

- `set_referrer()` - Wrapper for repository method
- `add_referral_reward()` - Wrapper for repository method
- `get_holder_by_wallet_address()` - Wrapper for repository method

### Claim Handler Updates (`telegram_handler.rs`)

The `claim()` handler now:

1. **Validates Referrer** (if provided):
   - Checks referrer exists
   - Prevents self-referral
   - Sets referrer relationship (immutable once set)

2. **Calculates Referral Reward**:
   - Reward = points claimed by referee (1:1 ratio)
   - Both referrer and referee get the same reward amount

3. **Adds Reward to Referrer**:
   - Updates referrer's `referral_rewards_earned`
   - Increments `total_referred`
   - Adds reward to `total_coins` and `current_coins` (appears in leaderboard)

4. **Processes Claim**:
   - Marks holder as claimed
   - Sets owed amount in contract
   - Returns success response

### API Changes

**Claim Request** (`ClaimRequest`):
```rust
pub struct ClaimRequest {
    pub telegram_id: u64,
    pub wallet_address: String,
    pub message: String,
    pub signature: String,
    pub referrer_telegram_id: Option<u64>,  // NEW: Optional referrer
}
```

**Holder DTO** (`HolderDto`):
```rust
pub struct HolderDto {
    // ... existing fields ...
    pub referrer_telegram_id: Option<String>,
    pub referral_rewards_earned: i64,
    pub total_referred: i32,
    pub referral_rewards_claimed: i64,
}
```

## Frontend Implementation

### Type Updates

**`types/telegram.ts`**:
```typescript
export interface ClaimRequest {
  telegramId: number;
  walletAddress: string;
  message: string;
  signature: string;
  referrerTelegramId?: number;  // NEW: Optional referrer
}
```

**`lib/api/holders.ts`**:
```typescript
export interface HolderDto {
  // ... existing fields ...
  referrerTelegramId?: string;
  referralRewardsEarned: number;
  totalReferred: number;
  referralRewardsClaimed: number;
}
```

### Claim Page Updates

1. **Referrer Detection**: Reads `tgref` parameter from URL
   - Example: `/claim?tgref=123456789`
   - Sets `referrerTelegramId` state

2. **Claim Request**: Includes `referrerTelegramId` in claim request if provided

### Display Referral Stats

Referral stats are available in `HolderDto` and can be displayed on:
- User profile pages
- Leaderboard entries
- Referral dashboard sections

## Reward Calculation

### Current Implementation

- **Referrer Reward**: Equal to referee's points claimed (1:1 ratio)
- **Referee Reward**: Their original points (unchanged)
- **Both rewards added to leaderboard**: Yes, via `total_coins` and `current_coins`

### Example

1. User A has 1000 points and refers User B
2. User B has 500 points and claims with User A as referrer
3. **Result**:
   - User B: Claims 500 points (their original points)
   - User A: Receives 500 referral reward points
   - User A's leaderboard: `total_coins` = 1000 + 500 = 1500
   - User A's stats: `referral_rewards_earned` = 500, `total_referred` = 1

## Marketing Allocation

**Note**: Currently, referral rewards are added to the holder's points and tracked separately. The contract's marketing allocation (50 Trillion tokens) is separate from the referral reward tracking in the database.

For contract-level tracking, referral rewards come from the contract's public allocation balance. The database tracks referral rewards separately for leaderboard and display purposes.

## Usage

### For Users

1. **Share Referral Link**: 
   - Format: `https://yourdomain.com/claim?tgref=<TELEGRAM_ID>`
   - Replace `<TELEGRAM_ID>` with your Telegram ID

2. **When Friend Claims**:
   - Friend verifies Telegram and claims points
   - Both you and your friend receive rewards
   - Rewards appear in your leaderboard ranking

### For Developers

1. **Check Referral Stats**:
   ```typescript
   const holder = await holdersApi.getHolders();
   const myHolder = holder.holders.find(h => h.telegramId === myTelegramId);
   console.log(`Referred: ${myHolder.totalReferred}, Earned: ${myHolder.referralRewardsEarned}`);
   ```

2. **Display Referral Stats**:
   ```tsx
   <div>
     <p>Total Referred: {holder.totalReferred}</p>
     <p>Referral Rewards Earned: {holder.referralRewardsEarned} points</p>
   </div>
   ```

## Security Considerations

1. **Immutable Referrer**: Once set, referrer cannot be changed (prevents gaming)
2. **Self-Referral Prevention**: Users cannot refer themselves
3. **Referrer Validation**: Backend validates referrer exists before processing
4. **Atomic Updates**: Database updates are atomic to prevent race conditions

## Future Enhancements

1. **Contract Integration**: Track referral rewards in contract and deduct from marketing pool
2. **Referral Dashboard**: Dedicated page showing referral stats and earnings
3. **Referral Leaderboard**: Separate leaderboard for top referrers
4. **Tiered Rewards**: Different reward amounts based on referral count

## Testing

To test the referral system:

1. Create two test holders in the database
2. Use holder A's Telegram ID as referrer for holder B
3. Claim points for holder B with referrer set
4. Verify:
   - Holder B's `referrer_telegram_id` is set to Holder A
   - Holder A's `referral_rewards_earned` increased
   - Holder A's `total_referred` incremented
   - Holder A's `total_coins` and `current_coins` increased
   - Both appear correctly in leaderboard

## Migration

Run the migration to add referral fields:

```bash
cd backend
sqlx migrate run
```

To rollback:

```bash
sqlx migrate revert
```

