ALTER TABLE tournaments
    ADD COLUMN owner_id UUID REFERENCES users(id);
