-- Add up migration script here

ALTER TABLE project ADD COLUMN IF NOT EXISTS proposal_id INT;
