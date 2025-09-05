-- Add up migration script here

ALTER TABLE dao ADD COLUMN IF NOT EXISTS details TEXT;
