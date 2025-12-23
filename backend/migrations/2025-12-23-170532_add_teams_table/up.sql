
CREATE TABLE teams (
    teamid BIGSERIAL UNIQUE PRIMARY KEY NOT NULL,           -- a unique id for the team
    divisionid BIGINT REFERENCES divisions(did) NOT NULL,   -- teams are owned by divisions; a division can have many teams
    coachid BIGINT REFERENCES users(id) NOT NULL,           -- there is 1 (primary) coach and they are a User
    name VARCHAR(128) NOT NULL
);
