-- Add wallet_address and claimed_at fields to holder table for token claiming
ALTER TABLE holder
ADD COLUMN IF NOT EXISTS wallet_address VARCHAR(255),
ADD COLUMN IF NOT EXISTS claimed_at TIMESTAMPTZ;

-- Create index for wallet_address lookups
CREATE INDEX IF NOT EXISTS idx_holder_wallet_address ON holder(wallet_address);

-- Create index for claimed_at to filter unclaimed holders
CREATE INDEX IF NOT EXISTS idx_holder_claimed_at ON holder(claimed_at);

