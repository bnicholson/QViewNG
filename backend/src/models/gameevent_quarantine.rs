//! Game events the server acknowledged to a QuizMachine client but did not apply.
//!
//! A client sends its events in order and retries the one at the head of its queue until the
//! server accepts it, so an event the server would reject on every attempt holds back everything
//! queued behind it. Rather than reject such an event, the server acknowledges it — letting the
//! queue move — and keeps it here with the reason, where an operator can see it from the Server
//! Monitor. Nothing that reaches this table is lost.

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::database;

/// Failed a check it would fail on every retry (unknown game, a value too long, …).
pub const KIND_REJECTED: &str = "rejected";
/// Set aside while an operator cleared the room's pending queue.
pub const KIND_CLEARED: &str = "cleared";

/// The most rows the Server Monitor's dialog asks for in one go.
pub const READ_LIMIT: i64 = 500;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Selectable, Identifiable, ToSchema)]
#[diesel(table_name = crate::schema::gameevent_quarantine)]
#[diesel(primary_key(quarantine_id))]
pub struct QuarantinedGameEvent {
    pub quarantine_id: i64,
    pub created_at: DateTime<Utc>,
    pub kind: String,
    pub reason: String,
    pub roomid: Option<Uuid>,
    pub room_clear_id: Option<i64>,
    pub gid_raw: String,             // as the client sent it
    pub gid: Option<Uuid>,           // parsed, when it parses
    pub tournament: String,
    pub division: String,
    pub room: String,
    pub round: String,
    pub question: i32,
    pub eventnum: i32,
    pub event: String,
    pub name: String,
    pub team: i32,
    pub quizzer: i32,
    pub parm1: String,
    pub parm2: String,
    pub clientkey: String,
    pub query_string: String,        // the whole request, for inspection or exact replay
}

#[derive(Insertable, Debug, Clone, Default)]
#[diesel(table_name = crate::schema::gameevent_quarantine)]
pub struct NewQuarantinedGameEvent {
    pub kind: String,
    pub reason: String,
    pub roomid: Option<Uuid>,
    pub room_clear_id: Option<i64>,
    pub gid_raw: String,
    pub gid: Option<Uuid>,
    pub tournament: String,
    pub division: String,
    pub room: String,
    pub round: String,
    pub question: i32,
    pub eventnum: i32,
    pub event: String,
    pub name: String,
    pub team: i32,
    pub quizzer: i32,
    pub parm1: String,
    pub parm2: String,
    pub clientkey: String,
    pub query_string: String,
}

pub fn create(db: &mut database::Connection, item: &NewQuarantinedGameEvent) -> QueryResult<QuarantinedGameEvent> {
    use crate::schema::gameevent_quarantine::dsl::*;
    diesel::insert_into(gameevent_quarantine).values(item).get_result(db)
}

/// A room's quarantined events, newest first.
pub fn read_all_of_room(db: &mut database::Connection, room_id: Uuid) -> QueryResult<Vec<QuarantinedGameEvent>> {
    use crate::schema::gameevent_quarantine::dsl::*;
    gameevent_quarantine
        .filter(roomid.eq(room_id))
        .order((created_at.desc(), quarantine_id.desc()))
        .limit(READ_LIMIT)
        .load(db)
}

pub fn count_of_room(db: &mut database::Connection, room_id: Uuid) -> QueryResult<i64> {
    use crate::schema::gameevent_quarantine::dsl::*;
    gameevent_quarantine.filter(roomid.eq(room_id)).count().get_result(db)
}

/// The events a particular clear set aside.
pub fn read_cleared_of_clear(db: &mut database::Connection, clear_id: i64) -> QueryResult<Vec<QuarantinedGameEvent>> {
    use crate::schema::gameevent_quarantine::dsl::*;
    gameevent_quarantine
        .filter(room_clear_id.eq(clear_id))
        .filter(kind.eq(KIND_CLEARED))
        .order(quarantine_id.asc())
        .load(db)
}

/// Whether the event `(gid, question, eventnum)` was quarantined as rejected at or after `since`.
///
/// After a clear, a resent event that the server rejects again lands back here instead of in
/// `gameevents`; it has still been accounted for — it came back, and was set aside with a reason.
pub fn rejected_since(
    db: &mut database::Connection,
    game_id: Uuid,
    question_num: i32,
    event_num: i32,
    since: DateTime<Utc>,
) -> QueryResult<bool> {
    use crate::schema::gameevent_quarantine::dsl::*;
    diesel::select(diesel::dsl::exists(
        gameevent_quarantine
            .filter(gid.eq(game_id))
            .filter(question.eq(question_num))
            .filter(eventnum.eq(event_num))
            .filter(kind.eq(KIND_REJECTED))
            .filter(created_at.ge(since)),
    ))
    .get_result(db)
}
