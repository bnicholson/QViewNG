
DROP TABLE gameevents;

CREATE TABLE quizevents (
       gid BIGINT NOT NULL,                                 -- unique identifier (Tournament, Division, Room, Round, Id)
       question integer NOT NULL,                           -- Question (part of the key), what question is this event
       eventnum integer NOT NULL,                           -- There can be multiple events per question
       name varchar(64) NOT NULL,                           -- A Quizzer name or team name
       team integer NOT NULL,                               -- team #   (0-2)
       quizzer integer NOT NULL,                            -- quizzer # (0-4)
       event varchar(2) NOT NULL,                           -- event identifier (TC, BE, ...)
       parm1 varchar(8) NOT NULL,                           -- parameter for an event
       parm2 varchar(8) NOT NULL,                           -- parameter #2 for an event
       clientts timestamptz NOT NULL,                       -- timestampe when the client thought this happened
       serverts timestamptz NOT NULL,                       -- server time when this event was received
       md5digest varchar(32) NOT NULL default '',           -- md5digest of all the data - helps avoid corruption
       primary key (gid, question, eventnum));
