
ALTER TABLE teams DROP COLUMN quizzer_one;
ALTER TABLE teams DROP COLUMN quizzer_two;
ALTER TABLE teams DROP COLUMN quizzer_three;
ALTER TABLE teams DROP COLUMN quizzer_four;
ALTER TABLE teams DROP COLUMN quizzer_five;
ALTER TABLE teams DROP COLUMN quizzer_six;

ALTER TABLE teams ADD COLUMN quizzer_one_id UUID REFERENCES users(id);
ALTER TABLE teams ADD COLUMN quizzer_two_id UUID REFERENCES users(id);
ALTER TABLE teams ADD COLUMN quizzer_three_id UUID REFERENCES users(id);
ALTER TABLE teams ADD COLUMN quizzer_four_id UUID REFERENCES users(id);
ALTER TABLE teams ADD COLUMN quizzer_five_id UUID REFERENCES users(id);
ALTER TABLE teams ADD COLUMN quizzer_six_id UUID REFERENCES users(id);
