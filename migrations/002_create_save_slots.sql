-- Save slots for game progress
CREATE TABLE IF NOT EXISTS save_slots (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    slot_name VARCHAR(50) NOT NULL,
    game_data JSONB NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(user_id, slot_name)
);

CREATE INDEX idx_save_slots_user_id ON save_slots(user_id);