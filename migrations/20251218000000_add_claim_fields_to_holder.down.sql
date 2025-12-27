-- Remove indexes
DROP INDEX IF EXISTS idx_holder_claimed_at;
DROP INDEX IF EXISTS idx_holder_wallet_address;

-- Remove columns
ALTER TABLE holder
DROP COLUMN IF EXISTS claimed_at,
DROP COLUMN IF EXISTS wallet_address;

