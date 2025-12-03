-- Add user_id foreign key to existing tables
-- This links sessions, progress, and saves to authenticated users

-- Add user_id to sessions table
ALTER TABLE sessions ADD COLUMN IF NOT EXISTS user_id UUID REFERENCES users(id) ON DELETE CASCADE;
CREATE INDEX IF NOT EXISTS idx_sessions_user_id ON sessions(user_id) WHERE user_id IS NOT NULL;

-- Add user_id to player_progress table
ALTER TABLE player_progress ADD COLUMN IF NOT EXISTS user_id UUID REFERENCES users(id) ON DELETE CASCADE;
CREATE INDEX IF NOT EXISTS idx_player_progress_user_id ON player_progress(user_id) WHERE user_id IS NOT NULL;

-- Add user_id to save_slots table
ALTER TABLE save_slots ADD COLUMN IF NOT EXISTS user_id UUID REFERENCES users(id) ON DELETE CASCADE;
CREATE INDEX IF NOT EXISTS idx_save_slots_user_id ON save_slots(user_id) WHERE user_id IS NOT NULL;

-- Note: device_id columns are kept for backward compatibility during migration
-- They can be removed in a future migration once all users are authenticated

