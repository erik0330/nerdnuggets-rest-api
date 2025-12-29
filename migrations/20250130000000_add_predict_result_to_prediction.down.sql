DO $$
BEGIN
  IF EXISTS (
    SELECT 1
    FROM information_schema.tables
    WHERE table_schema = 'public' AND table_name = 'prediction'
  ) THEN
    ALTER TABLE prediction
      DROP COLUMN IF EXISTS predict_result;
  END IF;
END $$;
