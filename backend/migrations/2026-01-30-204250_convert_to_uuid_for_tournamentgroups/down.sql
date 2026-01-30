
DROP TABLE tournamentgroups_tournaments;
DROP TABLE tournamentgroups;

CREATE TABLE tournamentgroups (
    tgid BIGSERIAL UNIQUE PRIMARY KEY NOT NULL,  -- a unique id for the tournamentgroup; just an identifiable grouping
    name VARCHAR(64) NOT NULL,
    description VARCHAR(256),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE tournamentgroups_tournaments (
    tournamentid UUID NOT NULL REFERENCES tournaments(tid),
    tournamentgroupid BIGINT NOT NULL REFERENCES tournamentgroups(tgid) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tournamentgroupid,tournamentid)
);
