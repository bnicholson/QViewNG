
DROP TABLE quizevents;  -- we're renaming and redefining this

CREATE TABLE gameevents (
       gid UUID NOT NULL REFERENCES games(gid),     -- Game ID
       question integer NOT NULL,                   -- Question (part of the key), what question is this event
       eventnum integer NOT NULL,                   -- There can be multiple events per question
       name varchar(64) NOT NULL,                   -- A Quizzer name or team name
       team integer NOT NULL,                       -- team #   (0-2)
       -- teamid UUID NOT NULL,                     -- could be added to QuizMachine for more definite data communication
       quizzer integer NOT NULL,                    -- quizzer # (0-4)
       -- quizzerid UUID NOT NULL,                  -- could be added to QuizMachine for more definite data communication
       event varchar(2) NOT NULL,                   -- event identifier (TC, BE, ...)
       parm1 varchar(8) NOT NULL,                   -- parameter for an event
       parm2 varchar(8) NOT NULL,                   -- parameter #2 for an event
       clientts timestamptz NOT NULL,               -- timestamp when the client thought this happened
       serverts timestamptz NOT NULL,               -- server time when this event was received
       md5digest varchar(32) NOT NULL default '',   -- md5digest of all the data - helps avoid corruption
       primary key (gid, question, eventnum));
