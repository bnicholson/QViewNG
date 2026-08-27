
use crate::database;
use crate::models::common::PaginationParams;
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult,AsChangeset,Insertable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use chrono::{Utc,DateTime};

pub struct RoomBuilder {
    name: Option<String>,                           // Name of the room (human readable)
    building: Option<String>,                       // What is the building this room is in
    comments: Option<String>,                       // Any comments about the room,
    tid: Uuid,                                      // id of the associated tournament
    clientkey: Option<String>,
    quizmaster_id: Option<Uuid>,
    contentjudge_id: Option<Uuid>,
}

impl RoomBuilder {
    pub fn new(room_name: &str, tid: Uuid) -> Self {
        Self {
            name: Some(room_name.to_string()),
            building: None,
            comments: None,
            tid: tid,
            clientkey: None,
            quizmaster_id: None,
            contentjudge_id: None,
        }
    }
    pub fn new_default(room_name: &str, tid: Uuid) -> Self {
        Self {
            name: Some(room_name.to_string()),
            building: Some("Building 451".to_string()),
            comments: Some("None at this time.".to_string()),
            tid: tid,
            clientkey: Some("".to_string()),
            quizmaster_id: None,
            contentjudge_id: None,
        }
    }
    pub fn set_name(mut self, room_name: String) -> Self {
        self.name = Some(room_name);
        self
    }
    pub fn set_building(mut self, building: String) -> Self {
        self.building = Some(building);
        self
    }
    pub fn set_comments(mut self, comments: String) -> Self {
        self.comments = Some(comments);
        self
    }
    pub fn set_tid(mut self, tid: Uuid) -> Self {
        self.tid = tid;
        self
    }
    pub fn set_clientkey(mut self, clientkey: Option<String>) -> Self {
        self.clientkey = clientkey;
        self
    }
    pub fn set_quizmaster_id(mut self, quizmaster_id: Option<Uuid>) -> Self {
        self.quizmaster_id = quizmaster_id;
        self
    }
    pub fn set_contentjudge_id(mut self, contentjudge_id: Option<Uuid>) -> Self {
        self.contentjudge_id = contentjudge_id;
        self
    }
    fn validate_all_are_some(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if self.name.is_none() {
            errors.push("name is required".to_string());
        }
        if self.building.is_none() {
            errors.push("building is required".to_string());
        }
        if self.comments.is_none() {
            errors.push("comments is required".to_string());
        }
        if self.clientkey.is_none() {
            errors.push("clientkey is required".to_string());
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(())
    }
    pub fn build(self) -> Result<NewRoom, Vec<String>> {
        match self.validate_all_are_some() {
            Err(e) => {
                Err(e)
            },
            Ok(_) => {
                Ok(
                    NewRoom {
                        name: self.name.unwrap(),            // Name of the room (human readable)
                        building: self.building.unwrap(),    // What is the building this room is in
                        comments: self.comments.unwrap(),    // Any comments about the room,
                        tid: self.tid,                       // id of the associated tournament
                        clientkey: self.clientkey.unwrap(),
                        quizmaster_id: self.quizmaster_id,
                        contentjudge_id: self.contentjudge_id,
                    }
                )
            }
        }
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<Room> {
        let new_room = self.build();
        create(db, &new_room.unwrap())
    }
}

// #[tsync::tsync]
#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Identifiable,
    ToSchema
)]
#[diesel(table_name = crate::schema::rooms)]
#[diesel(primary_key(roomid))]
pub struct Room {
    pub roomid: Uuid,                           // identifies the room uniquely
    pub name: String,                           // Name of the room (human readable)
    pub building: String,                       // What is the building this room is in
    pub comments: String,                       // Any comments about the room,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tid: Uuid,                              // id of the associated tournament
    pub clientkey: String,                      // registration key from the QuizMachine client, used here for ID
    pub quizmaster_id: Option<Uuid>,            // optional quizmaster assigned to this room
    pub contentjudge_id: Option<Uuid>,          // optional content judge assigned to this room
    pub ping_question_number: Option<i32>,      // latest question number reported by the room's client
    pub ping_qm_version: Option<String>,        // latest QuizMachine version reported by the room's client
    pub ping_client_ts: Option<DateTime<Utc>>,  // latest client timestamp reported by the room's client
    pub ping_jobspending: Option<i32>,          // latest jobs-pending count reported by the room's client
    pub ping_room: Option<String>,              // latest room name reported by the room's client
    pub ping_round: Option<String>,             // latest round name reported by the room's client
    pub ping_game_id: Option<Uuid>,             // optional game the room's client last pinged about
    pub ping_host_ip: Option<String>,           // latest host/IP reported by the room's client
    pub ping_last_checkin_ts: Option<DateTime<Utc>>, // timestamp of the last check-in (ping) from the room's client
}

#[derive(
    Insertable,
    Serialize,
    Deserialize,
    Debug
)]
#[diesel(table_name = crate::schema::rooms)]
pub struct NewRoom {
    pub name: String,                           // Name of the room (human readable)
    pub building: String,                       // What is the building this room is in
    pub comments: String,                       // Any comments about the room,
    pub tid: Uuid,                              // id of the associated tournament
    pub clientkey: String,
    pub quizmaster_id: Option<Uuid>,            // optional quizmaster assigned to this room
    pub contentjudge_id: Option<Uuid>,          // optional content judge assigned to this room
}

// #[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::rooms)]
#[diesel(primary_key(roomid))]
pub struct RoomChangeset {
    pub name: Option<String>,                   // Name of the room (human readable)
    pub building: Option<String>,               // What is the building this room is in
    pub comments: Option<String>,               // Any comments about the room
    pub clientkey: Option<String>,
    pub quizmaster_id: Option<Uuid>,            // optional quizmaster assigned to this room
    pub contentjudge_id: Option<Uuid>,          // optional content judge assigned to this room
    pub ping_question_number: Option<i32>,      // latest question number reported by the room's client
    pub ping_qm_version: Option<String>,        // latest QuizMachine version reported by the room's client
    pub ping_client_ts: Option<DateTime<Utc>>,  // latest client timestamp reported by the room's client
    pub ping_jobspending: Option<i32>,          // latest jobs-pending count reported by the room's client
    pub ping_room: Option<String>,              // latest room name reported by the room's client
    pub ping_round: Option<String>,             // latest round name reported by the room's client
    pub ping_game_id: Option<Uuid>,             // optional game the room's client last pinged about
    pub ping_host_ip: Option<String>,           // latest host/IP reported by the room's client
    pub ping_last_checkin_ts: Option<DateTime<Utc>>, // timestamp of the last check-in (ping) from the room's client
}

impl RoomChangeset {
    pub fn empty() -> Self {
        Self {
            name: None,
            building: None,
            comments: None,
            clientkey: None,
            quizmaster_id: None,
            contentjudge_id: None,
            ping_question_number: None,
            ping_qm_version: None,
            ping_client_ts: None,
            ping_jobspending: None,
            ping_room: None,
            ping_round: None,
            ping_game_id: None,
            ping_host_ip: None,
            ping_last_checkin_ts: None,
        }
    }
}

pub fn create(db: &mut database::Connection, item: &NewRoom) -> QueryResult<Room> {
    use crate::schema::rooms::dsl::*;
    insert_into(rooms).values(item).get_result::<Room>(db)
}

pub fn exists(db: &mut database::Connection, roomid: Uuid) -> bool {
    use crate::schema::rooms::dsl::rooms;
    rooms
        .find(roomid)
        .get_result::<Room>(db)
        .is_ok()
}

pub fn read(db: &mut database::Connection, item_id: Uuid) -> QueryResult<Room> {
    use crate::schema::rooms::dsl::*;
    rooms.filter(roomid.eq(item_id)).first::<Room>(db)
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<Room>> {
    use crate::schema::rooms::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    rooms
        .order(created_at)
        .limit(page_size)
        .offset(offset_val)
        .load::<Room>(db)
}

pub fn read_all_rooms_of_tournament(
    db: &mut database::Connection,
    item_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<Vec<Room>> {
    use crate::schema::rooms::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    rooms
        .filter(tid.eq(item_id))
        .order(name.asc())
        .limit(page_size)
        .offset(offset_val)
        .load::<Room>(db)
}

pub fn update(db: &mut database::Connection, item_id: Uuid, item: &RoomChangeset) -> QueryResult<Room> {
    use crate::schema::rooms::dsl::*;
    diesel::update(rooms.filter(roomid.eq(item_id)))
        .set((
            item,
            updated_at.eq(diesel::dsl::now),
        ))
        .get_result(db)
}

pub fn find_by_name_in_tournament(db: &mut database::Connection, room_name: &str, tournament_id: Uuid) -> QueryResult<Room> {
    use crate::schema::rooms::dsl::*;
    rooms
        .filter(name.eq(room_name).and(tid.eq(tournament_id)))
        .first::<Room>(db)
}

pub fn count(db: &mut database::Connection) -> QueryResult<i64> {
    use crate::schema::rooms::dsl::*;
    rooms.count().get_result(db)
}

pub fn delete(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::rooms::dsl::*;
    diesel::delete(rooms.filter(roomid.eq(item_id))).execute(db)
}

// ─── Room Monitor (per-room live ping + resend status for a tournament) ────────

/// One row of the Room Monitor: a room's latest ping data plus the referenced
/// game's resend status. `status_error` is intentionally left out for now.
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct RoomMonitorRow {
    pub roomid: Uuid,
    pub check_in: Option<DateTime<Utc>>,   // ping_last_checkin_ts (server receive time)
    pub client_ts: Option<DateTime<Utc>>,  // ping_client_ts (client-reported time; used for the "late" check)
    pub room: Option<String>,              // ping_room
    pub round: Option<String>,             // ping_round
    pub question: Option<i32>,             // ping_question_number
    pub host_ip: Option<String>,           // ping_host_ip
    pub qm_version: Option<String>,        // ping_qm_version
    pub pending: Option<i32>,              // ping_jobspending
    pub resend: Option<String>,            // referenced game's resend_gameevents_response
    pub game_id: Option<Uuid>,             // referenced game (target of a resend request), if any
    pub game_in_progress: bool,            // referenced game has started (gameplay) but is not finished
    pub data_incomplete: bool,             // retrieved events have a gap in questions or sub-events
}

// Event codes that make up round initialization (no gameplay yet).
const ROOM_MONITOR_INIT_CODES: [&str; 8] = ["RM", "QT", "IP", "OP", "TN", "QN", "SC", "SS"];

/// Finds the game referenced by a room's latest ping: by ping_game_id if present,
/// otherwise by the composite key (this room + a round matching ping_round).
fn referenced_game(db: &mut database::Connection, room: &Room) -> Option<crate::models::game::Game> {
    if let Some(gid) = room.ping_game_id {
        if let Ok(game) = crate::models::game::read(db, gid) {
            return Some(game);
        }
    }
    // Composite fallback: a game in this room whose round name matches ping_round.
    let round_name = room.ping_round.as_ref()?;
    use crate::schema::games::dsl as g;
    use crate::schema::rounds::dsl as r;
    g::games
        .inner_join(r::rounds.on(g::roundid.eq(r::roundid)))
        .filter(g::roomid.eq(room.roomid))
        .filter(r::name.eq(round_name))
        .select(crate::models::game::Game::as_select())
        .first::<crate::models::game::Game>(db)
        .ok()
}

/// Builds the Room Monitor rows for every room in a tournament.
pub fn read_room_monitor_of_tournament(db: &mut database::Connection, tournament_id: Uuid) -> QueryResult<Vec<RoomMonitorRow>> {
    let pagination = PaginationParams { page: 0, page_size: PaginationParams::MAX_PAGE_SIZE as i64 };
    let all_rooms = read_all_rooms_of_tournament(db, tournament_id, &pagination)?;

    let mut monitor = Vec::with_capacity(all_rooms.len());
    for room in all_rooms {
        let game = referenced_game(db, &room);
        let resend = game.as_ref().and_then(|g| g.resend_gameevents_response.clone());

        // Started = at least one non-initialization (gameplay) event.
        // Finished = reached question 20 with all ties resolved (distinct final scores),
        // matching the Game Selection "Done" definition.
        // data_incomplete = the retrieved events have a gap in question numbers or in a
        // question's sub-event (eventnum) sequence.
        let (game_in_progress, data_incomplete) = match &game {
            Some(g) => {
                let events = crate::models::gameevent::read_all_gameevents_of_game(db, g.gid, &pagination)
                    .unwrap_or_default();

                // Validate sequential integrity of the retrieved events.
                let data_incomplete = crate::models::gameevent::events_have_gaps(&events);

                let has_events = !events.is_empty();
                let started = events.iter().any(|e| !ROOM_MONITOR_INIT_CODES.contains(&e.event.as_str()));
                let has_question_20 = events.iter().any(|e| e.question >= 20);

                let results = crate::models::gameevent::calculate_team_results_for_game(g.gid, events);
                let ties_resolved = match &results {
                    Ok(teams) if !teams.is_empty() => {
                        let mut scores: Vec<i32> = teams.iter().map(|t| t.score).collect();
                        scores.sort_unstable();
                        scores.windows(2).all(|w| w[0] != w[1])
                    }
                    _ => false,
                };
                let finished = has_events && results.is_ok() && has_question_20 && ties_resolved;
                (started && !finished, data_incomplete)
            }
            None => (false, false),
        };

        monitor.push(RoomMonitorRow {
            roomid: room.roomid,
            check_in: room.ping_last_checkin_ts,
            client_ts: room.ping_client_ts,
            room: room.ping_room.clone(),
            round: room.ping_round.clone(),
            question: room.ping_question_number,
            host_ip: room.ping_host_ip.clone(),
            qm_version: room.ping_qm_version.clone(),
            pending: room.ping_jobspending,
            resend,
            game_id: game.as_ref().map(|g| g.gid),
            game_in_progress,
            data_incomplete,
        });

        // While a room is currently reporting (recent check-in), also surface any of its OTHER
        // games whose data has gaps — even though the ping is about a different (current) game —
        // so a resend can be issued for them. Once such a game's data is whole again it no longer
        // has gaps and drops off the monitor on the next poll.
        let reporting = room
            .ping_last_checkin_ts
            .map(|c| Utc::now() - c <= chrono::Duration::minutes(2))
            .unwrap_or(false);
        if reporting {
            let current_gid = game.as_ref().map(|g| g.gid);
            let room_games =
                crate::models::game::read_all_games_of_room(db, room.roomid, &pagination).unwrap_or_default();
            for g in room_games {
                if Some(g.gid) == current_gid {
                    continue; // the current game already has its row above
                }
                let events = crate::models::gameevent::read_all_gameevents_of_game(db, g.gid, &pagination)
                    .unwrap_or_default();
                if events.is_empty() || !crate::models::gameevent::events_have_gaps(&events) {
                    continue; // only surface games that have recorded events with gaps
                }
                let max_question = events.iter().map(|e| e.question).max();
                let round_name = crate::models::round::read(db, g.roundid).ok().map(|r| r.name);
                monitor.push(RoomMonitorRow {
                    roomid: room.roomid,
                    check_in: None,                  // only the current (ping) game shows a check-in
                    client_ts: room.ping_client_ts,
                    room: room.ping_room.clone(),
                    round: round_name,               // this game's own round, not the current ping's
                    question: max_question,          // highest recorded question for this game
                    host_ip: room.ping_host_ip.clone(),
                    qm_version: room.ping_qm_version.clone(),
                    pending: room.ping_jobspending,
                    resend: g.resend_gameevents_response.clone(),
                    game_id: Some(g.gid),
                    game_in_progress: false,         // an extra (non-current) game isn't the in-progress one
                    data_incomplete: true,           // it has gaps — that's why it's surfaced
                });
            }
        }
    }
    Ok(monitor)
}
