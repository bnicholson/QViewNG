-- roles_permissions rows cascade on permission delete.
DELETE FROM permissions
WHERE resource IN ('roundgroup','poolbracketgroup','poolbracket',
                   'roomgroup','equipment','tournamentadmin','statsgroup');
