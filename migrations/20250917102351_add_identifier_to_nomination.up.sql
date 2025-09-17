-- Add identifier field to nomination table

ALTER TABLE nomination 
ADD COLUMN identifier VARCHAR(255);

-- Create index for better query performance
CREATE INDEX IF NOT EXISTS idx_nomination_identifier ON nomination(identifier);
