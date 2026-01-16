
ALTER TABLE games DROP COLUMN quizmasterid;
ALTER TABLE games DROP COLUMN contentjudgeid;

ALTER TABLE user_sessions DROP COLUMN user_id;

ALTER TABLE user_permissions DROP CONSTRAINT user_permissions_pkey;
ALTER TABLE user_permissions DROP COLUMN user_id;

ALTER TABLE user_roles DROP CONSTRAINT user_roles_pkey;
ALTER TABLE user_roles DROP COLUMN user_id;

ALTER TABLE teams DROP COLUMN coachid;

ALTER TABLE users DROP COLUMN id;
ALTER TABLE users ADD COLUMN id UUID PRIMARY KEY DEFAULT gen_random_uuid();

ALTER TABLE teams ADD COLUMN coachid UUID NOT NULL REFERENCES users(id);

ALTER TABLE user_roles ADD COLUMN user_id UUID NOT NULL REFERENCES users(id);
ALTER TABLE user_roles ADD PRIMARY KEY (user_id, role);

ALTER TABLE user_permissions ADD COLUMN user_id UUID NOT NULL REFERENCES users(id);
ALTER TABLE user_permissions ADD PRIMARY KEY (user_id, permission);

ALTER TABLE user_sessions ADD COLUMN user_id UUID NOT NULL REFERENCES users(id);

ALTER TABLE games ADD COLUMN quizmasterid UUID NOT NULL REFERENCES users(id);
ALTER TABLE games ADD COLUMN contentjudgeid UUID NOT NULL REFERENCES users(id);
