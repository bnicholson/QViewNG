
ALTER TABLE users ADD COLUMN is_merged_user_id UUID REFERENCES users(id);
ALTER TABLE users ADD COLUMN when_merged TIMESTAMPTZ;
