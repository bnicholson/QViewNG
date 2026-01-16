
CREATE TABLE tournaments_admins (
    tournamentid UUID NOT NULL REFERENCES tournaments(tid) ON DELETE CASCADE,
    adminid UUID NOT NULL REFERENCES users(id),
    role_description VARCHAR(64),
    access_lvl INT NOT NULL DEFAULT 0,  -- zero is the greatest scope of access; incrementing limits the scope a bit more; scope limits as well as number of levels of access are defined by the code; a different access control design may be used later on
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_tournamentid_adminid PRIMARY KEY (tournamentid,adminid)
);

CREATE TABLE rosters_coaches (
    coachid UUID NOT NULL REFERENCES users(id),
    rosterid BIGINT NOT NULL REFERENCES rosters(rosterid),
    CONSTRAINT pk_coachid_rosterid PRIMARY KEY (coachid,rosterid)
);

CREATE TABLE rosters_quizzers (
    quizzerid UUID NOT NULL REFERENCES users(id),
    rosterid BIGINT NOT NULL REFERENCES rosters(rosterid),
    CONSTRAINT pk_quizzerid_rosterid PRIMARY KEY (quizzerid,rosterid)
);
