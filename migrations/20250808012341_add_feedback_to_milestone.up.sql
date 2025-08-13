-- Add feedback field to milestone table for admin feedback on milestone submissions

ALTER TABLE milestone ADD COLUMN IF NOT EXISTS feedback TEXT;
