-- More latest-ping data reported by a room's QuizMachine client. All nullable (blank by default).
ALTER TABLE rooms ADD COLUMN ping_room VARCHAR(32);
ALTER TABLE rooms ADD COLUMN ping_round VARCHAR(64);
-- Optional reference to the game the room's client last pinged about.
ALTER TABLE rooms ADD COLUMN ping_game_id UUID REFERENCES games(gid) ON DELETE SET NULL;
