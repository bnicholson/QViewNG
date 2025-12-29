// use diesel::{AsChangeset,Insertable,Identifiable,Queryable};
// use diesel::prelude::*;
// use diesel::upsert::*;
// use diesel::insert_into;
// use crate::database;
// use crate::models::common::*;
// use serde::{Deserialize, Serialize};
// use utoipa::ToSchema;
// // this import requires this syntax (to appease rustc):
// use crate::schema::games::dsl::{org,clientkey,ignore,ruleset};

// // #[tsync::tsync]
// #[derive(
//     Debug,
//     Serialize,
//     Deserialize,
//     Clone,
//     Queryable,
//     Insertable,
//     Identifiable,
//     AsChangeset,
//     Associations,
//     ToSchema,
//     Selectable
// )]
// #[diesel(table_name = crate::schema::games)]
// #[diesel(primary_key(gid))]
// #[diesel(belongs_to(Tournament, foreign_key = "tournamentid"))]
// #[diesel(belongs_to(Division, foreign_key = "divisionid"))]
// #[diesel(belongs_to(Room, foreign_key = "roomid"))]
// #[diesel(belongs_to(Round, foreign_key = "roundid"))]
// #[diesel(belongs_to(LeftTeam, foreign_key = "leftteamid"))]
// #[diesel(belongs_to(CenterTeam, foreign_key = "centerteamid"))]
// #[diesel(belongs_to(RightTeam, foreign_key = "rightteamid"))]
// #[diesel(belongs_to(QuizMaster, foreign_key = "quizmasterid"))]
// #[diesel(belongs_to(ContentJudge, foreign_key = "contentjudgeid"))]
// pub struct Game {
//     pub gid: i64,
//     pub org: String,
//     pub clientkey: String,
//     pub ignore: bool,
//     pub ruleset: String,
//     pub quizmasterid: Option<i64>,
//     pub contentjudgeid: Option<i64>,
//     pub tournamentid: Option<i64>,
//     pub divisionid: Option<i64>,
//     pub roomid: Option<i64>,
//     pub roundid: Option<i64>,
//     pub leftteamid: i64,
//     pub centerteamid: Option<i64>,
//     pub rightteamid: i64,
//     #[schema(value_type = String, format = DateTime)]
//     pub created_at: UTC,
//     #[schema(value_type = String, format = DateTime)]
//     pub updated_at: UTC,
// }


// // #[tsync::tsync]
// #[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset, Selectable)]
// #[diesel(table_name = crate::schema::games)]
// #[diesel(primary_key(gid))]
// #[belongs_to(Tournament, foreign_key = "tournamentid")]
// #[belongs_to(Division, foreign_key = "divisionid")]
// #[belongs_to(Room, foreign_key = "roomid")]
// #[belongs_to(Round, foreign_key = "roundid")]
// pub struct GameChangeset {
//     pub org: String,
//     pub tournamentid: Option<i64>,
//     pub divisionid: Option<i64>,
//     pub roomid: Option<i64>,
//     pub roundid: Option<i64>,
//     pub clientkey: String,
//     pub ignore: bool,
//     pub ruleset: String,
// }

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

// pub fn create(db_conn: &mut database::Connection, item: &GameChangeset) -> QueryResult<Game> {
//     use crate::schema::games::dsl::*;
//     insert_into(games).values(item).get_result::<Game>(db_conn)
// }

// pub fn create_update(db_conn: &mut database::Connection, item: &GameChangeset) -> QueryResult<Game> {
//     use crate::schema::games::dsl::*;
//     insert_into(games).values(item).on_conflict(on_constraint(
//         "games_org_tournament_division_room_round_clientkey_key"))
//         .do_update()//games.filter(gid.eq(item_id)))
//         .set(item)
//         .get_result::<Game>(db_conn)
// }

// pub fn read(db_conn: &mut database::Connection, item_id: i64) -> QueryResult<Game> {
//     use crate::schema::games::dsl::*;
//     games.filter(gid.eq(item_id)).first::<Game>(db_conn)
// }

// pub fn read_all(db_conn: &mut database::Connection) -> QueryResult<Vec<Game>> {
//     use crate::schema::games::dsl::*;
//     games
//         .order(gid)
//         .limit(10)
//         // .offset(44)
//         .load::<Game>(db_conn)
// }

// pub fn update(db_conn: &mut database::Connection, item_id: i64, item: &GameChangeset) -> QueryResult<Game> {
//     use crate::schema::games::dsl::*;
//     diesel::update(games.find(item_id))
//         .set(item)
//         .returning(Game::as_returning())
//         .get_result(db_conn)
// }

// pub fn delete(db_conn: &mut database::Connection, item_id: i64) -> QueryResult<usize> {
//     use crate::schema::games::dsl::*;
//     diesel::delete(games.filter(gid.eq(item_id))).execute(db_conn)
// }

// // Construct a key for the game information.
// // we will use this to retrieve any information we have
// // on this particular game.  
// pub fn get_gid_from_cache(game: &GameChangeset) -> BigId {
//     use crate::schema::games::dsl::*;
//     let gamekey = format!("QV:GAME:{}:{:?}:{:?}:{:?}:{:?}:{}",game.org,game.tournamentid, game.divisionid, game.roomid, game.roundid, game.clientkey);

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