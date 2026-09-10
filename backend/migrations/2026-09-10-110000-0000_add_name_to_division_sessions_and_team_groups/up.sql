-- A division session's name must be unique within its parent division.
ALTER TABLE division_sessions ADD COLUMN name VARCHAR(64) NOT NULL;
ALTER TABLE division_sessions
    ADD CONSTRAINT uq_division_sessions_did_name UNIQUE (did, name);

-- A team group's name must be unique within its parent division session.
ALTER TABLE team_groups ADD COLUMN name VARCHAR(64) NOT NULL;
ALTER TABLE team_groups
    ADD CONSTRAINT uq_team_groups_division_session_id_name UNIQUE (division_session_id, name);
