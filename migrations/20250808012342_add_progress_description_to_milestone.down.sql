-- Remove progress_description field from milestone table

ALTER TABLE milestone DROP COLUMN IF EXISTS progress_description;
