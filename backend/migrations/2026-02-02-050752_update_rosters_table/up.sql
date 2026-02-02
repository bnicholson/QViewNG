
DROP TABLE rosters_coaches;
DROP TABLE rosters_quizzers;
DROP TABLE rosters;

CREATE TABLE rosters (
    rosterid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(64) NOT NULL,
    description VARCHAR(256),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE rosters_coaches (
    coachid UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    rosterid UUID NOT NULL REFERENCES rosters(rosterid),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT pk_coachid_rosterid PRIMARY KEY (coachid,rosterid)
);

CREATE TABLE rosters_quizzers (
    quizzerid UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    rosterid UUID NOT NULL REFERENCES rosters(rosterid),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT pk_quizzerid_rosterid PRIMARY KEY (quizzerid,rosterid)
);
