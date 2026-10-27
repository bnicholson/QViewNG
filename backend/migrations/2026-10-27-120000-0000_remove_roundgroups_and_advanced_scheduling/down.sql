-- Restores the schema, but not the data up.sql dropped:
--   * Session names and groupings are gone, so each division that has rounds gets a single
--     "Session 1" (owned by the tournament owner) holding all of its rounds — the shape a new
--     division is seeded with.
--   * Every tournament goes back to Normal scheduling mode (use_advanced_scheduling = false).
--   * Rounds renamed by up.sql to avoid a clash keep their " (session name)" suffix.
--   * roundgroup:* permissions are re-granted per the default role grants; any custom grants made
--     to other roles are not restored.

ALTER TABLE tournaments ADD COLUMN use_advanced_scheduling BOOLEAN NOT NULL DEFAULT false;

CREATE TABLE roundgroups (
    roundgroup_id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    did UUID NOT NULL REFERENCES divisions(did),
    created_date TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    creator_userid UUID NOT NULL REFERENCES users(id),
    last_modified_date TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified_userid UUID NOT NULL REFERENCES users(id),
    name VARCHAR(64) NOT NULL,
    del_fl BOOLEAN NOT NULL DEFAULT FALSE
);
CREATE UNIQUE INDEX uq_roundgroups_did_name_active
    ON roundgroups (did, name) WHERE del_fl = false;

INSERT INTO roundgroups (did, name, creator_userid, last_modified_userid)
SELECT d.did, 'Session 1', t.owner_id, t.owner_id
FROM divisions d
JOIN tournaments t ON t.tid = d.tid
WHERE EXISTS (SELECT 1 FROM rounds r WHERE r.did = d.did);

ALTER TABLE rounds ADD COLUMN roundgroup_id UUID REFERENCES roundgroups(roundgroup_id);

UPDATE rounds r
SET roundgroup_id = rg.roundgroup_id
FROM roundgroups rg
WHERE rg.did = r.did;

ALTER TABLE rounds ALTER COLUMN roundgroup_id SET NOT NULL;

DROP INDEX uq_rounds_did_name_active;
ALTER TABLE rounds DROP COLUMN did;
CREATE UNIQUE INDEX uq_rounds_division_session_id_name_active
    ON rounds (roundgroup_id, name) WHERE del_fl = false;

INSERT INTO permissions (name, resource, action)
SELECT 'roundgroup:' || a.action, 'roundgroup', a.action
FROM (VALUES ('create'), ('read'), ('update'), ('delete')) AS a(action)
ON CONFLICT (name) DO NOTHING;

INSERT INTO roles_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r CROSS JOIN permissions p
WHERE p.resource = 'roundgroup'
  AND (   (r.name = 'member' AND p.action = 'read')
       OR (r.name IN ('tournament_manager', 'tournament_admin') AND p.action <> 'read')
       OR r.name = 'super_user')
ON CONFLICT DO NOTHING;
