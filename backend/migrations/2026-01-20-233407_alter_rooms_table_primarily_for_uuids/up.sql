
-- dependency to take care of, before and after main interest here:
ALTER TABLE games DROP COLUMN roomid;

-- Main interest of this migration script:
DROP TABLE rooms;

CREATE TABLE rooms (
    roomid UUID PRIMARY KEY DEFAULT gen_random_uuid(),  -- a unique id for the room
    tid UUID NOT NULL,                                  -- tournament id
    name varchar(32) NOT NULL,                          -- room name itself
    building varchar(32) NOT NULL DEFAULT '',           -- what build is this room in
    comments text NOT NULL DEFAULT '',                  -- comments on the room
    UNIQUE (roomid,tid)
);

-- Some updates to Games because its references still have BigInt for tour 
-- and div tables even though they are UUIDs now:
ALTER TABLE games DROP COLUMN tournamentid;
ALTER TABLE games ADD COLUMN tournamentid UUID REFERENCES tournaments(tid);

ALTER TABLE games DROP COLUMN divisionid;
ALTER TABLE games ADD COLUMN divisionid UUID REFERENCES divisions(did);

-- The last step actually needed here:
ALTER TABLE games ADD COLUMN roomid UUID REFERENCES rooms(roomid);
