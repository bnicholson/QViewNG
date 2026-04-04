-- tournaments: back-fill creator_id from owner_id (same person created it).
ALTER TABLE tournaments
    ADD COLUMN creator_id UUID REFERENCES users(id);

UPDATE tournaments SET creator_id = owner_id;

ALTER TABLE tournaments
    ALTER COLUMN creator_id SET NOT NULL;

-- tournamentgroups: back-fill both columns from the earliest user as a sentinel.
ALTER TABLE tournamentgroups
    ADD COLUMN creator_id UUID REFERENCES users(id),
    ADD COLUMN owner_id   UUID REFERENCES users(id);

UPDATE tournamentgroups
    SET creator_id = (SELECT id FROM users ORDER BY created_at LIMIT 1),
        owner_id   = (SELECT id FROM users ORDER BY created_at LIMIT 1)
    WHERE creator_id IS NULL;

ALTER TABLE tournamentgroups
    ALTER COLUMN creator_id SET NOT NULL,
    ALTER COLUMN owner_id   SET NOT NULL;
