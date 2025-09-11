-- Remove privy_id field from users table
DROP INDEX IF EXISTS idx_users_privy_id;
ALTER TABLE users DROP COLUMN IF EXISTS privy_id;
