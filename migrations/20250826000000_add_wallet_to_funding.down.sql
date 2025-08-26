-- Remove wallet field from funding table and restore user_id as NOT NULL

ALTER TABLE funding DROP COLUMN IF EXISTS wallet;
ALTER TABLE funding ALTER COLUMN user_id SET NOT NULL;
