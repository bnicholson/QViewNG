
DROP TABLE games_statsgroups;
DROP TABLE statsgroups;
DROP TABLE games;

CREATE TABLE games (
    gid UUID PRIMARY KEY DEFAULT gen_random_uuid(),    -- Unique identifier    
    org VARCHAR(48) NOT NULL,                          -- What organizatoin is this game from
    tournamentid UUID REFERENCES tournaments(tid),     -- What Tournament is this game for
    divisionid UUID REFERENCES divisions(did),         -- What Division
    roomid UUID NOT NULL REFERENCES rooms(roomid),     -- What Room is this game in
    roundid UUID NOT NULL REFERENCES rounds(roundid),  -- What Round are we in
    clientkey VARCHAR(64) NOT NULL DEFAULT '',         -- Unique identifier for the quizmachine client
    ignore BOOLEAN NOT NULL DEFAULT false,             -- Should this game be ignored
    ruleset VARCHAR(32) NOT NULL DEFAULT 'Nazarene',   -- What rules were used by this game
    leftteamid UUID NOT NULL REFERENCES teams(teamid),
    centerteamid UUID REFERENCES teams(teamid),
    rightteamid UUID NOT NULL REFERENCES teams(teamid),
    quizmasterid UUID NOT NULL REFERENCES users(id),
    contentjudgeid UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (org, tournamentid, divisionid, roomid, roundid, clientkey)
);

-- an identifiable grouping of Games for purposes of calculating stats:
CREATE TABLE statsgroups (
    sgid UUID PRIMARY KEY DEFAULT gen_random_uuid(),  -- updating type: BIGSERIAL -> UUID
    name VARCHAR(64) NOT NULL,
    description VARCHAR(256),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE games_statsgroups (
    gameid UUID NOT NULL REFERENCES games(gid),
    statsgroupid UUID NOT NULL REFERENCES statsgroups(sgid) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_gid_sgid PRIMARY KEY (gameid,statsgroupid)
);
