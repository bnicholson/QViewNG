
ALTER TABLE users
    ALTER COLUMN id TYPE INT;

ALTER TABLE user_sessions
    ALTER COLUMN id TYPE INT;

ALTER TABLE user_sessions
    ALTER COLUMN user_id TYPE INT;

ALTER TABLE user_roles
    ALTER COLUMN user_id TYPE INT;

ALTER TABLE user_permissions
    ALTER COLUMN user_id TYPE INT;
