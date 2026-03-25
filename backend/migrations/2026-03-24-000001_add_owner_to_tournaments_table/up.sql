ALTER TABLE tournaments
    ADD COLUMN owner_id UUID NOT NULL REFERENCES users(id);
