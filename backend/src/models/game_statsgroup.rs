
use crate::database;
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult,Insertable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use chrono::{Utc,DateTime};

pub struct GameStatsGroupBuilder {
    gameid: Uuid,
    statsgroupid: Uuid,
}

impl GameStatsGroupBuilder {
    pub fn new(gameid: Uuid, statsgroupid: Uuid) -> Self {
        Self {
            gameid,
            statsgroupid,
        }
    }
    pub fn new_default(gameid: Uuid, statsgroupid: Uuid) -> Self {
        Self {
            gameid,
            statsgroupid,
        }
    }
    // pub fn set_name(mut self, gamestatsgroup_name: String) -> Self {
    //     self.name = gamestatsgroup_name;
    //     self
    // }
    // pub fn set_description(mut self, description: Option<String>) -> Self {
    //     self.description = description;
    //     self
    // }
    // fn validate_all_are_some(&self) -> Result<(), Vec<String>> {
    //     let mut errors = Vec::new();
    //     if self.name.is_none() {
    //         errors.push("name is required".to_string());
    //     }
    //     if !errors.is_empty() {
    //         return Err(errors);
    //     }
    //     Ok(())
    // }
    pub fn build(self) -> Result<NewGameStatsGroup, Vec<String>> {
        Ok(
            NewGameStatsGroup {
                gameid: self.gameid,
                statsgroupid: self.statsgroupid,
            }
        )
        // match self.validate_all_are_some() {
        //     Err(e) => {
        //         Err(e)
        //     },
        //     Ok(_) => {
        //         Ok(
        //             NewGameStatsGroup {
        //                 name: self.name,
        //                 description: self.description,
        //             }
        //         )
        //     }
        // }
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<GameStatsGroup> {
        let new_gamestatsgroup = self.build();
        create(db, &new_gamestatsgroup.unwrap())
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
#[diesel(table_name = crate::schema::games_statsgroups)]
#[diesel(primary_key(gameid, statsgroupid))]
pub struct GameStatsGroup {
    pub gameid: Uuid,
    pub statsgroupid: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(
    Insertable,
    Serialize,
    Deserialize,
    Debug
)]
#[diesel(table_name = crate::schema::games_statsgroups)]
pub struct NewGameStatsGroup {
    pub gameid: Uuid,
    pub statsgroupid: Uuid,
}

// #[tsync::tsync]
// #[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
// #[diesel(table_name = crate::schema::games_statsgroups)]
// #[diesel(primary_key(gameid, statsgroupid))]
// pub struct GameStatsGroupChangeset {
//     pub name: String,                              // Name of the gamestatsgroup (human readable)
//     pub description: Option<String>,               // Description of the gamestatsgroup
// }

pub fn create(db: &mut database::Connection, item: &NewGameStatsGroup) -> QueryResult<GameStatsGroup> {
    use crate::schema::games_statsgroups::dsl::*;
    insert_into(games_statsgroups)
        .values(item)
        .get_result::<GameStatsGroup>(db)
}

// pub fn exists(db: &mut database::Connection, gamestatsgroupid: Uuid) -> bool {
//     use crate::schema::games_statsgroups::dsl::games_statsgroups;
//     games_statsgroups
//         .find(gamestatsgroupid)
//         .get_result::<GameStatsGroup>(db)
//         .is_ok()
// }

// pub fn read(db: &mut database::Connection, item_id: Uuid) -> QueryResult<GameStatsGroup> {
//     use crate::schema::games_statsgroups::dsl::*;
//     games_statsgroups.filter(sgid.eq(item_id)).first::<GameStatsGroup>(db)
// }

// pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<GameStatsGroup>> {
//     use crate::schema::games_statsgroups::dsl::*;
//     games_statsgroups
//         .order(created_at)
//         .limit(pagination.page_size)
//         .offset(
//             pagination.page
//                 * std::cmp::max(pagination.page_size, PaginationParams::MAX_PAGE_SIZE as i64),
//         )
//         .load::<GameStatsGroup>(db)
// }

// pub fn update(db: &mut database::Connection, sg_id: Uuid, item: &GameStatsGroupChangeset) -> QueryResult<GameStatsGroup> {
//     use crate::schema::games_statsgroups::dsl::*;
//     diesel::update(games_statsgroups.filter(sgid.eq(sg_id)))
//         .set((
//             item,
//             updated_at.eq(diesel::dsl::now),
//         ))
//         .get_result(db)
// }

pub fn delete(db: &mut database::Connection, statsgroup_id: Uuid, game_id: Uuid) -> QueryResult<usize> {
    use crate::schema::games_statsgroups::dsl::*;
    diesel::delete(
        games_statsgroups
            .filter(statsgroupid.eq(statsgroup_id))
            .filter(gameid.eq(game_id))
    ).execute(db)
}
