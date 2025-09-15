-- Add down migration script here

DROP INDEX IF EXISTS idx_rewards_claimed;
DROP INDEX IF EXISTS idx_rewards_type;
DROP INDEX IF EXISTS idx_rewards_user_id;
DROP TABLE IF EXISTS rewards;
