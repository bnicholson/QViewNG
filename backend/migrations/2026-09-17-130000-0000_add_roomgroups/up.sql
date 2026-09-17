-- A roomgroup groups rooms (e.g. a building) and belongs to a tournament, so a tournament's
-- roomgroups are a direct lookup. Rooms reference a roomgroup so the tournament can look up its
-- rooms by roomgroup. Audit columns + soft delete follow the codebase convention.
CREATE TABLE roomgroups (
    roomgroupid UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    tournamentid UUID NOT NULL REFERENCES tournaments(tid),
    type VARCHAR(64) NOT NULL DEFAULT 'building',
    name VARCHAR(128) NOT NULL,
    notes TEXT NOT NULL DEFAULT '',
    created_date TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    creator_userid UUID NOT NULL REFERENCES users(id),
    last_modified_date TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified_userid UUID NOT NULL REFERENCES users(id),
    del_fl BOOLEAN NOT NULL DEFAULT false
);

-- Nullable FK: a room need not belong to a roomgroup.
ALTER TABLE rooms ADD COLUMN roomgroupid UUID REFERENCES roomgroups(roomgroupid);
