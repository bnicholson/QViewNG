-- A stats group is scoped to a required tournament and, optionally, a single division.
ALTER TABLE statsgroups
    ADD COLUMN tournament_id UUID NOT NULL REFERENCES tournaments(tid) ON DELETE CASCADE,
    ADD COLUMN division_id   UUID          REFERENCES divisions(did)   ON DELETE CASCADE;
