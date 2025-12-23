
CREATE TABLE division_games (
    did BIGINT,                 -- Division id
    tdrri BIGINT,               -- ID of a particular quiz game
    PRIMARY Key (did,tdrri)     -- ensure that we don't duplicate quiz games in a division
);
