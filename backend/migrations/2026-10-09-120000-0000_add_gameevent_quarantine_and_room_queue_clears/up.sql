-- Keeping a QuizMachine client's event queue moving.
--
-- A client sends its game events in order and retries the one at the head of its queue until the
-- server accepts it, so an event the server rejects every time holds back everything behind it.
-- Three additions let the server get a room unstuck without losing data:
--
--   1. gameevent_quarantine — events the server acknowledged to the client but did not apply,
--      kept with the reason. 'rejected' rows failed a check they would fail on every retry;
--      'cleared' rows were set aside while an operator cleared the room's queue.
--   2. room_queue_clears    — the operator-initiated "Clear Pending + Resend" runs, one per row,
--      which double as the audit trail of who cleared which room and how it ended.
--   3. rooms.ping_pending_stalled_since — when the room's pending count last stopped going down,
--      so the server (not each browser) decides whether a room is stuck.

CREATE TABLE room_queue_clears (
    room_clear_id        BIGSERIAL PRIMARY KEY,
    roomid               UUID NOT NULL REFERENCES rooms(roomid) ON DELETE CASCADE,
    -- clearing   : acknowledging and setting aside every event the room sends
    -- resending  : queue drained; asking the room to resend the affected games until they are whole
    -- complete   : every set-aside event that could come back has come back
    -- unconfirmed: gave up waiting for the resent events
    -- cancelled  : stopped by a person before anything was set aside
    -- expired    : the clearing window passed with nothing set aside
    status               TEXT NOT NULL
        CHECK (status IN ('clearing', 'resending', 'complete', 'unconfirmed', 'cancelled', 'expired')),
    requested_by         UUID NOT NULL REFERENCES users(id),
    requested_at         TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- Stop acknowledging after this whether or not the queue drained, so a room that goes offline
    -- mid-clear cannot have its events silently discarded when it comes back.
    clear_until          TIMESTAMPTZ NOT NULL,
    drained_at           TIMESTAMPTZ,
    -- The game the room was on when its queue drained; it is resent along with the cleared games.
    resend_game_id       UUID,
    resend_last_sent_at  TIMESTAMPTZ,
    resend_attempts      INTEGER NOT NULL DEFAULT 0,
    finished_at          TIMESTAMPTZ,
    finished_by          UUID REFERENCES users(id)
);

-- At most one clear in flight per room.
CREATE UNIQUE INDEX uq_room_queue_clears_active
    ON room_queue_clears (roomid) WHERE status IN ('clearing', 'resending');
CREATE INDEX ix_room_queue_clears_roomid ON room_queue_clears (roomid, requested_at DESC);

-- Strings are TEXT rather than the VARCHAR widths of gameevents/gameeventlogs: an event too long
-- for those columns is exactly the kind of event that lands here, so this table must not reject it.
CREATE TABLE gameevent_quarantine (
    quarantine_id   BIGSERIAL PRIMARY KEY,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    kind            TEXT NOT NULL CHECK (kind IN ('rejected', 'cleared')),
    reason          TEXT NOT NULL,
    roomid          UUID REFERENCES rooms(roomid) ON DELETE SET NULL,
    room_clear_id   BIGINT REFERENCES room_queue_clears(room_clear_id) ON DELETE SET NULL,
    -- As sent, and parsed when it parses. No foreign key: the game may not exist, which is one of
    -- the reasons an event ends up here.
    gid_raw         TEXT NOT NULL DEFAULT '',
    gid             UUID,
    tournament      TEXT NOT NULL DEFAULT '',
    division        TEXT NOT NULL DEFAULT '',
    room            TEXT NOT NULL DEFAULT '',
    round           TEXT NOT NULL DEFAULT '',
    question        INTEGER NOT NULL DEFAULT 0,
    eventnum        INTEGER NOT NULL DEFAULT 0,
    event           TEXT NOT NULL DEFAULT '',
    name            TEXT NOT NULL DEFAULT '',
    team            INTEGER NOT NULL DEFAULT 0,
    quizzer         INTEGER NOT NULL DEFAULT 0,
    parm1           TEXT NOT NULL DEFAULT '',
    parm2           TEXT NOT NULL DEFAULT '',
    clientkey       TEXT NOT NULL DEFAULT '',
    -- The whole request, so an event can be inspected or replayed exactly as the client sent it.
    query_string    TEXT NOT NULL DEFAULT ''
);

CREATE INDEX ix_gameevent_quarantine_roomid ON gameevent_quarantine (roomid, created_at DESC);
CREATE INDEX ix_gameevent_quarantine_clear ON gameevent_quarantine (room_clear_id);

ALTER TABLE rooms ADD COLUMN ping_pending_stalled_since TIMESTAMPTZ;
