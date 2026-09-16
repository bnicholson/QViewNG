-- Rename the division_sessions table (and its id column) to roundgroups. A "round group" is a
-- time-bound grouping of rounds within a division; the new name reflects that its rounds are what
-- make it time-bound. This is a pure rename: relationships and data are unchanged.
ALTER TABLE division_sessions RENAME TO roundgroups;
ALTER TABLE roundgroups RENAME COLUMN division_session_id TO roundgroup_id;

-- The rounds FK column that points at it.
ALTER TABLE rounds RENAME COLUMN division_session_id TO roundgroup_id;

-- Keep the partial unique index name in step with the table.
ALTER INDEX uq_division_sessions_did_name_active RENAME TO uq_roundgroups_did_name_active;
