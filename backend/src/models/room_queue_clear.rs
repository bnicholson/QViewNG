//! "Clear Pending + Resend": an operator-initiated recovery for a room whose QuizMachine client is
//! stuck behind an event the server will not accept.
//!
//! While a clear is `clearing`, the server acknowledges every event the room sends without applying
//! it, setting each aside in the quarantine, so the client's queue drains. Once the room reports
//! nothing pending (or the clearing window runs out), the clear moves to `resending`: the server
//! asks the room to resend every game it set events aside for, and keeps asking until each of
//! those events is back. Each clear is one row, which doubles as the audit trail.

use chrono::{DateTime, Duration, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::database;

pub const STATUS_CLEARING: &str = "clearing";
pub const STATUS_RESENDING: &str = "resending";
pub const STATUS_COMPLETE: &str = "complete";
pub const STATUS_UNCONFIRMED: &str = "unconfirmed";
pub const STATUS_CANCELLED: &str = "cancelled";
pub const STATUS_EXPIRED: &str = "expired";

/// How long the server keeps acknowledging the room's events, drained or not. Bounded so a room
/// that goes offline mid-clear cannot have its events silently discarded when it comes back.
pub const CLEAR_WINDOW: Duration = Duration::minutes(5);
/// How long to wait for resent events before asking the room again.
pub const RESEND_RETRY_AFTER: Duration = Duration::seconds(60);
/// How long after draining to keep asking before giving up as `unconfirmed`.
pub const RESEND_DEADLINE: Duration = Duration::minutes(10);
/// How long a finished clear's outcome stays on the Server Monitor.
pub const SHOW_FINISHED_FOR: Duration = Duration::minutes(15);

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Selectable, Identifiable, ToSchema)]
#[diesel(table_name = crate::schema::room_queue_clears)]
#[diesel(primary_key(room_clear_id))]
pub struct RoomQueueClear {
    pub room_clear_id: i64,
    pub roomid: Uuid,
    pub status: String,
    pub requested_by: Uuid,
    pub requested_at: DateTime<Utc>,
    pub clear_until: DateTime<Utc>,
    pub drained_at: Option<DateTime<Utc>>,
    pub resend_game_id: Option<Uuid>,
    pub resend_last_sent_at: Option<DateTime<Utc>>,
    pub resend_attempts: i32,
    pub finished_at: Option<DateTime<Utc>>,
    pub finished_by: Option<Uuid>,
}

impl RoomQueueClear {
    pub fn is_active(&self) -> bool {
        self.status == STATUS_CLEARING || self.status == STATUS_RESENDING
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::room_queue_clears)]
struct NewRoomQueueClear {
    roomid: Uuid,
    status: String,
    requested_by: Uuid,
    clear_until: DateTime<Utc>,
}

/// Starts a clear. Fails with a unique violation if the room already has one in flight.
pub fn start(db: &mut database::Connection, room_id: Uuid, user_id: Uuid) -> QueryResult<RoomQueueClear> {
    use crate::schema::room_queue_clears::dsl::*;
    diesel::insert_into(room_queue_clears)
        .values(&NewRoomQueueClear {
            roomid: room_id,
            status: STATUS_CLEARING.to_string(),
            requested_by: user_id,
            clear_until: Utc::now() + CLEAR_WINDOW,
        })
        .get_result(db)
}

/// The room's clear that is `clearing` or `resending`, if any.
pub fn active_for_room(db: &mut database::Connection, room_id: Uuid) -> QueryResult<Option<RoomQueueClear>> {
    use crate::schema::room_queue_clears::dsl::*;
    room_queue_clears
        .filter(roomid.eq(room_id))
        .filter(status.eq_any([STATUS_CLEARING, STATUS_RESENDING]))
        .first(db)
        .optional()
}

/// Cheap pre-check for the event ingest path: is any room being cleared right now? Lets every
/// other event skip resolving its room.
pub fn any_clearing_now(db: &mut database::Connection) -> QueryResult<bool> {
    use crate::schema::room_queue_clears::dsl::*;
    diesel::select(diesel::dsl::exists(
        room_queue_clears
            .filter(status.eq(STATUS_CLEARING))
            .filter(clear_until.gt(Utc::now())),
    ))
    .get_result(db)
}

/// The room's clear that is acknowledging events right now — `clearing` and inside its window.
pub fn clearing_now_for_room(db: &mut database::Connection, room_id: Uuid) -> QueryResult<Option<RoomQueueClear>> {
    use crate::schema::room_queue_clears::dsl::*;
    room_queue_clears
        .filter(roomid.eq(room_id))
        .filter(status.eq(STATUS_CLEARING))
        .filter(clear_until.gt(Utc::now()))
        .first(db)
        .optional()
}

/// The clear to show on the Server Monitor: the one in flight, or the last one if it finished
/// recently enough that its outcome is still worth showing.
pub fn latest_to_show_for_room(db: &mut database::Connection, room_id: Uuid) -> QueryResult<Option<RoomQueueClear>> {
    use crate::schema::room_queue_clears::dsl::*;
    let latest: Option<RoomQueueClear> = room_queue_clears
        .filter(roomid.eq(room_id))
        .order(requested_at.desc())
        .first(db)
        .optional()?;
    Ok(latest.filter(|c| {
        c.is_active() || c.finished_at.map_or(false, |f| Utc::now() - f <= SHOW_FINISHED_FOR)
    }))
}

pub fn mark_resending(
    db: &mut database::Connection,
    clear_id: i64,
    drained: DateTime<Utc>,
    current_game: Option<Uuid>,
) -> QueryResult<RoomQueueClear> {
    use crate::schema::room_queue_clears::dsl::*;
    diesel::update(room_queue_clears.find(clear_id))
        .set((status.eq(STATUS_RESENDING), drained_at.eq(drained), resend_game_id.eq(current_game)))
        .get_result(db)
}

pub fn record_resend_sent(db: &mut database::Connection, clear_id: i64, sent: DateTime<Utc>) -> QueryResult<RoomQueueClear> {
    use crate::schema::room_queue_clears::dsl::*;
    diesel::update(room_queue_clears.find(clear_id))
        .set((resend_last_sent_at.eq(sent), resend_attempts.eq(resend_attempts + 1)))
        .get_result(db)
}

pub fn finish(
    db: &mut database::Connection,
    clear_id: i64,
    final_status: &str,
    by: Option<Uuid>,
) -> QueryResult<RoomQueueClear> {
    use crate::schema::room_queue_clears::dsl::*;
    diesel::update(room_queue_clears.find(clear_id))
        .set((status.eq(final_status), finished_at.eq(Utc::now()), finished_by.eq(by)))
        .get_result(db)
}

/// What a ping should do about the room's clear.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClearAction {
    /// Nothing to do on this ping (still clearing, or a resend went out too recently to repeat).
    Wait,
    /// Stop acknowledging and start asking for the affected games back.
    StartResending,
    /// Send (or repeat) the resend commands now.
    SendResend,
    /// End the clear with this status.
    Finish(&'static str),
}

/// Decides the next step for a clear from what the room just reported.
///
/// `cleared_count` is how many events the clear set aside; `all_back` is whether every one that
/// can come back has (see `outstanding` in the ping handler).
pub fn decide(
    clear: &RoomQueueClear,
    jobs_pending: Option<i32>,
    cleared_count: usize,
    all_back: bool,
    now: DateTime<Utc>,
) -> ClearAction {
    match clear.status.as_str() {
        STATUS_CLEARING => {
            let drained = jobs_pending == Some(0);
            let window_over = now >= clear.clear_until;
            if drained {
                // The plan's trigger: the room reports nothing pending, so ask for its games back
                // — even if nothing was set aside, the current game is resent in full.
                ClearAction::StartResending
            } else if window_over {
                // Out of time. Anything already set aside is owed back; otherwise nothing is.
                if cleared_count > 0 { ClearAction::StartResending } else { ClearAction::Finish(STATUS_EXPIRED) }
            } else {
                ClearAction::Wait
            }
        }
        STATUS_RESENDING => {
            if clear.resend_attempts == 0 {
                // Always ask at least once, in the same ping that reported the queue drained.
                ClearAction::SendResend
            } else if all_back {
                ClearAction::Finish(STATUS_COMPLETE)
            } else if clear.drained_at.map_or(false, |d| now >= d + RESEND_DEADLINE) {
                ClearAction::Finish(STATUS_UNCONFIRMED)
            } else if clear.resend_last_sent_at.map_or(true, |s| now >= s + RESEND_RETRY_AFTER) {
                ClearAction::SendResend
            } else {
                ClearAction::Wait
            }
        }
        _ => ClearAction::Wait,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clear(status: &str) -> RoomQueueClear {
        let now = Utc::now();
        RoomQueueClear {
            room_clear_id: 1,
            roomid: Uuid::new_v4(),
            status: status.to_string(),
            requested_by: Uuid::new_v4(),
            requested_at: now,
            clear_until: now + CLEAR_WINDOW,
            drained_at: None,
            resend_game_id: None,
            resend_last_sent_at: None,
            resend_attempts: 0,
            finished_at: None,
            finished_by: None,
        }
    }

    #[test]
    fn clearing_waits_while_events_are_still_pending() {
        assert_eq!(decide(&clear(STATUS_CLEARING), Some(4), 2, false, Utc::now()), ClearAction::Wait);
    }

    #[test]
    fn clearing_starts_resending_once_the_room_reports_nothing_pending() {
        assert_eq!(decide(&clear(STATUS_CLEARING), Some(0), 3, false, Utc::now()), ClearAction::StartResending);
        // Even with nothing set aside: the current game is still resent in full.
        assert_eq!(decide(&clear(STATUS_CLEARING), Some(0), 0, true, Utc::now()), ClearAction::StartResending);
    }

    #[test]
    fn an_expired_window_still_owes_back_what_it_set_aside() {
        let c = clear(STATUS_CLEARING);
        let after = c.clear_until + Duration::seconds(1);
        assert_eq!(decide(&c, Some(5), 2, false, after), ClearAction::StartResending);
        assert_eq!(decide(&c, Some(5), 0, true, after), ClearAction::Finish(STATUS_EXPIRED));
    }

    #[test]
    fn resending_always_asks_once_before_checking_for_completion() {
        let mut c = clear(STATUS_RESENDING);
        c.drained_at = Some(Utc::now());
        assert_eq!(decide(&c, Some(0), 0, true, Utc::now()), ClearAction::SendResend);
    }

    #[test]
    fn resending_completes_when_everything_is_back() {
        let mut c = clear(STATUS_RESENDING);
        c.drained_at = Some(Utc::now());
        c.resend_attempts = 1;
        c.resend_last_sent_at = Some(Utc::now());
        assert_eq!(decide(&c, Some(0), 3, true, Utc::now()), ClearAction::Finish(STATUS_COMPLETE));
    }

    #[test]
    fn resending_repeats_only_after_the_retry_interval_and_gives_up_at_the_deadline() {
        let now = Utc::now();
        let mut c = clear(STATUS_RESENDING);
        c.drained_at = Some(now);
        c.resend_attempts = 1;
        c.resend_last_sent_at = Some(now);

        assert_eq!(decide(&c, Some(0), 3, false, now + Duration::seconds(10)), ClearAction::Wait);
        assert_eq!(decide(&c, Some(0), 3, false, now + RESEND_RETRY_AFTER), ClearAction::SendResend);
        assert_eq!(
            decide(&c, Some(0), 3, false, now + RESEND_DEADLINE),
            ClearAction::Finish(STATUS_UNCONFIRMED),
        );
    }

    #[test]
    fn finished_clears_do_nothing() {
        for s in [STATUS_COMPLETE, STATUS_UNCONFIRMED, STATUS_CANCELLED, STATUS_EXPIRED] {
            assert_eq!(decide(&clear(s), Some(0), 1, false, Utc::now()), ClearAction::Wait);
        }
    }
}
