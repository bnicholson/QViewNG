-- Authorization previously fell back to role names in a number of places (the `super_user` bypass,
-- the `tournament_manager` / `tournament_admin` checks behind the pairing code, the audit columns,
-- schedule editing, and the user-visibility scopes). Those checks now resolve against permissions,
-- which needs a handful of permissions that are not a plain CRUD action on a resource.
--
-- Grants below reproduce what each role could already do, so no user gains or loses access:
--   super_user         → all of them
--   tournament_manager → the three tournament ones + user:read_tournament_participants
--   tournament_admin   → the three tournament ones
--   member             → none

INSERT INTO permissions (name, resource, action) VALUES
    -- Unrestricted access; the permission form of what the `super_user` role used to mean.
    ('system:administer',                   'system',     'administer'),
    -- Reach the /dev page and the developer tooling it links to.
    ('system:access_dev_tools',             'system',     'access_dev_tools'),
    -- See a tournament's pairing code (withheld from the tournament payload otherwise).
    ('tournament:read_pairing_code',        'tournament', 'read_pairing_code'),
    -- See "Created" / "Last Modified" audit columns on a tournament's entity tables.
    ('tournament:read_audit',               'tournament', 'read_audit'),
    -- Use the schedule editor rather than the read-only schedule view.
    ('tournament:update_schedule',          'tournament', 'update_schedule'),
    -- Read any user's account, not just one's own.
    ('user:read_any',                       'user',       'read_any'),
    -- Read the users participating in tournaments one manages.
    ('user:read_tournament_participants',   'user',       'read_tournament_participants')
ON CONFLICT (name) DO NOTHING;

-- super_user: everything, including every pre-existing permission it may be missing.
INSERT INTO roles_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r CROSS JOIN permissions p
WHERE r.name = 'super_user'
ON CONFLICT DO NOTHING;

-- tournament_manager: the tournament-scoped views it already had by role, plus its user scope.
INSERT INTO roles_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r CROSS JOIN permissions p
WHERE r.name = 'tournament_manager'
  AND p.name IN (
    'tournament:read_pairing_code',
    'tournament:read_audit',
    'tournament:update_schedule',
    'user:read_tournament_participants'
  )
ON CONFLICT DO NOTHING;

-- tournament_admin: the same tournament-scoped views (it never had the user scope).
INSERT INTO roles_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r CROSS JOIN permissions p
WHERE r.name = 'tournament_admin'
  AND p.name IN (
    'tournament:read_pairing_code',
    'tournament:read_audit',
    'tournament:update_schedule'
  )
ON CONFLICT DO NOTHING;
