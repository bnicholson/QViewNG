use chrono::{DateTime, Utc};
use diesel::{AsChangeset,Insertable,Identifiable,Queryable};
use diesel::prelude::*;
use diesel::upsert::*;
use diesel::insert_into;
use uuid::Uuid;
use crate::database;
use crate::models::common::PaginationParams;
use crate::models::division::Division;
use crate::models::round::Round;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub struct GameBuilder {
    org: Option<String>,
    tournamentid: Option<Uuid>,
    divisionid: Option<Uuid>,
    roomid: Uuid,
    roundid: Uuid,
    clientkey: Option<String>,
    ignore: Option<bool>,
    ruleset: Option<String>,
    leftteamid: Option<Uuid>,
    centerteamid: Option<Uuid>,
    rightteamid: Option<Uuid>,
    quizmasterid: Option<Uuid>,
    contentjudgeid: Option<Uuid>
}

impl GameBuilder {
    pub fn new(room_id: Uuid, round_id: Uuid) -> Self {
        Self {
            org: None,
            tournamentid: None,
            divisionid: None,
            roomid: room_id,
            roundid: round_id,
            clientkey: None,
            ignore: None,
            ruleset: None,
            leftteamid: None,
            centerteamid: None,
            rightteamid: None,
            quizmasterid: None,
            contentjudgeid: None
        }
    }
    pub fn new_default(room_id: Uuid, round_id: Uuid) -> Self {
        Self {
            org: Some("Nazarene".to_string()),
            tournamentid: None,
            divisionid: None,
            roomid: room_id,
            roundid: round_id,
            clientkey: Some("".to_string()),
            ignore: Some(false),
            ruleset: Some("Tournament".to_string()),
            leftteamid: None,
            centerteamid: None,
            rightteamid: None,
            quizmasterid: None,
            contentjudgeid: None
        }
    }
    pub fn set_org(mut self, val: String) -> Self {
        self.org = Some(val);
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
    pub fn set_clientkey(mut self, val: String) -> Self {
        self.clientkey = Some(val);
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
        // if self.tournamentid.is_none() {
        //     errors.push("tournamentid is required".to_string());
        // }
        // if self.divisionid.is_none() {
        //     errors.push("divisionid is required".to_string());
        // }
        if self.clientkey.is_none() {
            errors.push("clientkey is required".to_string());
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
                        clientkey: self.clientkey.unwrap(),
                        ignore: self.ignore.unwrap(),
                        ruleset: self.ruleset.unwrap(),
                        leftteamid: self.leftteamid.unwrap(),
                        centerteamid: self.centerteamid,
                        rightteamid: self.rightteamid.unwrap(),
                        quizmasterid: self.quizmasterid.unwrap(),
                        contentjudgeid: self.contentjudgeid
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

// #[tsync::tsync]
// #[diesel(belongs_to(Tournament, foreign_key = "tournamentid"))]
// #[diesel(belongs_to(Division, foreign_key = "divisionid"))]
// #[diesel(belongs_to(Room, foreign_key = "roomid"))]
// #[diesel(belongs_to(Game, foreign_key = "roundid"))]
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
// #[belongs_to(Game, foreign_key = "roundid")]
// #[diesel(primary_key(gid))]
#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::games)]
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

// pub fn create_update(db_conn: &mut database::Connection, item: &GameChangeset) -> QueryResult<Game> {
//     use crate::schema::games::dsl::*;
//     insert_into(games).values(item).on_conflict(on_constraint(
//         "games_org_tournament_division_room_round_clientkey_key"))
//         .do_update()//games.filter(gid.eq(item_id)))
//         .set(item)
//         .get_result::<Game>(db_conn)
// }

pub fn read(db_conn: &mut database::Connection, item_id: Uuid) -> QueryResult<Game> {
    use crate::schema::games::dsl::*;
    games.filter(gid.eq(item_id)).first::<Game>(db_conn)
}

pub fn read_all(db_conn: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<Game>> {
    use crate::schema::games::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    games
        .order(gid)
        .limit(page_size)
        .offset(offset_val)
        .load::<Game>(db_conn)
}

pub fn read_all_games_of_round(db_conn: &mut database::Connection, round_id: Uuid, pagination: &PaginationParams) -> QueryResult<Vec<Game>> {
    use crate::schema::games::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    games
        .filter(roundid.eq(round_id))
        .order(gid)
        .limit(page_size)
        .offset(offset_val)
        .load::<Game>(db_conn)
}

pub fn read_all_games_of_division(db: &mut database::Connection, division_id: Uuid, pagination: &PaginationParams) -> QueryResult<Vec<Game>> {
    use crate::schema::games::dsl::*;
    use crate::schema::rounds::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    let rounds_ids: Vec<Uuid> = rounds
        .filter(did.eq(division_id))
        .order(scheduled_start_time.asc())
        .limit(page_size)
        .offset(offset_val)
        .load::<Round>(db)
        .unwrap()
        .iter()
        .map(|entity| entity.roundid)
        .collect();

    games
        .filter(crate::schema::games::dsl::roundid.eq_any(rounds_ids))
        .order(gid)
        .limit(page_size)
        .offset(offset_val)
        .load::<Game>(db)
}

pub fn read_all_games_of_tournament(db: &mut database::Connection, tournament_id: Uuid, pagination: &PaginationParams) -> QueryResult<Vec<Game>> {
    use crate::schema::games::dsl::*;
    use crate::schema::rounds::dsl::*;
    use crate::schema::divisions::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    let divisions_ids: Vec<Uuid> = divisions
        .filter(tid.eq(tournament_id))
        .order(dname.asc())
        .limit(page_size)
        .offset(offset_val)
        .load::<Division>(db)
        .unwrap()
        .iter()
        .map(|entity| entity.did)
        .collect();

    let rounds_ids: Vec<Uuid> = rounds
        .filter(crate::schema::rounds::dsl::did.eq_any(divisions_ids))
        .order(scheduled_start_time.asc())
        .limit(page_size)
        .offset(offset_val)
        .load::<Round>(db)
        .unwrap()
        .iter()
        .map(|entity| entity.roundid)
        .collect();

    games
        .filter(crate::schema::games::dsl::roundid.eq_any(rounds_ids))
        .order(gid)
        .limit(page_size)
        .offset(offset_val)
        .load::<Game>(db)
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