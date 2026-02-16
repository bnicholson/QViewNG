use chrono::{DateTime, Utc};
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
    pub updated_at: DateTime<Utc>
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
//         ignore: false,
//         ruleset: "".to_string()
//     }
// }

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

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    games
        .filter(divisionid.eq(division_id))
        .order(gid)
        .limit(page_size)
        .offset(offset_val)
        .load::<Game>(db)
}

pub fn read_all_games_of_tournament(db: &mut database::Connection, tournament_id: Uuid, pagination: &PaginationParams) -> QueryResult<Vec<Game>> {
    use crate::schema::games::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    games
        .filter(tournamentid.eq(tournament_id))
        .order(gid)
        .limit(page_size)
        .offset(offset_val)
        .load::<Game>(db)
}

pub fn read_all_games_of_room(db: &mut database::Connection, room_id: Uuid, pagination: &PaginationParams) -> QueryResult<Vec<Game>> {
    use crate::schema::games::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    games
        .filter(roomid.eq(room_id))
        .order(gid)
        .limit(page_size)
        .offset(offset_val)
        .load::<Game>(db)
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
        .order(gid)
        .limit(page_size)
        .offset(offset_val)
        .load::<Game>(db)
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
        .order(gid.asc())
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