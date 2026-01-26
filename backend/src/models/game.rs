use chrono::{DateTime, Utc};
use diesel::result::Error;
use diesel::{AsChangeset,Insertable,Identifiable,Queryable};
use diesel::prelude::*;
use diesel::upsert::*;
use diesel::insert_into;
use uuid::Uuid;
use crate::database;
use crate::models::common::PaginationParams;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
// this import requires this syntax (to appease rustc):
// use crate::schema::games::dsl::{org,clientkey,ignore,ruleset};

// #[tsync::tsync]
// #[diesel(belongs_to(Tournament, foreign_key = "tournamentid"))]
// #[diesel(belongs_to(Division, foreign_key = "divisionid"))]
// #[diesel(belongs_to(Room, foreign_key = "roomid"))]
// #[diesel(belongs_to(Round, foreign_key = "roundid"))]
// #[diesel(belongs_to(LeftTeam, foreign_key = "leftteamid"))]
// #[diesel(belongs_to(CenterTeam, foreign_key = "centerteamid"))]
// #[diesel(belongs_to(RightTeam, foreign_key = "rightteamid"))]
// #[diesel(belongs_to(QuizMaster, foreign_key = "quizmasterid"))]
// #[diesel(belongs_to(ContentJudge, foreign_key = "contentjudgeid"))]
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
    pub tournamentid: Option<Uuid>,
    pub divisionid: Option<Uuid>,
    pub roomid: Uuid,
    pub roundid: Uuid,
    pub clientkey: String,
    pub ignore: bool,
    pub ruleset: String,
    pub leftteamid: Uuid,
    pub centerteamid: Option<Uuid>,
    pub rightteamid: Uuid,
    pub quizmasterid: Uuid,
    pub contentjudgeid: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(
    Insertable,
    Serialize,
    Deserialize,
    Debug
)]
#[diesel(table_name = crate::schema::games)]
pub struct NewGame {
    pub org: String,
    pub tournamentid: Option<Uuid>,
    pub divisionid: Option<Uuid>,
    pub roomid: Uuid,
    pub roundid: Uuid,
    pub clientkey: String,
    pub ignore: bool,
    pub ruleset: String,
    pub leftteamid: Uuid,
    pub centerteamid: Option<Uuid>,
    pub rightteamid: Uuid,
    pub quizmasterid: Uuid,
    pub contentjudgeid: Option<Uuid>
}


// #[tsync::tsync]
// #[belongs_to(Tournament, foreign_key = "tournamentid")]
// #[belongs_to(Division, foreign_key = "divisionid")]
// #[belongs_to(Room, foreign_key = "roomid")]
// #[belongs_to(Round, foreign_key = "roundid")]
#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::games)]
#[diesel(primary_key(gid))]
pub struct GameChangeset {
    pub org: Option<String>,
    pub tournamentid: Option<Uuid>,
    pub divisionid: Option<Uuid>,
    pub roomid: Option<Uuid>,
    pub roundid: Option<Uuid>,
    pub clientkey: Option<String>,
    pub ignore: Option<bool>,
    pub ruleset: Option<String>,
    pub leftteamid: Option<Uuid>,
    pub centerteamid: Option<Uuid>,
    pub rightteamid: Option<Uuid>,
    pub quizmasterid: Option<Uuid>,
    pub contentjudgeid: Option<Uuid>
}

// pub fn empty_changeset() -> GameChangeset {
//     return GameChangeset {   
//         org: "".to_string(),
//         tournamentid: None,
//         divisionid: None,
//         roomid: None,
//         roundid: None,
//         clientkey: "".to_string(),
//         ignore: false,
//         ruleset: "".to_string()
//     }
// }

pub fn create(db_conn: &mut database::Connection, item: &NewGame) -> QueryResult<Game> {
    use crate::schema::games::dsl::*;

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

    insert_into(games).values(item).get_result::<Game>(db_conn)
}

pub fn create_update(db_conn: &mut database::Connection, item: &GameChangeset) -> QueryResult<Game> {
    use crate::schema::games::dsl::*;
    insert_into(games).values(item).on_conflict(on_constraint(
        "games_org_tournament_division_room_round_clientkey_key"))
        .do_update()//games.filter(gid.eq(item_id)))
        .set(item)
        .get_result::<Game>(db_conn)
}

pub fn read(db_conn: &mut database::Connection, item_id: Uuid) -> QueryResult<Game> {
    use crate::schema::games::dsl::*;
    games.filter(gid.eq(item_id)).first::<Game>(db_conn)
}

pub fn read_all(db_conn: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<Game>> {
    use crate::schema::games::dsl::*;
    games
        .order(gid)
        .limit(10)
        .offset(
            pagination.page
                * std::cmp::max(pagination.page_size, PaginationParams::MAX_PAGE_SIZE as i64),
        )
        .load::<Game>(db_conn)
}

pub fn update(db_conn: &mut database::Connection, item_id: Uuid, item: &GameChangeset) -> QueryResult<Game> {
    use crate::schema::games::dsl::*;
    diesel::update(games.find(item_id))
        .set((
            item,
            updated_at.eq(diesel::dsl::now),
        ))
        .returning(Game::as_returning())
        .get_result(db_conn)
}

pub fn delete(db_conn: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
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