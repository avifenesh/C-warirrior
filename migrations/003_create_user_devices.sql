-- Device-to-user mapping for multi-device sync
CREATE TABLE IF NOT EXISTS user_devices (
    device_id VARCHAR(64) PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_user_devices_user_id ON user_devices(user_id);