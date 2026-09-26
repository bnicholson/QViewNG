-- Whether this room may host 3-team games. Defaults false; games with a center team can only be
-- scheduled in a room where this is true.
ALTER TABLE rooms ADD COLUMN supports_threeteam_games BOOLEAN NOT NULL DEFAULT false;
