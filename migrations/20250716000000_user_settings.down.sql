-- Remove user settings columns from users table

ALTER TABLE users 
DROP COLUMN IF EXISTS website,
DROP COLUMN IF EXISTS email_notifications,
DROP COLUMN IF EXISTS push_notifications,
DROP COLUMN IF EXISTS milestone_updates,
DROP COLUMN IF EXISTS funding_updates,
DROP COLUMN IF EXISTS dao_proposals,
DROP COLUMN IF EXISTS prediction_markets,
DROP COLUMN IF EXISTS profile_visibility,
DROP COLUMN IF EXISTS show_funding_history,
DROP COLUMN IF EXISTS show_prediction_history,
DROP COLUMN IF EXISTS two_factor_enabled,
DROP COLUMN IF EXISTS dark_mode,
DROP COLUMN IF EXISTS language,
DROP COLUMN IF EXISTS timezone,
DROP COLUMN IF EXISTS display_currency; 