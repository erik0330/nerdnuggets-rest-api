-- Remove apple_id field from users table
DROP INDEX IF EXISTS idx_users_apple_id;
ALTER TABLE users DROP COLUMN IF EXISTS apple_id; 