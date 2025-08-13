-- Update bounty category field from UUID to UUID array

-- First, add a new column for the array
ALTER TABLE bounty ADD COLUMN category_array UUID[] DEFAULT ARRAY[]::UUID[];

-- Copy existing category values to the new array column
UPDATE bounty SET category_array = ARRAY[category] WHERE category IS NOT NULL;

-- Drop the old category column
ALTER TABLE bounty DROP COLUMN category;

-- Rename the new column to category
ALTER TABLE bounty RENAME COLUMN category_array TO category;

-- Make the category column NOT NULL (since we're ensuring all existing records have at least one category)
ALTER TABLE bounty ALTER COLUMN category SET NOT NULL;
