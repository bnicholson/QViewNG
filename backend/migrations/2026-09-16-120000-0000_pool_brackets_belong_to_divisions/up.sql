-- Pool brackets move from belonging to a (time-bound) division session to belonging directly to a
-- division. A division is not time-bound, and its pools/brackets are stable structural groupings
-- rather than per-session ones, so the parent should be the division.

-- New parent: the division. Nullable at first so we can backfill existing rows.
ALTER TABLE pool_brackets ADD COLUMN divisionid UUID REFERENCES divisions(did);

-- Backfill each bracket's division from the division of its current session.
UPDATE pool_brackets pb
   SET divisionid = ds.did
  FROM division_sessions ds
 WHERE pb.division_session_id = ds.division_session_id;

-- Every live row is mapped now; enforce the relationship.
ALTER TABLE pool_brackets ALTER COLUMN divisionid SET NOT NULL;

-- Drop the old session relationship. Dropping the column also drops its FK and the partial unique
-- index that included it (uq_pool_brackets_division_session_id_name_active).
ALTER TABLE pool_brackets DROP COLUMN division_session_id;

-- Re-establish active-row name uniqueness, now scoped to the division instead of the session.
CREATE UNIQUE INDEX uq_pool_brackets_divisionid_name_active
    ON pool_brackets (divisionid, name) WHERE del_fl = false;
