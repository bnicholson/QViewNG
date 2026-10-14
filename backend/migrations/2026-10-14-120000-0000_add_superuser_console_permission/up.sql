-- Reaching the Superuser Console (/su-console) gets its own permission rather than riding on
-- `system:administer`, so the console's access can be granted or withheld on its own.
-- Only super_user holds it, matching who could reach the console before.

INSERT INTO permissions (name, resource, action) VALUES
    ('system:access_superuser_console', 'system', 'access_superuser_console')
ON CONFLICT (name) DO NOTHING;

INSERT INTO roles_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r CROSS JOIN permissions p
WHERE r.name = 'super_user'
  AND p.name = 'system:access_superuser_console'
ON CONFLICT DO NOTHING;
