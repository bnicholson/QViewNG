
CREATE TABLE statsgroups (
    sgid BIGSERIAL UNIQUE PRIMARY KEY NOT NULL,  -- a unique id for the statsgroup; just an identifiable grouping
    name VARCHAR(64) NOT NULL,
    description VARCHAR(256)
);

CREATE TABLE games_statsgroups (
    gid BIGINT NOT NULL REFERENCES games(gid),
    sgid BIGINT NOT NULL REFERENCES statsgroups(sgid) ON DELETE CASCADE
);
