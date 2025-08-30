-- Remove predict_result column from prediction table
ALTER TABLE prediction DROP COLUMN IF EXISTS predict_result;
