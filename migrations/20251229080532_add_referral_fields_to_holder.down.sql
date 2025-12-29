-- Remove referral tracking fields from holder table
DROP INDEX IF EXISTS idx_holder_referral_rewards_earned;
DROP INDEX IF EXISTS idx_holder_referrer_telegram_id;

ALTER TABLE holder
DROP COLUMN IF EXISTS referral_rewards_claimed,
DROP COLUMN IF EXISTS total_referred,
DROP COLUMN IF EXISTS referral_rewards_earned,
DROP COLUMN IF EXISTS referrer_telegram_id;
