-- Drop trigger and function
DROP TRIGGER IF EXISTS update_notifications_updated_at ON notifications;
DROP FUNCTION IF EXISTS update_notifications_updated_at();

-- Drop indexes
DROP INDEX IF EXISTS idx_notifications_user_id;
DROP INDEX IF EXISTS idx_notifications_created_at;
DROP INDEX IF EXISTS idx_notifications_is_read;
DROP INDEX IF EXISTS idx_notifications_type;

-- Drop notifications table
DROP TABLE IF EXISTS notifications; 