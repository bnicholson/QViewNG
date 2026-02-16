
ALTER TABLE rooms ADD COLUMN clientkey VARCHAR(64) NOT NULL DEFAULT '';      -- Unique identifier for the quizmachine client; registration key

ALTER TABLE computers ADD COLUMN clientkey VARCHAR(64) NOT NULL DEFAULT '';  -- Unique identifier for the quizmachine client; registration key
