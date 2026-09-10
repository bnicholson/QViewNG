-- Scheduling structures for round-robin play. A division session groups teams for scheduling so
-- that, with more rooms available, the total game count can be reduced.

-- A division has zero-to-many division sessions; each session belongs to exactly one division.
CREATE TABLE division_sessions (
    division_session_id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    did UUID NOT NULL REFERENCES divisions(did),
    created_date TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    creator_userid UUID NOT NULL REFERENCES users(id),
    last_modified_date TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified_userid UUID NOT NULL REFERENCES users(id)
);

-- A division session has zero-to-many team groups (e.g. round-robin pools); each team group
-- belongs to exactly one division session.
CREATE TABLE team_groups (
    team_group_id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    division_session_id UUID NOT NULL REFERENCES division_sessions(division_session_id),
    type VARCHAR(64) NOT NULL DEFAULT 'pool',
    created_date TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    creator_userid UUID NOT NULL REFERENCES users(id),
    last_modified_date TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified_userid UUID NOT NULL REFERENCES users(id)
);

-- Bridge resolving the many-to-many relationship between teams and team groups: a team can belong
-- to many team groups, and a team group can contain many teams.
CREATE TABLE team_teamgroups (
    teamid UUID NOT NULL REFERENCES teams(teamid),
    team_group_id UUID NOT NULL REFERENCES team_groups(team_group_id),
    created_date TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    creator_userid UUID NOT NULL REFERENCES users(id),
    last_modified_date TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified_userid UUID NOT NULL REFERENCES users(id),
    CONSTRAINT pk_team_teamgroups PRIMARY KEY (teamid, team_group_id)
);
