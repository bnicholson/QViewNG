-- Latest host/IP reported by a room's QuizMachine client. Nullable (blank by default).
ALTER TABLE rooms ADD COLUMN ping_host_ip VARCHAR(32);
