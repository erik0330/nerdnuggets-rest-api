-- Remove feedback field from milestone table

ALTER TABLE milestone DROP COLUMN IF EXISTS feedback;
