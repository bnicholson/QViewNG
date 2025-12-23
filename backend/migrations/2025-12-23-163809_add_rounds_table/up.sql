
CREATE TABLE rounds (
    roundid BIGSERIAL UNIQUE PRIMARY KEY NOT NULL,       -- a unique id for the room
    did BIGINT REFERENCES divisions(did),                 -- a division can have many rounds
    scheduled_start_time TIMESTAMPTZ,                    -- value to be set when all rounds of a division exist
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
