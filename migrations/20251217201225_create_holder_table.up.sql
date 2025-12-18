-- Create holder table for migrated MongoDB user data
CREATE TABLE holder (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    telegram_id VARCHAR(255) NOT NULL UNIQUE,
    username VARCHAR(100),
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    photo_url VARCHAR(500),
    total_coins BIGINT NOT NULL DEFAULT 0,
    current_coins BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for better query performance
CREATE INDEX idx_holder_telegram_id ON holder(telegram_id);
CREATE INDEX idx_holder_total_coins ON holder(total_coins DESC);
CREATE INDEX idx_holder_current_coins ON holder(current_coins DESC);
CREATE INDEX idx_holder_created_at ON holder(created_at);

