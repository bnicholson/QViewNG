
ALTER TABLE games DROP COLUMN leftteamid;
ALTER TABLE games DROP COLUMN centerteamid;
ALTER TABLE games DROP COLUMN rightteamid;

DROP TABLE teams;

CREATE TABLE teams (
    teamid UUID PRIMARY KEY DEFAULT gen_random_uuid(),              -- a unique id for the team
    did UUID NOT NULL REFERENCES divisions(did) ON DELETE CASCADE,  -- teams are owned by divisions; a division can have many teams
    coachid UUID NOT NULL REFERENCES users(id),                     -- there is 1 (primary) coach and they are a User
    name VARCHAR(128) NOT NULL,
    quizzer_one UUID REFERENCES users(id),
    quizzer_two UUID REFERENCES users(id),
    quizzer_three UUID REFERENCES users(id),
    quizzer_four UUID REFERENCES users(id),
    quizzer_five UUID REFERENCES users(id),
    quizzer_six UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(did,name)
);

ALTER TABLE games ADD COLUMN leftteamid UUID NOT NULL REFERENCES teams(teamid);
ALTER TABLE games ADD COLUMN centerteamid UUID NOT NULL REFERENCES teams(teamid);
ALTER TABLE games ADD COLUMN rightteamid UUID NOT NULL REFERENCES teams(teamid);
