-- Revert bounty category field from UUID array back to single UUID

-- First, add a new column for the single UUID
ALTER TABLE bounty ADD COLUMN category_single UUID;

-- Copy the first category from the array to the new single column
UPDATE bounty SET category_single = category[1] WHERE array_length(category, 1) > 0;

-- Drop the old category array column
ALTER TABLE bounty DROP COLUMN category;

-- Rename the new column to category
ALTER TABLE bounty RENAME COLUMN category_single TO category;

-- Make the category column NOT NULL
ALTER TABLE bounty ALTER COLUMN category SET NOT NULL;
