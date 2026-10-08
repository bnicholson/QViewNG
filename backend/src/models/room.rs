
use crate::cache::Cache;
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
    last_modified_user: Option<Uuid>,
    roomgroupid: Option<Uuid>,
    supports_threeteam_games: bool,
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
            last_modified_user: None,
            roomgroupid: None,
            supports_threeteam_games: false,
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
            last_modified_user: None,
            roomgroupid: None,
            supports_threeteam_games: false,
        }
    }
    pub fn set_supports_threeteam_games(mut self, v: bool) -> Self {
        self.supports_threeteam_games = v;
        self
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
    pub fn set_last_modified_user(mut self, user_id: Uuid) -> Self {
        self.last_modified_user = Some(user_id);
        self
    }
    pub fn set_roomgroupid(mut self, roomgroupid: Option<Uuid>) -> Self {
        self.roomgroupid = roomgroupid;
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
                        last_modified_user: self.last_modified_user.unwrap_or_else(Uuid::nil),
                        roomgroupid: self.roomgroupid,
                        supports_threeteam_games: self.supports_threeteam_games,
                    }
                )
            }
        }
    }
    pub fn build_and_insert(mut self, db: &mut database::Connection) -> QueryResult<Room> {
        // For seed/test convenience: if no modifier was set, attribute it to the tournament owner.
        if self.last_modified_user.is_none() {
            if let Ok(tournament) = crate::models::tournament::read(db, self.tid) {
                self.last_modified_user = Some(tournament.owner_id);
            }
        }
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
    pub last_modified_user: Uuid,
    /// Soft-delete flag. When true the room is treated as deleted and excluded from reads.
    pub del_fl: bool,
    /// Optional roomgroup (e.g. building) this room belongs to.
    pub roomgroupid: Option<Uuid>,
    /// Whether this room may host 3-team games (games with a center team).
    pub supports_threeteam_games: bool,
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
    #[serde(default)]
    pub last_modified_user: Uuid,
    #[serde(default)]
    pub roomgroupid: Option<Uuid>,
    #[serde(default)]
    pub supports_threeteam_games: bool,
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
    pub supports_threeteam_games: Option<bool>, // whether this room may host 3-team games
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
            supports_threeteam_games: None,
        }
    }
}

pub fn create(db: &mut database::Connection, item: &NewRoom) -> QueryResult<Room> {
    use crate::schema::rooms::dsl::*;
    insert_into(rooms).values(item).get_result::<Room>(db)
}

pub fn exists(db: &mut database::Connection, roomid_val: Uuid) -> bool {
    use crate::schema::rooms::dsl::*;
    rooms
        .find(roomid_val)
        .filter(del_fl.eq(false))
        .get_result::<Room>(db)
        .is_ok()
}

pub fn read(db: &mut database::Connection, item_id: Uuid) -> QueryResult<Room> {
    use crate::schema::rooms::dsl::*;
    rooms.filter(roomid.eq(item_id)).filter(del_fl.eq(false)).first::<Room>(db)
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<Room>> {
    use crate::schema::rooms::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    rooms
        .filter(del_fl.eq(false))
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
        .filter(del_fl.eq(false))
        .order(name.asc())
        .limit(page_size)
        .offset(offset_val)
        .load::<Room>(db)
}

/// One fully-formed row of the rooms data table: the room plus the display name of the user who
/// last modified it. Populated in a single API call.
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, utoipa::ToSchema)]
pub struct RoomRow {
    pub roomid: Uuid,
    pub name: String,
    pub building: String,
    pub comments: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Utc>,
    pub last_modified_user_name: String,
    pub last_modified_user_id: Uuid,
}

/// Returns one page of room-table rows for the tournament (ordered by name) plus the total count.
pub fn read_room_rows_of_tournament(
    db: &mut database::Connection,
    tournament_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<(Vec<RoomRow>, i64)> {
    let total: i64 = {
        use crate::schema::rooms::dsl::*;
        rooms.filter(tid.eq(tournament_id)).filter(del_fl.eq(false)).count().get_result(db)?
    };
    let list = read_all_rooms_of_tournament(db, tournament_id, pagination)?;
    Ok((enrich_room_rows(db, list)?, total))
}

/// One page of rooms belonging to a roomgroup (e.g. building), ordered by name.
pub fn read_all_rooms_of_roomgroup(
    db: &mut database::Connection,
    roomgroup_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<Vec<Room>> {
    use crate::schema::rooms::dsl::*;
    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;
    rooms
        .filter(roomgroupid.eq(roomgroup_id))
        .filter(del_fl.eq(false))
        .order(name.asc())
        .limit(page_size)
        .offset(offset_val)
        .load::<Room>(db)
}

/// Returns one page of enriched room-table rows for a roomgroup (building) plus the total count.
pub fn read_room_rows_of_roomgroup(
    db: &mut database::Connection,
    roomgroup_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<(Vec<RoomRow>, i64)> {
    let total: i64 = {
        use crate::schema::rooms::dsl::*;
        rooms.filter(roomgroupid.eq(roomgroup_id)).filter(del_fl.eq(false)).count().get_result(db)?
    };
    let list = read_all_rooms_of_roomgroup(db, roomgroup_id, pagination)?;
    Ok((enrich_room_rows(db, list)?, total))
}

/// Attaches the last-modifier's display name to a page of rooms.
fn enrich_room_rows(db: &mut database::Connection, list: Vec<Room>) -> QueryResult<Vec<RoomRow>> {
    let name_ids: Vec<Uuid> = list.iter().map(|r| r.last_modified_user).collect();
    let name_by_id = crate::models::user::read_display_names(db, &name_ids)?;
    Ok(list
        .into_iter()
        .map(|r| RoomRow {
            last_modified_user_name: name_by_id
                .get(&r.last_modified_user)
                .cloned()
                .unwrap_or_else(|| r.last_modified_user.to_string()),
            last_modified_user_id: r.last_modified_user,
            roomid: r.roomid,
            name: r.name,
            building: r.building,
            comments: r.comments,
            created_at: r.created_at,
            updated_at: r.updated_at,
        })
        .collect())
}

pub fn update(db: &mut database::Connection, item_id: Uuid, item: &RoomChangeset, modified_by: Uuid) -> QueryResult<Room> {
    use crate::schema::rooms::dsl::*;
    diesel::update(rooms.filter(roomid.eq(item_id)))
        .set((
            item,
            updated_at.eq(diesel::dsl::now),
            last_modified_user.eq(modified_by),
        ))
        .get_result(db)
}

pub fn find_by_name_in_tournament(db: &mut database::Connection, room_name: &str, tournament_id: Uuid) -> QueryResult<Room> {
    use crate::schema::rooms::dsl::*;
    rooms
        .filter(name.eq(room_name).and(tid.eq(tournament_id)))
        .filter(del_fl.eq(false))
        .first::<Room>(db)
}

pub fn count(db: &mut database::Connection) -> QueryResult<i64> {
    use crate::schema::rooms::dsl::*;
    rooms.filter(del_fl.eq(false)).count().get_result(db)
}

/// Soft delete: mark the room deleted (excluded from reads) without removing the row.
pub fn delete(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::rooms::dsl::*;
    diesel::update(rooms.filter(roomid.eq(item_id)))
        .set(del_fl.eq(true))
        .execute(db)
}

/// Purge: permanently remove the room row from the database.
pub fn purge(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
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
    pub resend_sent: bool,                 // a resend command for this game has been picked up by the room (resend_request_sent_ts set)
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
            resend_sent: game.as_ref().map_or(false, |g| g.resend_request_sent_ts.is_some()),
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
                    resend_sent: g.resend_request_sent_ts.is_some(),
                });
            }
        }
    }
    Ok(monitor)
}

// ─── Games in Progress (public tickertape) ───────────────────────────────────

/// A room is dropped from the tickertape once its last check-in is older than this.
///
/// The QuizMachine clients ping every few seconds while a round is running, so a room that
/// has been quiet this long has packed up. Without a window the view would accumulate every
/// room that ever pinged during the tournament.
pub const GAMES_IN_PROGRESS_STALE_AFTER: chrono::Duration = chrono::Duration::minutes(30);

/// One team's live standing within a round in progress.
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct GamesInProgressTeam {
    pub name: String,   // team name, from the game's TN event
    pub score: i32,     // live score as of the events recorded so far
}

/// One actively-reporting room, with the live score of the game it is running.
///
/// A `Vec` of teams rather than team1/score1/team2/score2/team3/score3 so that 2-team and
/// 3-team rounds are both represented naturally.
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct GamesInProgressRow {
    pub roomid: Uuid,
    pub tournament: String,
    pub division: Option<String>,
    pub room: Option<String>,                      // ping_room (what the client calls it)
    pub round: Option<String>,                     // ping_round
    pub question: Option<i32>,                     // ping_question_number
    #[schema(value_type = Option<String>, format = DateTime)]
    pub check_in: Option<DateTime<Utc>>,           // ping_last_checkin_ts
    pub teams: Vec<GamesInProgressTeam>,          // empty when the game has no scoreable events yet
}

/// Builds the public "Games in Progress" tickertape for a tournament.
///
/// Reads the ping columns the pingmsg endpoint already maintains on `rooms` -- Postgres is the
/// source of truth for where each room is -- and joins each row to the live score of the game
/// that room is running, taken from the Valkey-cached `CalculatedGame`.
///
/// Cost on a warm cache: one query for the tournament, one for its rooms, one Valkey GET per
/// reporting room, and no gameevents reads at all. A room whose game is not yet cached pays
/// one event read plus one replay, and that result is then shared with the stats endpoints,
/// which key off the same per-game entry.
pub async fn read_games_in_progress_of_tournament(
    cache: &Cache,
    db: &mut database::Connection,
    tournament_id: Uuid,
) -> QueryResult<Vec<GamesInProgressRow>> {
    let tournament_name = crate::models::tournament::read(db, tournament_id)?.tname;

    let pagination = PaginationParams { page: 0, page_size: PaginationParams::MAX_PAGE_SIZE as i64 };
    let all_rooms = read_all_rooms_of_tournament(db, tournament_id, &pagination)?;

    let cutoff = Utc::now() - GAMES_IN_PROGRESS_STALE_AFTER;

    let mut rows = Vec::new();
    for room in all_rooms {
        // Only rooms that are currently reporting belong on the tickertape.
        match room.ping_last_checkin_ts {
            Some(ts) if ts >= cutoff => {}
            _ => continue,
        }

        // The game the room last reported against, if any.
        let game = room
            .ping_game_id
            .and_then(|gid| crate::models::game::read(db, gid).ok());

        let division = game
            .as_ref()
            .and_then(|g| crate::models::game::read_division_of_game(db, g).ok())
            .map(|d| d.dname);

        // Live scores from the cached calculation. A game with no scoreable events yet (or
        // one whose stream is mid-write) simply yields no teams rather than dropping the row:
        // "Jester 3 is on question 7" is still useful to a viewer.
        let teams = match &game {
            Some(g) => {
                match crate::models::gameevent::calculate_game_cached(cache, db, g.gid).await? {
                    Some(calculated) => {
                        let mut results =
                            crate::models::gameevent::team_results_from_calculated(&calculated);
                        // Left-to-right as the room sees them, not by rank.
                        results.sort_by_key(|r| r.team);
                        results
                            .into_iter()
                            .filter(|r| !r.name.trim().is_empty())
                            .map(|r| GamesInProgressTeam { name: r.name, score: r.score })
                            .collect()
                    }
                    None => Vec::new(),
                }
            }
            None => Vec::new(),
        };

        rows.push(GamesInProgressRow {
            roomid: room.roomid,
            tournament: tournament_name.clone(),
            division,
            room: room.ping_room.clone(),
            round: room.ping_round.clone(),
            question: room.ping_question_number,
            check_in: room.ping_last_checkin_ts,
            teams,
        });
    }

    // Stable, human-sensible order: division, then room name.
    rows.sort_by(|a, b| {
        a.division.cmp(&b.division)
            .then(a.room.cmp(&b.room))
            .then(a.roomid.cmp(&b.roomid))
    });

    Ok(rows)
}

#[cfg(test)]
mod games_in_progress_tests {
    use super::*;

    /// Pins the JSON contract with the frontend's `GamesInProgressRow` interface in
    /// frontend/src/containers/GamesInProgress.tsx. A silent rename here would leave the
    /// tickertape rendering blank cells rather than failing loudly, so the field names are
    /// asserted explicitly.
    #[test]
    fn row_json_matches_the_frontend_interface() {
        let row = GamesInProgressRow {
            roomid: Uuid::new_v4(),
            tournament: "Q2026".to_string(),
            division: Some("District Novice".to_string()),
            room: Some("Jester 3".to_string()),
            round: Some("Tue-07b".to_string()),
            question: Some(7),
            check_in: Some(Utc::now()),
            teams: vec![
                GamesInProgressTeam { name: "Red Team".to_string(), score: 120 },
                GamesInProgressTeam { name: "Blue Team".to_string(), score: 80 },
            ],
        };

        let value: serde_json::Value = serde_json::to_value(&row).unwrap();
        let object = value.as_object().expect("a row serializes to a JSON object");

        for field in [
            "roomid", "tournament", "division", "room", "round", "question", "check_in", "teams",
        ] {
            assert![object.contains_key(field), "the frontend expects a '{field}' field"];
        }
        // Nothing extra: an unexpected field means the TS interface has drifted.
        assert_eq![object.len(), 8, "unexpected fields: {:?}", object.keys().collect::<Vec<_>>()];

        let teams = object["teams"].as_array().unwrap();
        assert_eq![teams.len(), 2];
        assert_eq![teams[0]["name"], "Red Team"];
        assert_eq![teams[0]["score"], 120];

        // An absent division/room/round must serialize as null, which the frontend renders
        // as an em dash -- not be omitted, which would read as undefined.
        let sparse = GamesInProgressRow {
            roomid: Uuid::new_v4(),
            tournament: "Q2026".to_string(),
            division: None,
            room: None,
            round: None,
            question: None,
            check_in: None,
            teams: vec![],
        };
        let sparse_value = serde_json::to_value(&sparse).unwrap();
        for field in ["division", "room", "round", "question", "check_in"] {
            assert![sparse_value[field].is_null(), "'{field}' should serialize as null"];
        }
        assert![sparse_value["teams"].as_array().unwrap().is_empty()];
    }

    /// The staleness window is what makes this "games in progress" rather than "every room
    /// that ever pinged", so its value is worth pinning.
    #[test]
    fn the_staleness_window_is_thirty_minutes() {
        assert_eq![GAMES_IN_PROGRESS_STALE_AFTER.num_minutes(), 30];
    }
}
