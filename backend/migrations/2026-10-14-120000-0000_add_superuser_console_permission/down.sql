-- roles_permissions rows cascade on permission delete.
DELETE FROM permissions WHERE name = 'system:access_superuser_console';
