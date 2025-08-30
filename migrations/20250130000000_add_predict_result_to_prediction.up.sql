-- Add predict_result column to prediction table
ALTER TABLE prediction ADD COLUMN IF NOT EXISTS predict_result BOOLEAN;
