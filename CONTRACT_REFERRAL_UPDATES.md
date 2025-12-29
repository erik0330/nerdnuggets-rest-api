# Contract Referral System Updates

## Summary

Updated the contract to deduct referral rewards from the marketing allocation instead of the public claimable supply. Both referrer and referee now receive rewards immediately when a claim is made.

## Key Changes

### 1. Referral Reward Payment Source

**Before**: Referral rewards were stored for referrer to claim later from contract balance (public allocation)

**After**: Referral rewards are paid immediately from `marketingWallet` balance to both referrer and referee

### 2. New Tracking Variable

Added `totalReferralRewardsPaidFromMarketing` to track total referral rewards paid from marketing allocation.

### 3. Updated Functions

#### `_processClaim()`
- Now transfers referral rewards immediately from `marketingWallet` to both referrer and referee
- Both parties receive the same reward amount (TOKENS_PER_CLAIM = 4 Billion tokens)
- Updates `totalReferralRewardsPaidFromMarketing` and `totalReferralRewardsClaimed`
- Emits `ReferralRewardPaid` event instead of `ReferralRewardOwed`

#### `_claimOwedAmount()`
- Same updates as `_processClaim()` for owed amount claims

#### `contributeWithReferrer()`
- Updated to pay referral rewards immediately from marketing wallet
- Both referrer and referee receive rewards

#### `claimReferralRewards()`
- Updated to pay from marketing wallet (for backward compatibility with old pending rewards)
- Still works for any old rewards that were stored before the update

#### `setOwedAmount()` and `batchSetOwedAmounts()`
- Removed referral rewards from CLAIMABLE_SUPPLY validation
- Referral rewards no longer count against claimable supply since they come from marketing allocation

### 4. Supply Validation Updates

All supply validation checks now exclude referral rewards from CLAIMABLE_SUPPLY calculations:
- Referral rewards come from marketing allocation (50 Trillion tokens)
- They don't count against the 350 Trillion claimable supply
- Validation: `totalClaimed + totalContributed + totalOwedAmounts <= CLAIMABLE_SUPPLY`

## Reward Flow

1. **User Claims with Referrer**:
   - User gets their regular claim tokens from contract balance
   - Referrer gets reward from `marketingWallet` (immediately)
   - Referee (user) gets bonus reward from `marketingWallet` (immediately)

2. **Both Rewarded**:
   - Referrer: Receives TOKENS_PER_CLAIM (4 Billion) from marketing wallet
   - Referee: Receives TOKENS_PER_CLAIM (4 Billion) from marketing wallet (bonus on top of their claim)

3. **Marketing Allocation Deduction**:
   - Each referral deducts `TOKENS_PER_CLAIM * 2` from marketing wallet
   - Tracked in `totalReferralRewardsPaidFromMarketing`

## Example

1. User A refers User B
2. User B claims 4 Billion tokens (from contract balance)
3. **Result**:
   - User B: Gets 4 Billion from contract + 4 Billion bonus from marketing = 8 Billion total
   - User A: Gets 4 Billion from marketing wallet
   - Marketing wallet: Deducts 8 Billion (4 for referrer + 4 for referee)

## Security Considerations

1. **Marketing Wallet Balance Check**: Contract verifies `marketingWallet` has sufficient balance before paying rewards
2. **Immediate Payment**: Rewards are paid immediately, no deferred claims
3. **Supply Separation**: Referral rewards don't affect claimable supply limits
4. **Immutable Referrer**: Once set, referrer relationship cannot be changed

## Events

- `ReferralRewardPaid(referrer, referee, rewardAmount, claimsCount)` - Emitted when referral rewards are paid
- `ReferralRewardClaimed(referrer, amount)` - Emitted when old pending rewards are claimed

## Migration Notes

- Old pending rewards in `referralRewardsToClaim` can still be claimed via `claimReferralRewards()`
- New referrals pay immediately from marketing wallet
- `totalReferralRewardsOwed` may still have old pending rewards but new referrals don't add to it

## Testing

To test the updated referral system:

1. Ensure `marketingWallet` has sufficient balance (50 Trillion tokens)
2. Enable referral program: `setReferralProgram(true, 0)`
3. User A claims with referrer B
4. Verify:
   - User A receives claim tokens from contract
   - User A receives bonus from marketing wallet
   - User B receives reward from marketing wallet
   - Marketing wallet balance decreased by (reward * 2)
   - `totalReferralRewardsPaidFromMarketing` increased

