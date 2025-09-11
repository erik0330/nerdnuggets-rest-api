-- Add privy_id field to users table for Privy authentication
ALTER TABLE users ADD COLUMN IF NOT EXISTS privy_id VARCHAR(255);
CREATE INDEX IF NOT EXISTS idx_users_privy_id ON users(privy_id);
