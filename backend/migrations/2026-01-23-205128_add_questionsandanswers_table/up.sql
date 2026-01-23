
-- *assumes Nazarene quizzing as the use case (mimicking SetMaker CSV export)
CREATE TABLE questionsandanswers (
    id UUID PRIMARY KEY,        -- *BIGSERIAL intentional (human-readable)*
    -- where 0 is local coach-accessible questions for use at practices & 
    -- non-zero values are higher and higher lvls of competition
    competition_lvl INTEGER NOT NULL,
    -- based on Nazarene Scripture rotation, 1 through 8, where John = 1 and 1 & 2 Corinthians = 8
    quiz_season INTEGER NOT NULL,
    type VARCHAR(32) NOT NULL,
    question TEXT NOT NULL,
    answer TEXT NOT NULL,
    book VARCHAR(32) NOT NULL,
    chapter INTEGER NOT NULL,
    verses VARCHAR(32) NOT NULL,
    beginning_verse INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(question,book,chapter,verses) 
);

ALTER TABLE rounds ADD COLUMN scheduled_question_one_id UUID REFERENCES questionsandanswers(id);
ALTER TABLE rounds ADD COLUMN scheduled_question_two_id UUID REFERENCES questionsandanswers(id);
ALTER TABLE rounds ADD COLUMN scheduled_question_three_id UUID REFERENCES questionsandanswers(id);
ALTER TABLE rounds ADD COLUMN scheduled_question_four_id UUID REFERENCES questionsandanswers(id);
ALTER TABLE rounds ADD COLUMN scheduled_question_five_id UUID REFERENCES questionsandanswers(id);
ALTER TABLE rounds ADD COLUMN scheduled_question_six_id UUID REFERENCES questionsandanswers(id);
ALTER TABLE rounds ADD COLUMN scheduled_question_seven_id UUID REFERENCES questionsandanswers(id);
ALTER TABLE rounds ADD COLUMN scheduled_question_eight_id UUID REFERENCES questionsandanswers(id);
ALTER TABLE rounds ADD COLUMN scheduled_question_nine_id UUID REFERENCES questionsandanswers(id);
ALTER TABLE rounds ADD COLUMN scheduled_question_ten_id UUID REFERENCES questionsandanswers(id);
ALTER TABLE rounds ADD COLUMN scheduled_question_eleven_id UUID REFERENCES questionsandanswers(id);
ALTER TABLE rounds ADD COLUMN scheduled_question_twelve_id UUID REFERENCES questionsandanswers(id);
ALTER TABLE rounds ADD COLUMN scheduled_question_thirteen_id UUID REFERENCES questionsandanswers(id);
ALTER TABLE rounds ADD COLUMN scheduled_question_fourteen_id UUID REFERENCES questionsandanswers(id);
ALTER TABLE rounds ADD COLUMN scheduled_question_fifteen_id UUID REFERENCES questionsandanswers(id);
ALTER TABLE rounds ADD COLUMN scheduled_question_sixteen_id UUID REFERENCES questionsandanswers(id);
ALTER TABLE rounds ADD COLUMN scheduled_question_seventeen_id UUID REFERENCES questionsandanswers(id);
ALTER TABLE rounds ADD COLUMN scheduled_question_eighteen_id UUID REFERENCES questionsandanswers(id);
ALTER TABLE rounds ADD COLUMN scheduled_question_nineteen_id UUID REFERENCES questionsandanswers(id);
ALTER TABLE rounds ADD COLUMN scheduled_question_twenty_id UUID REFERENCES questionsandanswers(id);
