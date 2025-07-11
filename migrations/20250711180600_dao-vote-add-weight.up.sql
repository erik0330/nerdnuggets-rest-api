-- Add up migration script here

ALTER TABLE dao_vote ADD COLUMN IF NOT EXISTS weight REAL;
