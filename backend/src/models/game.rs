use chrono::{DateTime, NaiveDate, Utc};
use diesel::upsert::on_constraint;
use std::collections::HashMap;
use std::cmp::Ordering;
use diesel::{AsChangeset,Insertable,Identifiable,Queryable};
use diesel::prelude::*;
use diesel::insert_into;
use uuid::Uuid;
use crate::models::game_statsgroup::GameStatsGroup;
use crate::{database, models};
use crate::models::common::PaginationParams;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub struct GameBuilder {
    org: Option<String>,
    tournamentid: Option<Uuid>,
    divisionid: Option<Uuid>,
    roomid: Uuid,
    roundid: Uuid,
    ignore: Option<bool>,
    ruleset: Option<String>,
    leftteamid: Option<Uuid>,
    centerteamid: Option<Uuid>,
    rightteamid: Option<Uuid>,
    quizmasterid: Option<Uuid>,
    contentjudgeid: Option<Uuid>,
    clientkey: Option<String>,
    poolbracket_id: Uuid,
    last_modified_user: Option<Uuid>,
    creator_id: Option<Uuid>
}

impl GameBuilder {
    pub fn new(room_id: Uuid, round_id: Uuid) -> Self {
        Self {
            org: None,
            tournamentid: None,
            divisionid: None,
            roomid: room_id,
            roundid: round_id,
            ignore: None,
            ruleset: None,
            leftteamid: None,
            centerteamid: None,
            rightteamid: None,
            quizmasterid: None,
            contentjudgeid: None,
            clientkey: None,
            poolbracket_id: uuid::Uuid::nil(),
            last_modified_user: None,
            creator_id: None
        }
    }
    pub fn new_default(room_id: Uuid, round_id: Uuid) -> Self {
        Self {
            org: Some("Nazarene".to_string()),
            tournamentid: None,
            divisionid: None,
            roomid: room_id,
            roundid: round_id,
            ignore: Some(false),
            ruleset: Some("Tournament".to_string()),
            leftteamid: None,
            centerteamid: None,
            rightteamid: None,
            quizmasterid: None,
            contentjudgeid: None,
            clientkey: Some(String::new()),
            poolbracket_id: uuid::Uuid::nil(),
            last_modified_user: None,
            creator_id: None
        }
    }
    pub fn set_poolbracket_id(mut self, val: Uuid) -> Self {
        self.poolbracket_id = val;
        self
    }
    pub fn set_org(mut self, val: String) -> Self {
        self.org = Some(val);
        self
    }
    pub fn set_last_modified_user(mut self, user_id: Uuid) -> Self {
        self.last_modified_user = Some(user_id);
        self
    }
    pub fn set_creator_id(mut self, user_id: Uuid) -> Self {
        self.creator_id = Some(user_id);
        self
    }
    pub fn set_tournamentid(mut self, val: Option<Uuid>) -> Self {
        self.tournamentid = val;
        self
    }
    pub fn set_divisionid(mut self, val: Option<Uuid>) -> Self {
        self.divisionid = val;
        self
    }
    pub fn set_roomid(mut self, val: Uuid) -> Self {
        self.roomid = val;
        self
    }
    pub fn set_roundid(mut self, val: Uuid) -> Self {
        self.roundid = val;
        self
    }
    pub fn set_ignore(mut self, val: bool) -> Self {
        self.ignore = Some(val);
        self
    }
    pub fn set_ruleset(mut self, val: String) -> Self {
        self.ruleset = Some(val);
        self
    }
    pub fn set_leftteamid(mut self, val: Uuid) -> Self {
        self.leftteamid = Some(val);
        self
    }
    pub fn set_centerteamid(mut self, val: Option<Uuid>) -> Self {
        self.centerteamid = val;
        self
    }
    pub fn set_rightteamid(mut self, val: Uuid) -> Self {
        self.rightteamid = Some(val);
        self
    }
    pub fn set_quizmasterid(mut self, val: Uuid) -> Self {
        self.quizmasterid = Some(val);
        self
    }
    pub fn set_contentjudgeid(mut self, val: Option<Uuid>) -> Self {
        self.contentjudgeid = val;
        self
    }
    fn validate_all_are_some(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.org.is_none() {
            errors.push("org is required".to_string());
        }
        if self.ignore.is_none() {
            errors.push("ignore is required".to_string());
        }
        if self.ruleset.is_none() {
            errors.push("ruleset is required".to_string());
        }
        if self.leftteamid.is_none() {
            errors.push("leftteamid is required".to_string());
        }
        if self.rightteamid.is_none() {
            errors.push("rightteamid is required".to_string());
        }
        if self.quizmasterid.is_none() {
            errors.push("quizmasterid is required".to_string());
        }
        // if self.contentjudgeid.is_none() {
        //     errors.push("contentjudgeid is required".to_string());
        // }

        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(())
    }
    pub fn build(self) -> Result<NewGame, Vec<String>> {
        match self.validate_all_are_some() {
            Err(e) => {
                Err(e)
            },
            Ok(_) => {
                Ok(
                    NewGame {
                        org: self.org.unwrap(),
                        tournamentid: self.tournamentid,
                        divisionid: self.divisionid,
                        roomid: self.roomid,
                        roundid: self.roundid,
                        ignore: self.ignore.unwrap(),
                        ruleset: self.ruleset.unwrap(),
                        leftteamid: self.leftteamid.unwrap(),
                        centerteamid: self.centerteamid,
                        rightteamid: self.rightteamid.unwrap(),
                        quizmasterid: self.quizmasterid.unwrap(),
                        contentjudgeid: self.contentjudgeid,
                        clientkey: self.clientkey.unwrap_or_default(),
                        poolbracket_id: self.poolbracket_id,
                        last_modified_user: self.last_modified_user.unwrap_or(self.quizmasterid.unwrap()),
                        creator_id: self.creator_id.unwrap_or(self.quizmasterid.unwrap())
                    }
                )
            }
        }
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<Game> {
        let new_entity = self.build();
        create(db, &new_entity.unwrap())
    }
}

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
#[diesel(table_name = crate::schema::games)]
#[diesel(primary_key(gid))]
pub struct Game {
    pub gid: Uuid,
    pub org: String,
    pub tournamentid: Uuid,
    pub divisionid: Uuid,
    pub roomid: Uuid,
    pub roundid: Uuid,
    pub ignore: bool,
    pub ruleset: String,
    pub leftteamid: Uuid,
    pub centerteamid: Option<Uuid>,
    pub rightteamid: Uuid,
    pub quizmasterid: Uuid,
    pub contentjudgeid: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub clientkey: String,
    pub resend_gameevents_request_ts: Option<DateTime<Utc>>,
    pub resend_gameevents_response: Option<String>,
    pub resend_request_sent_ts: Option<DateTime<Utc>>,
    pub last_modified_user: Uuid,
    pub creator_id: Uuid,
    /// Soft-delete flag. When true the game is treated as deleted and excluded from reads.
    pub del_fl: bool,
    /// Pool bracket this game belongs to (required FK to pool_brackets).
    pub poolbracket_id: Uuid
}

#[derive(
    Insertable,
    Serialize,
    Deserialize,
    Debug,
    Clone
)]
#[diesel(table_name = crate::schema::games)]
pub struct NewGame {
    pub org: String,
    pub tournamentid: Option<Uuid>,
    pub divisionid: Option<Uuid>,
    pub roomid: Uuid,
    pub roundid: Uuid,
    pub ignore: bool,
    pub ruleset: String,
    pub leftteamid: Uuid,
    pub centerteamid: Option<Uuid>,
    pub rightteamid: Uuid,
    pub quizmasterid: Uuid,
    pub contentjudgeid: Option<Uuid>,
    pub clientkey: String,
    // The pool bracket this game belongs to (required FK). The create endpoint requires this; when
    // it is nil (programmatic callers such as seeds), the model resolves a default bracket for the
    // game's division.
    #[serde(default)]
    pub poolbracket_id: Uuid,
    #[serde(default)]
    pub last_modified_user: Uuid,
    // Set server-side from the authenticated user on create; a client-sent value is ignored.
    #[serde(default)]
    pub creator_id: Uuid
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::games)]
pub struct GameChangeset {
    pub org: Option<String>,
    pub tournamentid: Option<Uuid>,
    pub divisionid: Option<Uuid>,
    pub roomid: Option<Uuid>,
    pub roundid: Option<Uuid>,
    pub ignore: Option<bool>,
    pub ruleset: Option<String>,
    pub leftteamid: Option<Uuid>,
    pub centerteamid: Option<Uuid>,
    pub rightteamid: Option<Uuid>,
    pub quizmasterid: Option<Uuid>,
    pub contentjudgeid: Option<Uuid>,
    pub clientkey: Option<String>,
    pub poolbracket_id: Option<Uuid>,
    pub resend_gameevents_request_ts: Option<DateTime<Utc>>,
    pub resend_gameevents_response: Option<String>,
    pub resend_request_sent_ts: Option<DateTime<Utc>>
}

impl GameChangeset {
    pub fn empty() -> Self {
        Self {
            org: None,
            tournamentid: None,
            divisionid: None,
            roomid: None,
            roundid: None,
            ignore: None,
            ruleset: None,
            leftteamid: None,
            centerteamid: None,
            rightteamid: None,
            quizmasterid: None,
            contentjudgeid: None,
            clientkey: None,
            poolbracket_id: None,
            resend_gameevents_request_ts: None,
            resend_gameevents_response: None,
            resend_request_sent_ts: None
        }
    }
}

pub fn create(db: &mut database::Connection, item: &NewGame) -> QueryResult<Game> {
    use crate::schema::games::dsl::*;

    if !models::round::exists(db, item.roundid) {
        println!("Could not find Round by ID={}", &item.roundid);
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: Round with ID {} does not exist", item.roundid).into()
        ));
    }

    let mut game = item.clone();

    if item.tournamentid.is_none() {
        let round = crate::models::round::read(db,item.roundid).expect("round not found in database by ID");
        let division = crate::models::division::read(db, round.did).expect("division not found in database by ID");

        game = NewGame {
            tournamentid: Some(division.tid),
            divisionid: Some(round.did),
            ..game.clone()
        }
    } else if item.divisionid.is_none() {
        let round = crate::models::round::read(db,item.roundid).expect("round not found in database by ID");

        game = NewGame {
            divisionid: Some(round.did),
            ..game.clone()
        }
    }

    // API callers must supply a pool bracket (enforced in the service). Programmatic callers (e.g.
    // seeds, tests) may leave it nil, in which case we resolve/create a default bracket for the
    // division so a game always references a real bracket.
    if game.poolbracket_id.is_nil() {
        let division_id = game.divisionid.expect("divisionid resolved above");
        game.poolbracket_id = crate::models::pool_bracket::resolve_default_for_division(db, division_id, game.creator_id)?;
    }

    if !models::room::exists(db, item.roomid) {
        println!("Could not find Room by ID={}", &item.roomid);
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: Room with ID {} does not exist", item.roomid).into()
        ));
    }

    if !models::team::exists(db, item.leftteamid) {
        println!("Could not find Team by ID={}", &item.leftteamid);
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: Team with ID {} does not exist", item.leftteamid).into()
        ));
    }

    if !models::team::exists(db, item.rightteamid) {
        println!("Could not find Team by ID={}", &item.rightteamid);
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: Team with ID {} does not exist", item.rightteamid).into()
        ));
    }

    if !models::user::exists(db, item.quizmasterid) {
        println!("Could not find Team by ID={}", &item.quizmasterid);
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: User (QuizMaster) with ID {} does not exist", item.quizmasterid).into()
        ));
    }

    if item.leftteamid == item.rightteamid {
        return Err(diesel::result::Error::QueryBuilderError(
            "leftteamid and rightteamid cannot be the same".into()
        ));
    }

    if item.centerteamid.is_some() {
        if item.leftteamid == item.centerteamid.unwrap() {
            return Err(diesel::result::Error::QueryBuilderError(
                "leftteamid and centerteamid cannot be the same".into()
            ));
        }
        if item.centerteamid.unwrap() == item.rightteamid {
            return Err(diesel::result::Error::QueryBuilderError(
                "centerteamid and rightteamid cannot be the same".into()
            ));
        }
    }

    insert_into(games)
        .values(game)
        .get_result::<Game>(db)
}

pub fn create_update(db: &mut database::Connection, item: &GameChangeset) -> QueryResult<Game> {
    use crate::schema::games::dsl::*;

    insert_into(games).values(item).on_conflict(on_constraint(
        "games_org_tournament_division_room_round_clientkey_key"))
        .do_update()//games.filter(gid.eq(item_id)))
        .set(item)
        .get_result::<Game>(db)
}

pub fn read(db_conn: &mut database::Connection, item_id: Uuid) -> QueryResult<Game> {
    use crate::schema::games::dsl::*;
    games.filter(gid.eq(item_id)).filter(del_fl.eq(false)).first::<Game>(db_conn)
}

pub fn count(db_conn: &mut database::Connection) -> QueryResult<i64> {
    use crate::schema::games::dsl::*;
    games.filter(del_fl.eq(false)).count().get_result(db_conn)
}

pub fn count_by_tournament(db_conn: &mut database::Connection, tournament_id: Uuid) -> QueryResult<i64> {
    use crate::schema::games::dsl::*;
    games.filter(tournamentid.eq(tournament_id)).filter(del_fl.eq(false)).count().get_result(db_conn)
}

pub fn read_all(db_conn: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<Game>> {
    use crate::schema::games::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    games
        .filter(del_fl.eq(false))
        .order(gid)
        .limit(page_size)
        .offset(offset_val)
        .load::<Game>(db_conn)
}

/// Loads a page of games matching `games::<$col> == $val`, ordered by Division name,
/// then Start Time (the round's scheduled start; unscheduled sorts last via Postgres'
/// default ASC NULLS LAST), then Room name, with gid as a stable tiebreak. The ORDER BY
/// runs in SQL over joined tables so the ordering is stable across pages and the database
/// only materializes the requested page.
macro_rules! read_games_ordered {
    ($db:expr, $pagination:expr, $col:ident, $val:expr) => {{
        use crate::schema::{games, divisions, rounds, rooms};
        let page_size = $pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
        let offset_val = $pagination.page * page_size;
        games::table
            .inner_join(divisions::table.on(games::divisionid.eq(divisions::did)))
            .inner_join(rounds::table.on(games::roundid.eq(rounds::roundid)))
            .inner_join(rooms::table.on(games::roomid.eq(rooms::roomid)))
            .filter(games::$col.eq($val))
            .filter(games::del_fl.eq(false))
            .order((
                divisions::dname.asc(),
                rounds::scheduled_start_time.asc(),
                rooms::name.asc(),
                games::gid.asc(),
            ))
            .select(games::all_columns)
            .limit(page_size)
            .offset(offset_val)
            .load::<Game>($db)
    }};
}

pub fn read_all_games_of_round(db_conn: &mut database::Connection, round_id: Uuid, pagination: &PaginationParams) -> QueryResult<Vec<Game>> {
    read_games_ordered!(db_conn, pagination, roundid, round_id)
}

pub fn read_all_games_of_division(db: &mut database::Connection, division_id: Uuid, pagination: &PaginationParams) -> QueryResult<Vec<Game>> {
    read_games_ordered!(db, pagination, divisionid, division_id)
}

pub fn read_all_games_of_tournament(db: &mut database::Connection, tournament_id: Uuid, pagination: &PaginationParams) -> QueryResult<Vec<Game>> {
    read_games_ordered!(db, pagination, tournamentid, tournament_id)
}

pub fn read_all_games_of_room(db: &mut database::Connection, room_id: Uuid, pagination: &PaginationParams) -> QueryResult<Vec<Game>> {
    read_games_ordered!(db, pagination, roomid, room_id)
}

pub fn read_all_games_of_pool_bracket(db: &mut database::Connection, bracket_id: Uuid, pagination: &PaginationParams) -> QueryResult<Vec<Game>> {
    read_games_ordered!(db, pagination, poolbracket_id, bracket_id)
}

/// One page of games belonging to a division session — i.e. games whose `poolbracket_id` is one of
/// the session's pool brackets.
pub fn read_all_games_of_division_session(db: &mut database::Connection, session_id: Uuid, pagination: &PaginationParams) -> QueryResult<Vec<Game>> {
    let bracket_ids: Vec<Uuid> = {
        use crate::schema::pool_brackets::dsl::*;
        pool_brackets.filter(division_session_id.eq(session_id)).select(pool_bracket_id).load::<Uuid>(db)?
    };
    if bracket_ids.is_empty() {
        return Ok(Vec::new());
    }
    use crate::schema::{games, divisions, rounds, rooms};
    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;
    games::table
        .inner_join(divisions::table.on(games::divisionid.eq(divisions::did)))
        .inner_join(rounds::table.on(games::roundid.eq(rounds::roundid)))
        .inner_join(rooms::table.on(games::roomid.eq(rooms::roomid)))
        .filter(games::poolbracket_id.eq_any(&bracket_ids))
        .filter(games::del_fl.eq(false))
        .order((
            divisions::dname.asc(),
            rounds::scheduled_start_time.asc(),
            rooms::name.asc(),
            games::gid.asc(),
        ))
        .select(games::all_columns)
        .limit(page_size)
        .offset(offset_val)
        .load::<Game>(db)
}

/// One fully-formed row of the games data table: the game plus the display names of its
/// division/room/teams, the round's scheduled start time, and the game's 1-based ordinal
/// within its room (the "Round" column). Populates the whole table from a single request.
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct GameRow {
    pub gid: Uuid,
    pub divisionid: Uuid,
    pub division_name: String,
    pub roomid: Uuid,
    pub room_name: String,
    pub roundid: Uuid,
    /// 1-based position of this game among its room's games, ordered by scheduled start time.
    pub round_number: Option<i64>,
    #[schema(value_type = Option<String>, format = DateTime)]
    pub scheduled_start_time: Option<DateTime<Utc>>,
    pub leftteamid: Uuid,
    pub left_team_name: String,
    pub centerteamid: Option<Uuid>,
    pub center_team_name: Option<String>,
    pub rightteamid: Uuid,
    pub right_team_name: String,
    pub ignore: bool,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Utc>,
    pub last_modified_user_name: String,
    pub last_modified_user_id: Uuid,
}

/// Numbers every game within its room by scheduled start time (unscheduled sorts last, gid as
/// a stable tiebreak). Computed over the whole tournament so a game's number is consistent no
/// matter which page/scope it appears in. `all_games` is (gid, roomid, roundid) tuples.
fn compute_room_sequence(
    all_games: &[(Uuid, Uuid, Uuid)],
    round_start: &HashMap<Uuid, Option<DateTime<Utc>>>,
) -> HashMap<Uuid, i64> {
    let mut by_room: HashMap<Uuid, Vec<(Uuid, Option<DateTime<Utc>>)>> = HashMap::new();
    for (g_gid, g_roomid, g_roundid) in all_games {
        let start = round_start.get(g_roundid).cloned().flatten();
        by_room.entry(*g_roomid).or_default().push((*g_gid, start));
    }

    let mut sequence: HashMap<Uuid, i64> = HashMap::new();
    for (_room, mut list) in by_room {
        list.sort_by(|a, b| match (a.1, b.1) {
            (Some(x), Some(y)) => x.cmp(&y).then_with(|| a.0.to_string().cmp(&b.0.to_string())),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => a.0.to_string().cmp(&b.0.to_string()),
        });
        for (i, (g_gid, _)) in list.into_iter().enumerate() {
            sequence.insert(g_gid, (i + 1) as i64);
        }
    }
    sequence
}

/// Shared assembler: enriches a page of games with names, round start times, and the
/// tournament-wide room sequence numbering.
fn build_game_rows(
    db: &mut database::Connection,
    page_games: Vec<Game>,
    tournament_id: Uuid,
) -> QueryResult<Vec<GameRow>> {
    // All of the tournament's games (gid, roomid, roundid) — used only to number each game
    // within its room, so the number is stable across pages/scopes.
    let all_games: Vec<(Uuid, Uuid, Uuid)> = {
        use crate::schema::games::dsl::*;
        games
            .filter(tournamentid.eq(tournament_id))
            .select((gid, roomid, roundid))
            .load::<(Uuid, Uuid, Uuid)>(db)?
    };

    // Scheduled start time for every round those games reference.
    let round_ids: Vec<Uuid> = all_games.iter().map(|(_, _, r)| *r).collect();
    let round_start: HashMap<Uuid, Option<DateTime<Utc>>> = {
        use crate::schema::rounds::dsl::*;
        rounds
            .filter(roundid.eq_any(&round_ids))
            .select((roundid, scheduled_start_time))
            .load::<(Uuid, Option<DateTime<Utc>>)>(db)?
            .into_iter()
            .collect()
    };

    let sequence = compute_room_sequence(&all_games, &round_start);

    // Display names for just the page of games.
    let div_ids: Vec<Uuid> = page_games.iter().map(|g| g.divisionid).collect();
    let division_name_by_id: HashMap<Uuid, String> = {
        use crate::schema::divisions::dsl::*;
        divisions.filter(did.eq_any(&div_ids)).select((did, dname)).load::<(Uuid, String)>(db)?.into_iter().collect()
    };
    let room_ids: Vec<Uuid> = page_games.iter().map(|g| g.roomid).collect();
    let room_name_by_id: HashMap<Uuid, String> = {
        use crate::schema::rooms::dsl::*;
        rooms.filter(roomid.eq_any(&room_ids)).select((roomid, name)).load::<(Uuid, String)>(db)?.into_iter().collect()
    };
    let mut team_ids: Vec<Uuid> = Vec::new();
    for g in &page_games {
        team_ids.push(g.leftteamid);
        team_ids.push(g.rightteamid);
        if let Some(c) = g.centerteamid {
            team_ids.push(c);
        }
    }
    let team_name_by_id: HashMap<Uuid, String> = {
        use crate::schema::teams::dsl::*;
        teams.filter(teamid.eq_any(&team_ids)).select((teamid, name)).load::<(Uuid, String)>(db)?.into_iter().collect()
    };
    let modifier_ids: Vec<Uuid> = page_games.iter().map(|g| g.last_modified_user).collect();
    let modifier_name_by_id = crate::models::user::read_display_names(db, &modifier_ids)?;

    let rows = page_games
        .into_iter()
        .map(|g| GameRow {
            gid: g.gid,
            division_name: division_name_by_id.get(&g.divisionid).cloned().unwrap_or_default(),
            divisionid: g.divisionid,
            room_name: room_name_by_id.get(&g.roomid).cloned().unwrap_or_default(),
            roomid: g.roomid,
            round_number: sequence.get(&g.gid).copied(),
            scheduled_start_time: round_start.get(&g.roundid).cloned().flatten(),
            roundid: g.roundid,
            left_team_name: team_name_by_id.get(&g.leftteamid).cloned().unwrap_or_default(),
            leftteamid: g.leftteamid,
            center_team_name: g.centerteamid.and_then(|c| team_name_by_id.get(&c).cloned()),
            centerteamid: g.centerteamid,
            right_team_name: team_name_by_id.get(&g.rightteamid).cloned().unwrap_or_default(),
            rightteamid: g.rightteamid,
            ignore: g.ignore,
            created_at: g.created_at,
            updated_at: g.updated_at,
            last_modified_user_name: modifier_name_by_id
                .get(&g.last_modified_user)
                .cloned()
                .unwrap_or_else(|| g.last_modified_user.to_string()),
            last_modified_user_id: g.last_modified_user,
        })
        .collect();

    Ok(rows)
}

/// Returns one page of enriched game rows for the tournament, plus the total game count.
pub fn read_game_rows_of_tournament(
    db: &mut database::Connection,
    tournament_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<(Vec<GameRow>, i64)> {
    let total: i64 = {
        use crate::schema::games::dsl::*;
        games.filter(tournamentid.eq(tournament_id)).filter(del_fl.eq(false)).count().get_result(db)?
    };
    let page = read_all_games_of_tournament(db, tournament_id, pagination)?;
    Ok((build_game_rows(db, page, tournament_id)?, total))
}

/// Returns one page of enriched game rows for the division, plus the total game count.
pub fn read_game_rows_of_division(
    db: &mut database::Connection,
    division_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<(Vec<GameRow>, i64)> {
    let tournament_id: Uuid = {
        use crate::schema::divisions::dsl::*;
        divisions.filter(did.eq(division_id)).select(tid).first::<Uuid>(db)?
    };
    let total: i64 = {
        use crate::schema::games::dsl::*;
        games.filter(divisionid.eq(division_id)).filter(del_fl.eq(false)).count().get_result(db)?
    };
    let page = read_all_games_of_division(db, division_id, pagination)?;
    Ok((build_game_rows(db, page, tournament_id)?, total))
}

/// Returns one page of enriched game rows for the round, plus the total game count.
pub fn read_game_rows_of_round(
    db: &mut database::Connection,
    round_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<(Vec<GameRow>, i64)> {
    let division_id: Uuid = {
        use crate::schema::rounds::dsl::*;
        rounds.filter(roundid.eq(round_id)).select(did).first::<Uuid>(db)?
    };
    let tournament_id: Uuid = {
        use crate::schema::divisions::dsl::*;
        divisions.filter(did.eq(division_id)).select(tid).first::<Uuid>(db)?
    };
    let total: i64 = {
        use crate::schema::games::dsl::*;
        games.filter(roundid.eq(round_id)).filter(del_fl.eq(false)).count().get_result(db)?
    };
    let page = read_all_games_of_round(db, round_id, pagination)?;
    Ok((build_game_rows(db, page, tournament_id)?, total))
}

/// Returns one page of enriched game rows for the room, plus the total game count.
pub fn read_game_rows_of_room(
    db: &mut database::Connection,
    room_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<(Vec<GameRow>, i64)> {
    let tournament_id: Uuid = {
        use crate::schema::rooms::dsl::*;
        rooms.filter(roomid.eq(room_id)).select(tid).first::<Uuid>(db)?
    };
    let total: i64 = {
        use crate::schema::games::dsl::*;
        games.filter(roomid.eq(room_id)).filter(del_fl.eq(false)).count().get_result(db)?
    };
    let page = read_all_games_of_room(db, room_id, pagination)?;
    Ok((build_game_rows(db, page, tournament_id)?, total))
}

/// Returns one page of enriched game rows for the pool bracket (games whose `poolbracket_id`
/// matches), plus the total game count.
pub fn read_game_rows_of_pool_bracket(
    db: &mut database::Connection,
    bracket_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<(Vec<GameRow>, i64)> {
    // Resolve the owning tournament: pool_bracket -> division_session -> division -> tournament.
    let session_id: Uuid = {
        use crate::schema::pool_brackets::dsl::*;
        pool_brackets.filter(pool_bracket_id.eq(bracket_id)).select(division_session_id).first::<Uuid>(db)?
    };
    let division_id: Uuid = {
        use crate::schema::division_sessions::dsl::*;
        division_sessions.filter(division_session_id.eq(session_id)).select(did).first::<Uuid>(db)?
    };
    let tournament_id: Uuid = {
        use crate::schema::divisions::dsl::*;
        divisions.filter(did.eq(division_id)).select(tid).first::<Uuid>(db)?
    };
    let total: i64 = {
        use crate::schema::games::dsl::*;
        games.filter(poolbracket_id.eq(bracket_id)).filter(del_fl.eq(false)).count().get_result(db)?
    };
    let page = read_all_games_of_pool_bracket(db, bracket_id, pagination)?;
    Ok((build_game_rows(db, page, tournament_id)?, total))
}

/// Returns one page of enriched game rows for a division session (games across its pool brackets),
/// plus the total game count.
pub fn read_game_rows_of_division_session(
    db: &mut database::Connection,
    session_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<(Vec<GameRow>, i64)> {
    let division_id: Uuid = {
        use crate::schema::division_sessions::dsl::*;
        division_sessions.filter(division_session_id.eq(session_id)).select(did).first::<Uuid>(db)?
    };
    let tournament_id: Uuid = {
        use crate::schema::divisions::dsl::*;
        divisions.filter(did.eq(division_id)).select(tid).first::<Uuid>(db)?
    };
    let bracket_ids: Vec<Uuid> = {
        use crate::schema::pool_brackets::dsl::*;
        pool_brackets.filter(division_session_id.eq(session_id)).select(pool_bracket_id).load::<Uuid>(db)?
    };
    let total: i64 = {
        use crate::schema::games::dsl::*;
        games.filter(poolbracket_id.eq_any(&bracket_ids)).filter(del_fl.eq(false)).count().get_result(db)?
    };
    let page = read_all_games_of_division_session(db, session_id, pagination)?;
    Ok((build_game_rows(db, page, tournament_id)?, total))
}

pub fn read_all_games_of_team(db: &mut database::Connection, team_id: Uuid, pagination: &PaginationParams) -> QueryResult<Vec<Game>> {
    use crate::schema::games::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    games
        .filter(
            leftteamid.eq(team_id)
                .or(centerteamid.eq(team_id))
                .or(rightteamid.eq(team_id))
        )
        .filter(del_fl.eq(false))
        .order(gid)
        .limit(page_size)
        .offset(offset_val)
        .load::<Game>(db)
}

pub fn read_all_games_where_user_is_quizmaster(db: &mut database::Connection, qm_id: Uuid, pagination: &PaginationParams) -> QueryResult<Vec<Game>> {
    use crate::schema::games::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    games
        .filter(quizmasterid.eq(qm_id))
        .filter(del_fl.eq(false))
        .order(gid)
        .limit(page_size)
        .offset(offset_val)
        .load::<Game>(db)
}

pub fn read_all_games_where_user_is_contentjudge(db: &mut database::Connection, cj_id: Uuid, pagination: &PaginationParams) -> QueryResult<Vec<Game>> {
    use crate::schema::games::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    games
        .filter(contentjudgeid.eq(cj_id))
        .filter(del_fl.eq(false))
        .order(gid)
        .limit(page_size)
        .offset(offset_val)
        .load::<Game>(db)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameWithNames {
    pub gid: Uuid,
    pub org: String,
    pub tournamentid: Uuid,
    pub tournament_name: String,
    pub tournament_fromdate: NaiveDate,
    pub tournament_todate: NaiveDate,
    pub divisionid: Uuid,
    pub roomid: Uuid,
    pub roundid: Uuid,
    pub ignore: bool,
    pub ruleset: String,
    pub leftteamid: Uuid,
    pub left_team_name: String,
    pub centerteamid: Option<Uuid>,
    pub center_team_name: Option<String>,
    pub rightteamid: Uuid,
    pub right_team_name: String,
    pub quizmasterid: Uuid,
    pub contentjudgeid: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub creator_id: Uuid,
    pub creator_name: String,
    pub last_modified_user_id: Uuid,
    pub last_modified_user_name: String,
}

fn enrich_games_with_names(
    db: &mut database::Connection,
    game_list: Vec<Game>,
) -> QueryResult<Vec<GameWithNames>> {
    if game_list.is_empty() {
        return Ok(vec![]);
    }

    let tournament_ids: Vec<Uuid> = game_list.iter().map(|g| g.tournamentid).collect();
    let tournament_map: HashMap<Uuid, (String, NaiveDate, NaiveDate)> = {
        use crate::schema::tournaments::dsl::*;
        tournaments
            .filter(tid.eq_any(&tournament_ids))
            .load::<crate::models::tournament::Tournament>(db)?
            .into_iter()
            .map(|t| (t.tid, (t.tname, t.fromdate, t.todate)))
            .collect()
    };

    let team_ids: Vec<Uuid> = game_list.iter().flat_map(|g| {
        let mut ids = vec![g.leftteamid, g.rightteamid];
        if let Some(c) = g.centerteamid { ids.push(c); }
        ids
    }).collect();
    let team_map: HashMap<Uuid, String> = {
        use crate::schema::teams::dsl::*;
        teams
            .filter(teamid.eq_any(&team_ids))
            .load::<crate::models::team::Team>(db)?
            .into_iter()
            .map(|t| (t.teamid, t.name))
            .collect()
    };

    let mut modifier_ids: Vec<Uuid> = game_list.iter().map(|g| g.creator_id).collect();
    modifier_ids.extend(game_list.iter().map(|g| g.last_modified_user));
    let user_name_map = crate::models::user::read_display_names(db, &modifier_ids)?;
    let name_of = |id: Uuid| user_name_map.get(&id).cloned().unwrap_or_else(|| id.to_string());

    Ok(game_list.into_iter().filter_map(|g| {
        let (tournament_name, tournament_fromdate, tournament_todate) =
            tournament_map.get(&g.tournamentid)?.clone();
        let left_team_name = team_map.get(&g.leftteamid).cloned().unwrap_or_default();
        let right_team_name = team_map.get(&g.rightteamid).cloned().unwrap_or_default();
        let center_team_name = g.centerteamid.and_then(|cid| team_map.get(&cid).cloned());
        Some(GameWithNames {
            creator_name: name_of(g.creator_id),
            last_modified_user_name: name_of(g.last_modified_user),
            creator_id: g.creator_id,
            last_modified_user_id: g.last_modified_user,
            gid: g.gid,
            org: g.org,
            tournamentid: g.tournamentid,
            tournament_name,
            tournament_fromdate,
            tournament_todate,
            divisionid: g.divisionid,
            roomid: g.roomid,
            roundid: g.roundid,
            ignore: g.ignore,
            ruleset: g.ruleset,
            leftteamid: g.leftteamid,
            left_team_name,
            centerteamid: g.centerteamid,
            center_team_name,
            rightteamid: g.rightteamid,
            right_team_name,
            quizmasterid: g.quizmasterid,
            contentjudgeid: g.contentjudgeid,
            created_at: g.created_at,
            updated_at: g.updated_at,
        })
    }).collect())
}

pub fn read_all_games_where_user_is_quizmaster_enriched(
    db: &mut database::Connection,
    qm_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<Vec<GameWithNames>> {
    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    let game_list: Vec<Game> = {
        use crate::schema::games::dsl::*;
        games
            .filter(quizmasterid.eq(qm_id))
            .filter(del_fl.eq(false))
            .order(created_at.desc())
            .limit(page_size)
            .offset(offset_val)
            .load::<Game>(db)?
    };

    enrich_games_with_names(db, game_list)
}

pub fn read_all_games_where_user_is_contentjudge_enriched(
    db: &mut database::Connection,
    cj_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<Vec<GameWithNames>> {
    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    let game_list: Vec<Game> = {
        use crate::schema::games::dsl::*;
        games
            .filter(contentjudgeid.eq(cj_id))
            .filter(del_fl.eq(false))
            .order(created_at.desc())
            .limit(page_size)
            .offset(offset_val)
            .load::<Game>(db)?
    };

    enrich_games_with_names(db, game_list)
}

pub fn read_all_games_of_statsgroup(db: &mut database::Connection, sg_id: Uuid, pagination: &PaginationParams) -> QueryResult<Vec<Game>> {
    use crate::schema::games_statsgroups::dsl::*;
    use crate::schema::games::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    let game_ids: Vec<Uuid> = 
        games_statsgroups
            .filter(statsgroupid.eq(sg_id))
            .load::<GameStatsGroup>(db)
            .unwrap()
            .iter()
            .map(|gsg| gsg.gameid)
            .collect();

    games
        .filter(gid.eq_any(game_ids))
        .filter(del_fl.eq(false))
        .order(gid.asc())
        .limit(page_size)
        .offset(offset_val)
        .load::<Game>(db)
}

/// Flags a game so the next ping from its room returns a "resend all events" command:
/// sets resend_gameevents_request_ts to now and clears resend_request_sent_ts so the fresh
/// request is considered live (not yet sent).
pub fn request_gameevents_resend(db_conn: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::games::dsl::{games, gid, resend_gameevents_request_ts, resend_request_sent_ts};
    diesel::update(games.filter(gid.eq(item_id)))
        .set((
            resend_gameevents_request_ts.eq(Utc::now()),
            resend_request_sent_ts.eq(None::<DateTime<Utc>>),
        ))
        .execute(db_conn)
}

pub fn update(db_conn: &mut database::Connection, item_id: Uuid, item: &GameChangeset, modified_by: Uuid) -> QueryResult<Game> {
    use crate::schema::games::dsl::*;
    diesel::update(games.find(item_id))
        .set((
            item,
            updated_at.eq(diesel::dsl::now),
            last_modified_user.eq(modified_by),
        ))
        .returning(Game::as_returning())
        .get_result(db_conn)
}

/// Soft delete: mark the game deleted (excluded from reads) without removing the row.
pub fn delete(db_conn: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::games::dsl::*;
    diesel::update(games.filter(gid.eq(item_id)))
        .set(del_fl.eq(true))
        .execute(db_conn)
}

/// Purge: permanently remove the game row from the database.
pub fn purge(db_conn: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::games::dsl::*;
    diesel::delete(games.filter(gid.eq(item_id))).execute(db_conn)
}

// Construct a key for the game information.
// we will use this to retrieve any information we have
// on this particular game.  
// pub fn get_gid_from_cache(game: &GameChangeset) -> Uuid {
//     // use crate::schema::games::dsl::*;
//     let gamekey = format!("QV:GAME:{}:{:?}:{:?}:{:?}:{:?}:{}",game.org.unwrap(),game.tournamentid, game.divisionid, game.roomid, game.roundid, game.clientkey.unwrap());

//     let client = redis::Client::open("redis://127.0.0.1/").unwrap();
//     let mut con = client.get_connection().unwrap();

//     // Now lets read the cache to see if we have this entry.
//     match redis::cmd("get").arg(&gamekey).query::<Option<String>>(&mut con) {
//         Ok(nil) => {
//             return -1;   // not found
//         },
//         Ok(json) => {
//             let json_str : String = json.unwrap();
//             let info : Game = serde_json::from_str(&json_str).unwrap(); 
//             return info.gid;
//         },
//         Err(e) => {
//             log::error!("{} {} Fault retrieving redis cache for game {:?}",module_path!(),line!(),e);
//             return -1 ;   // not found
//         },
//     }
// }
// ─── Per-game readiness status (for the Game Selection view) ──────────────────

/// Whether a game's recorded events are internally valid and whether the game is complete.
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct GameStatus {
    pub gid: Uuid,
    pub done: bool,     // has a question-20 record and all ties are resolved (distinct final scores)
    pub data_ok: bool,  // the game has events that score without error
    pub next_question: Option<i32>, // highest recorded question number + 1 (None if no events)
}

/// Computes readiness status for every game in a tournament by scoring each game's events.
pub fn read_game_statuses_of_tournament(db: &mut database::Connection, tournament_id: Uuid) -> QueryResult<Vec<GameStatus>> {
    let pagination = PaginationParams { page: 0, page_size: PaginationParams::MAX_PAGE_SIZE as i64 };
    let games = read_all_games_of_tournament(db, tournament_id, &pagination)?;

    // First pass: load each game's events and record the latest server timestamp among
    // them. The timestamps let us tell, in the second pass, whether a game's teams have
    // moved on to a later round (their next game has recorded events after this one ended).
    let mut events_by_gid: HashMap<Uuid, Vec<crate::models::gameevent::GameEvent>> = HashMap::new();
    let mut latest_serverts_by_gid: HashMap<Uuid, DateTime<Utc>> = HashMap::new();
    for game in &games {
        let events = crate::models::gameevent::read_all_gameevents_of_game(db, game.gid, &pagination)?;
        if let Some(latest) = events.iter().map(|e| e.serverts).max() {
            latest_serverts_by_gid.insert(game.gid, latest);
        }
        events_by_gid.insert(game.gid, events);
    }

    let mut statuses = Vec::with_capacity(games.len());
    for game in &games {
        let events = events_by_gid.get(&game.gid).cloned().unwrap_or_default();
        let has_events = !events.is_empty();
        let has_question_20 = events.iter().any(|e| e.question >= 20);
        // The next question to be played: highest recorded question number + 1.
        let next_question = events.iter().map(|e| e.question).max().map(|m| m + 1);

        // A game can score without error yet still be missing events, so completeness (no
        // sequential gaps in question/eventnum) must be part of "data OK". Computed before
        // the calculator consumes `events`.
        let has_gaps = crate::models::gameevent::events_have_gaps(&events);
        let results = crate::models::gameevent::calculate_team_results_for_game(game.gid, events);
        // Data is OK when there are events, they have no gaps, and they score without error.
        let data_ok = has_events && !has_gaps && results.is_ok();
        // Ties are resolved when no two teams share the same placement rank. Ranks (not
        // scores) are the right signal: an overtime game decides a winner via distinct ranks
        // while the displayed score stays tied, so a score comparison would miss it.
        let ties_resolved = match &results {
            Ok(teams) if !teams.is_empty() => {
                let mut ranks: Vec<i32> = teams.iter().map(|t| t.rank).collect();
                ranks.sort_unstable();
                ranks.windows(2).all(|w| w[0] != w[1])
            }
            _ => false,
        };

        // A team has "moved on" when it appears in another game whose events were recorded
        // after this game's last event. This resolves overtime games where the score stays
        // tied (Q21+ decides a winner without changing the score): the ties_resolved score
        // check can't detect the outcome, but the teams starting their next round proves the
        // game is finished.
        let team_moved_on = match latest_serverts_by_gid.get(&game.gid).copied() {
            Some(this_game_end) => {
                let this_teams: Vec<Uuid> = [Some(game.leftteamid), game.centerteamid, Some(game.rightteamid)]
                    .into_iter()
                    .flatten()
                    .collect();
                games.iter().any(|other| {
                    other.gid != game.gid
                        && latest_serverts_by_gid
                            .get(&other.gid)
                            .is_some_and(|other_end| *other_end > this_game_end)
                        && this_teams.iter().any(|t| {
                            other.leftteamid == *t
                                || other.rightteamid == *t
                                || other.centerteamid == Some(*t)
                        })
                })
            }
            None => false,
        };

        let done = data_ok && has_question_20 && (ties_resolved || team_moved_on);

        statuses.push(GameStatus { gid: game.gid, done, data_ok, next_question });
    }
    Ok(statuses)
}
