-- Rename team_groups -> pool_brackets (including its primary-key column and the per-session unique
-- name constraint).
ALTER TABLE team_groups RENAME TO pool_brackets;
ALTER TABLE pool_brackets RENAME COLUMN team_group_id TO pool_bracket_id;
ALTER TABLE pool_brackets
    RENAME CONSTRAINT uq_team_groups_division_session_id_name TO uq_pool_brackets_division_session_id_name;

-- New teamgroups table: the same shape the original had, minus the name column, with
-- division_session_id replaced by a required pool_bracket_id FK. One-to-one with pool_brackets:
-- each team group belongs to exactly one bracket (NOT NULL), and each bracket has at most one team
-- group (UNIQUE).
CREATE TABLE teamgroups (
    team_group_id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    pool_bracket_id UUID NOT NULL UNIQUE REFERENCES pool_brackets(pool_bracket_id),
    type VARCHAR(64) NOT NULL DEFAULT 'pool',
    created_date TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    creator_userid UUID NOT NULL REFERENCES users(id),
    last_modified_date TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified_userid UUID NOT NULL REFERENCES users(id)
);

-- Repoint the bridge's team_group_id FK from pool_brackets (formerly team_groups) to the new
-- teamgroups table.
ALTER TABLE team_teamgroups DROP CONSTRAINT team_teamgroups_team_group_id_fkey;
ALTER TABLE team_teamgroups
    ADD CONSTRAINT team_teamgroups_team_group_id_fkey
    FOREIGN KEY (team_group_id) REFERENCES teamgroups(team_group_id);

-- Each game belongs to a pool bracket. (Requires an empty games table; only applied to a fresh DB.)
ALTER TABLE games ADD COLUMN poolbracket_id UUID NOT NULL REFERENCES pool_brackets(pool_bracket_id);
