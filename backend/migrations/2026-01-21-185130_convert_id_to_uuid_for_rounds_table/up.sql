
ALTER TABLE games DROP COLUMN roundid;

DROP TABLE rounds;

CREATE TABLE rounds (
    roundid UUID PRIMARY KEY DEFAULT gen_random_uuid(),  -- a unique id for the round
    did UUID NOT NULL REFERENCES divisions(did),         -- a division can have many rounds
    scheduled_start_time TIMESTAMPTZ,                    -- value to be set when all rounds of a division exist
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE games ADD COLUMN roundid UUID REFERENCES rounds(roundid);
