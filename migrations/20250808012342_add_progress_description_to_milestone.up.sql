-- Add progress_description field to milestone table for tracking milestone progress updates

ALTER TABLE milestone ADD COLUMN IF NOT EXISTS progress_description TEXT;
