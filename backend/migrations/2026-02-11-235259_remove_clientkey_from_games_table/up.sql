
-- Dropping here in favor of higher-resolution clientkey in QuizEvents table later on
-- (in case a computer or QuizMachine program fails, can't com back, and needs to be replaced mid-Game).
ALTER TABLE games DROP COLUMN clientkey;
