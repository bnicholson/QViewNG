-- Drop bridge table PKs and UUID FK columns
ALTER TABLE users_roles DROP CONSTRAINT IF EXISTS users_roles_pkey;
ALTER TABLE users_roles DROP COLUMN IF EXISTS role_id;

ALTER TABLE roles_permissions DROP CONSTRAINT IF EXISTS roles_permissions_pkey;
ALTER TABLE roles_permissions DROP COLUMN IF EXISTS role_id;
ALTER TABLE roles_permissions DROP COLUMN IF EXISTS permission_id;

-- Revert roles.id from UUID back to SERIAL
ALTER TABLE roles DROP COLUMN IF EXISTS id;
ALTER TABLE roles ADD COLUMN id SERIAL PRIMARY KEY;

-- Revert permissions.id from UUID back to SERIAL
ALTER TABLE permissions DROP COLUMN IF EXISTS id;
ALTER TABLE permissions ADD COLUMN id SERIAL PRIMARY KEY;

-- Re-add FK columns as INT in bridge tables and restore PKs
ALTER TABLE roles_permissions ADD COLUMN role_id       INT NOT NULL REFERENCES roles(id) ON DELETE CASCADE;
ALTER TABLE roles_permissions ADD COLUMN permission_id INT NOT NULL REFERENCES permissions(id) ON DELETE CASCADE;
ALTER TABLE roles_permissions ADD PRIMARY KEY (role_id, permission_id);

ALTER TABLE users_roles ADD COLUMN role_id INT NOT NULL REFERENCES roles(id) ON DELETE CASCADE;
ALTER TABLE users_roles ADD PRIMARY KEY (user_id, role_id);
