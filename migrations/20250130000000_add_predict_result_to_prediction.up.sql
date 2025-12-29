DO $$
BEGIN
  IF EXISTS (
    SELECT 1
    FROM information_schema.tables
    WHERE table_schema = 'public' AND table_name = 'prediction'
  ) THEN
    ALTER TABLE prediction
      ADD COLUMN IF NOT EXISTS predict_result BOOLEAN;
  END IF;
END $$;
