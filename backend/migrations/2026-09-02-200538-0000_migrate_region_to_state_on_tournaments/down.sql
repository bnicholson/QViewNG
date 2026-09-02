ALTER TABLE tournaments ADD COLUMN region VARCHAR(64) NOT NULL DEFAULT '';
UPDATE tournaments SET region = state;
