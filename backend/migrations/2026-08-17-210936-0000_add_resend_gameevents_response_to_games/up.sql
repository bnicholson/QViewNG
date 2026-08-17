-- Response text for a "resend game events" request on a game. Nullable (blank by default).
ALTER TABLE games ADD COLUMN resend_gameevents_response VARCHAR(256);
