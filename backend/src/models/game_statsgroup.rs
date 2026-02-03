
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
    pub fn build(self) -> Result<NewGameStatsGroup, Vec<String>> {
        Ok(
            NewGameStatsGroup {
                gameid: self.gameid,
                statsgroupid: self.statsgroupid,
            }
        )
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<GameStatsGroup> {
        let new_gamestatsgroup = self.build();
        create(db, &new_gamestatsgroup.unwrap())
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

pub fn delete(db: &mut database::Connection, statsgroup_id: Uuid, game_id: Uuid) -> QueryResult<usize> {
    use crate::schema::games_statsgroups::dsl::*;
    diesel::delete(
        games_statsgroups
            .filter(statsgroupid.eq(statsgroup_id))
            .filter(gameid.eq(game_id))
    ).execute(db)
}
