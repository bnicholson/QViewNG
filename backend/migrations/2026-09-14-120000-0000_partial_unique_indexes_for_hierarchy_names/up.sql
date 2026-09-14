-- Per-parent name uniqueness for divisions, division sessions and pool brackets should apply only
-- to live rows, so a name freed by a soft-deleted row can be reused. Postgres cannot put a WHERE
-- clause on a UNIQUE constraint, so each full constraint is dropped and replaced by a partial
-- unique index covering only active (del_fl = false) rows. This also makes the DB agree with the
-- model-level name checks, which already ignore soft-deleted rows.
--
-- The teamgroups -> pool_brackets 1:1 (teamgroups_pool_bracket_id_key) is intentionally left as a
-- full constraint: a soft-deleted team group still occupies its bracket.

-- Divisions: unique (tid, dname) among active rows.
ALTER TABLE divisions DROP CONSTRAINT divisions_tid_dname_key;
CREATE UNIQUE INDEX uq_divisions_tid_dname_active
    ON divisions (tid, dname) WHERE del_fl = false;

-- Division sessions: unique (did, name) among active rows.
ALTER TABLE division_sessions DROP CONSTRAINT uq_division_sessions_did_name;
CREATE UNIQUE INDEX uq_division_sessions_did_name_active
    ON division_sessions (did, name) WHERE del_fl = false;

-- Pool brackets: unique (division_session_id, name) among active rows.
ALTER TABLE pool_brackets DROP CONSTRAINT uq_pool_brackets_division_session_id_name;
CREATE UNIQUE INDEX uq_pool_brackets_division_session_id_name_active
    ON pool_brackets (division_session_id, name) WHERE del_fl = false;
