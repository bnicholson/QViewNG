-- A poolbracketgroup groups the pool_brackets that happen concurrently within a division: every team
-- competes in exactly one pool per group, and a later group may re-pool the same teams differently.
-- It belongs to a division (parent); pool_brackets reference it (child). Audit + soft delete per convention.
CREATE TABLE poolbracketgroups (
    poolbracketgroupid UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    divisionid UUID NOT NULL REFERENCES divisions(did),
    name VARCHAR(64) NOT NULL,
    created_date TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    creator_userid UUID NOT NULL REFERENCES users(id),
    last_modified_date TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified_userid UUID NOT NULL REFERENCES users(id),
    del_fl BOOLEAN NOT NULL DEFAULT false
);

-- Nullable FK: a pool bracket belongs to a poolbracketgroup (nullable so existing rows migrate cleanly).
ALTER TABLE pool_brackets ADD COLUMN poolbracketgroupid UUID REFERENCES poolbracketgroups(poolbracketgroupid);

-- Backfill: give every division that already has pools a "Group 1" and put its pools in it, so the
-- per-group placement UI works on existing data without a reseed.
INSERT INTO poolbracketgroups (divisionid, name, creator_userid, last_modified_userid)
SELECT d.did, 'Group 1', t.owner_id, t.owner_id
FROM divisions d
JOIN tournaments t ON t.tid = d.tid
WHERE EXISTS (SELECT 1 FROM pool_brackets pb WHERE pb.divisionid = d.did AND pb.del_fl = false);

UPDATE pool_brackets pb
SET poolbracketgroupid = g.poolbracketgroupid
FROM poolbracketgroups g
WHERE g.divisionid = pb.divisionid AND pb.poolbracketgroupid IS NULL;
