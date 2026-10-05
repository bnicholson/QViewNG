-- CRUD permissions for the domain entities behind the Tournament profile's sidebar that had none
-- of their own, plus PoolBracketGroup. They were previously guarded by a borrowed permission from
-- a neighbouring resource, or not at all:
--
--   roundgroup        (Division Sessions)  was division:*
--   poolbracket       (Pools, Brackets)    was division:*
--   poolbracketgroup                       was division:*
--   roomgroup         (Buildings)          was room:*
--   equipment         (Gear)               was unguarded; covers equipment, registrations, sets
--                                          and the eight gear types
--   tournamentadmin   (Admins)             was unguarded
--   statsgroup        (Server Stats)       was unguarded
--
-- Scoping is generic: a permission names the resource only, never the tournament it sits under.
-- Grants mirror the access each role has today, so no user gains or loses anything. They also
-- follow the existing convention that `member` is the role carrying every `:read` (a manager is
-- assigned `member` alongside `tournament_manager`), which is what `system_default_data.rs` builds:
--   member             → :read on each
--   tournament_manager → :create/:update/:delete (it already had these via the borrowed permissions)
--   tournament_admin   → :create/:update/:delete (same)
--   super_user         → every action

INSERT INTO permissions (name, resource, action)
SELECT r.resource || ':' || a.action, r.resource, a.action
FROM   (VALUES ('roundgroup'), ('poolbracketgroup'), ('poolbracket'),
               ('roomgroup'), ('equipment'), ('tournamentadmin'), ('statsgroup')) AS r(resource)
CROSS JOIN (VALUES ('create'), ('read'), ('update'), ('delete')) AS a(action)
ON CONFLICT (name) DO NOTHING;

-- member: read only.
INSERT INTO roles_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r CROSS JOIN permissions p
WHERE r.name = 'member'
  AND p.action = 'read'
  AND p.resource IN ('roundgroup','poolbracketgroup','poolbracket',
                     'roomgroup','equipment','tournamentadmin','statsgroup')
ON CONFLICT DO NOTHING;

-- tournament_manager, tournament_admin: write actions only; `member` supplies their reads.
INSERT INTO roles_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r CROSS JOIN permissions p
WHERE r.name IN ('tournament_manager', 'tournament_admin')
  AND p.action <> 'read'
  AND p.resource IN ('roundgroup','poolbracketgroup','poolbracket',
                     'roomgroup','equipment','tournamentadmin','statsgroup')
ON CONFLICT DO NOTHING;

-- super_user: every action on every resource, new ones included.
INSERT INTO roles_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r CROSS JOIN permissions p
WHERE r.name = 'super_user'
ON CONFLICT DO NOTHING;
