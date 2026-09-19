-- Default length (in minutes) of a round for this tournament, used by the schedule Auto-Schedule features.
ALTER TABLE tournaments ADD COLUMN default_round_duration INTEGER NOT NULL DEFAULT 30;
