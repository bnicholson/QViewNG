
CREATE TABLE rosters (
    rosterid BIGSERIAL UNIQUE PRIMARY KEY NOT NULL,  -- a unique id for the roster; just an identifiable grouping
    name VARCHAR(64) NOT NULL,
    description VARCHAR(256)
);

CREATE TABLE rosters_coaches (
    coachid BIGINT NOT NULL REFERENCES users(id),
    rosterid BIGINT NOT NULL REFERENCES rosters(rosterid),
    CONSTRAINT uk_coachid_rosterid
        UNIQUE(coachid,rosterid)
);

CREATE TABLE rosters_quizzers (
    quizzerid BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    rosterid BIGINT NOT NULL REFERENCES rosters(rosterid),
    CONSTRAINT uk_quizzerid_rosterid
        UNIQUE(quizzerid,rosterid)
);
