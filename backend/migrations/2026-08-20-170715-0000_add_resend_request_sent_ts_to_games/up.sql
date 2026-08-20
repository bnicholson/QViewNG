-- Timestamp when the resend command was actually sent to the client (so it is sent only once). Nullable (blank by default).
ALTER TABLE games ADD COLUMN resend_request_sent_ts TIMESTAMPTZ;
