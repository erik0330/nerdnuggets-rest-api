-- Add wallet field to funding table and make user_id nullable

ALTER TABLE funding ADD COLUMN IF NOT EXISTS wallet VARCHAR(255);
ALTER TABLE funding ALTER COLUMN user_id DROP NOT NULL;
