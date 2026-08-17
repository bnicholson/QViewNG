-- This file should undo anything in `up.sql`
ALTER TABLE rooms DROP COLUMN ping_game_id;
ALTER TABLE rooms DROP COLUMN ping_round;
ALTER TABLE rooms DROP COLUMN ping_room;
