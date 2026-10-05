-- roles_permissions rows cascade on permission delete.
DELETE FROM permissions WHERE name IN (
    'system:administer',
    'system:access_dev_tools',
    'tournament:read_pairing_code',
    'tournament:read_audit',
    'tournament:update_schedule',
    'user:read_any',
    'user:read_tournament_participants'
);
