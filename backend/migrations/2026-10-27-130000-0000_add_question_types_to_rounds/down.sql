-- roles_permissions rows cascade on permission delete.
DELETE FROM permissions WHERE name = 'round:upload_question_types';

-- Drops every round's saved question types; there is no earlier column to carry them back to.
ALTER TABLE rounds DROP COLUMN using_standard_question_types;
ALTER TABLE rounds DROP COLUMN question_types;
