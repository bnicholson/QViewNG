-- Nullable timestamp recording when a "resend game events" request was made for a game.
-- Defaults to NULL (blank).
ALTER TABLE games ADD COLUMN resend_gameevents_request_ts TIMESTAMPTZ;
