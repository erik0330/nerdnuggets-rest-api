-- Add referral tracking fields to holder table
ALTER TABLE holder
ADD COLUMN IF NOT EXISTS referrer_telegram_id VARCHAR(255),
ADD COLUMN IF NOT EXISTS referral_rewards_earned BIGINT NOT NULL DEFAULT 0,
ADD COLUMN IF NOT EXISTS total_referred INTEGER NOT NULL DEFAULT 0,
ADD COLUMN IF NOT EXISTS referral_rewards_claimed BIGINT NOT NULL DEFAULT 0;

-- Create indexes for referral lookups
CREATE INDEX IF NOT EXISTS idx_holder_referrer_telegram_id ON holder(referrer_telegram_id);
CREATE INDEX IF NOT EXISTS idx_holder_referral_rewards_earned ON holder(referral_rewards_earned DESC);

-- Add foreign key constraint (optional, for referential integrity)
-- Note: This allows NULL for referrer_telegram_id (users without referrers)
-- ALTER TABLE holder
-- ADD CONSTRAINT fk_holder_referrer 
-- FOREIGN KEY (referrer_telegram_id) REFERENCES holder(telegram_id) ON DELETE SET NULL;
