ALTER TABLE rooms
    ADD COLUMN quizmaster_id UUID REFERENCES users(id),
    ADD COLUMN contentjudge_id UUID REFERENCES users(id);
