
CREATE TABLE tournaments_admins (
    tournamentid BIGINT NOT NULL REFERENCES tournaments(tid) ON DELETE CASCADE,
    adminid BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_description VARCHAR(64),
    access_lvl INT NOT NULL DEFAULT 0,  -- zero is the greatest scope of access; incrementing limits the scope a bit more; scope limits as well as number of levels of access are defined by the code; a different access control design may be used later on
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
