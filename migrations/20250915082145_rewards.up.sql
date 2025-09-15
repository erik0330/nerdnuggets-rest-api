-- Add up migration script here

CREATE TABLE IF NOT EXISTS rewards (
    id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    user_id UUID,
    wallet_address VARCHAR(255) NOT NULL,
    reward_type VARCHAR(50) NOT NULL,
    points INTEGER DEFAULT 0,
    amounts INTEGER DEFAULT 0,
    claimed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    CONSTRAINT fk_rewards_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_rewards_wallet_address ON rewards(wallet_address);
CREATE INDEX IF NOT EXISTS idx_rewards_user_id ON rewards(user_id);
CREATE INDEX IF NOT EXISTS idx_rewards_type ON rewards(reward_type);
CREATE INDEX IF NOT EXISTS idx_rewards_claimed ON rewards(claimed_at);
