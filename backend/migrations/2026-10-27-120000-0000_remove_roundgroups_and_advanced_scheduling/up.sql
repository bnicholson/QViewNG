-- Sessions (roundgroups) and the Advanced scheduling mode are removed. A schedule is built from
-- Rounds and Games only; what a session used to express (e.g. a break between blocks of rounds) is
-- now worked out from the rounds' start times. Rounds go back to belonging directly to a division.
--
-- Carried forward: every round (live or soft-deleted) takes its session's division.
-- Dropped on purpose: session names and audit columns, each tournament's scheduling mode, and the
-- roundgroup:* permissions. down.sql can't bring that data back (see its header).

ALTER TABLE rounds ADD COLUMN did UUID REFERENCES divisions(did);

UPDATE rounds r
SET did = rg.did
FROM roundgroups rg
WHERE rg.roundgroup_id = r.roundgroup_id;

ALTER TABLE rounds ALTER COLUMN did SET NOT NULL;

-- Round names were unique per session and become unique per division. Two live rounds in different
-- sessions of one division may share a name (e.g. "Round 1" in two sessions); keep both by appending
-- the session name. Session names are unique per division, so the results can't collide with each
-- other. (The UPDATE's EXISTS reads the pre-update rows, so every member of a clash is renamed.)
UPDATE rounds r
SET name = left(r.name || ' (' || rg.name || ')', 64)
FROM roundgroups rg
WHERE rg.roundgroup_id = r.roundgroup_id
  AND r.del_fl = false
  AND EXISTS (
      SELECT 1 FROM rounds o
      WHERE o.did = r.did AND o.name = r.name AND o.del_fl = false AND o.roundid <> r.roundid
  );

-- Dropping the column also drops rounds_division_session_id_fkey and
-- uq_rounds_division_session_id_name_active.
ALTER TABLE rounds DROP COLUMN roundgroup_id;
CREATE UNIQUE INDEX uq_rounds_did_name_active
    ON rounds (did, name) WHERE del_fl = false;

DROP TABLE roundgroups;

ALTER TABLE tournaments DROP COLUMN use_advanced_scheduling;

DELETE FROM roles_permissions
WHERE permission_id IN (SELECT id FROM permissions WHERE resource = 'roundgroup');
DELETE FROM permissions WHERE resource = 'roundgroup';
