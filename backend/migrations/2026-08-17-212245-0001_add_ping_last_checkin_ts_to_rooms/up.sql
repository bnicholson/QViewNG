-- Timestamp of the last check-in (ping) received from a room's client. Nullable (blank by default).
ALTER TABLE rooms ADD COLUMN ping_last_checkin_ts TIMESTAMPTZ;
