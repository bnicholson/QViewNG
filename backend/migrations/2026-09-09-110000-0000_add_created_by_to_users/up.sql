-- The user (coach) who created this account, when it was created on someone else's behalf
-- (e.g. a coach creating a quizzer). NULL for self-registered users.
ALTER TABLE users ADD COLUMN created_by_userid UUID NULL REFERENCES users(id);
