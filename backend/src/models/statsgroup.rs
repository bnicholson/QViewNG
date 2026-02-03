
use crate::database;
use crate::models::common::PaginationParams;
use crate::models::game_statsgroup::GameStatsGroup;
use crate::schema::rounds::scheduled_question_eight_id;
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult,AsChangeset,Insertable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use chrono::{Utc,DateTime};

pub struct StatsGroupBuilder {
    name: String,                              // Name of the statsgroup (human readable)
    description: Option<String>,               // Description of the statsgroup
}

impl StatsGroupBuilder {
    pub fn new(statsgroup_name: &str) -> Self {
        Self {
            name: statsgroup_name.to_string(),
            description: None,
        }
    }
    pub fn new_default(statsgroup_name: &str) -> Self {
        Self {
            name: statsgroup_name.to_string(),
            description: None,
        }
    }
    pub fn set_name(mut self, statsgroup_name: String) -> Self {
        self.name = statsgroup_name;
        self
    }
    pub fn set_description(mut self, description: Option<String>) -> Self {
        self.description = description;
        self
    }
    pub fn build(self) -> Result<NewStatsGroup, Vec<String>> {
        Ok(
            NewStatsGroup {
                name: self.name,
                description: self.description,
            }
        )
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<StatsGroup> {
        let new_statsgroup = self.build();
        create(db, &new_statsgroup.unwrap())
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
#[diesel(table_name = crate::schema::statsgroups)]
#[diesel(primary_key(sgid))]
pub struct StatsGroup {
    pub sgid: Uuid,                                // identifies the statsgroup uniquely
    pub name: String,                              // Name of the statsgroup (human readable)
    pub description: Option<String>,               // Description of the statsgroup
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(
    Insertable,
    Serialize,
    Deserialize,
    Debug
)]
#[diesel(table_name = crate::schema::statsgroups)]
pub struct NewStatsGroup {
    pub name: String,                              // Name of the statsgroup (human readable)
    pub description: Option<String>,               // Description of the statsgroup
}

// #[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::statsgroups)]
#[diesel(primary_key(sgid))]
pub struct StatsGroupChangeset {
    pub name: String,                              // Name of the statsgroup (human readable)
    pub description: Option<String>,               // Description of the statsgroup
}

pub fn create(db: &mut database::Connection, item: &NewStatsGroup) -> QueryResult<StatsGroup> {
    use crate::schema::statsgroups::dsl::*;
    insert_into(statsgroups).values(item).get_result::<StatsGroup>(db)
}

pub fn exists(db: &mut database::Connection, statsgroupid: Uuid) -> bool {
    use crate::schema::statsgroups::dsl::statsgroups;
    statsgroups
        .find(statsgroupid)
        .get_result::<StatsGroup>(db)
        .is_ok()
}

pub fn read(db: &mut database::Connection, item_id: Uuid) -> QueryResult<StatsGroup> {
    use crate::schema::statsgroups::dsl::*;
    statsgroups.filter(sgid.eq(item_id)).first::<StatsGroup>(db)
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<StatsGroup>> {
    use crate::schema::statsgroups::dsl::*;
    statsgroups
        .order(created_at)
        .limit(pagination.page_size)
        .offset(
            pagination.page
                * std::cmp::max(pagination.page_size, PaginationParams::MAX_PAGE_SIZE as i64),
        )
        .load::<StatsGroup>(db)
}

pub fn read_all_statsgroups_of_game(db: &mut database::Connection, game_id: Uuid, pagination: &PaginationParams) -> QueryResult<Vec<StatsGroup>> {
    use crate::schema::games_statsgroups::dsl::*;
    use crate::schema::statsgroups::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    let sg_ids: Vec<Uuid> = 
        games_statsgroups
            .filter(gameid.eq(game_id))
            .load::<GameStatsGroup>(db)
            .unwrap()
            .iter()
            .map(|gsg| gsg.statsgroupid)
            .collect();

    statsgroups
        .filter(sgid.eq_any(sg_ids))
        .order(sgid.asc())
        .limit(page_size)
        .offset(offset_val)
        .load::<StatsGroup>(db)
}

pub fn update(db: &mut database::Connection, sg_id: Uuid, item: &StatsGroupChangeset) -> QueryResult<StatsGroup> {
    use crate::schema::statsgroups::dsl::*;
    diesel::update(statsgroups.filter(sgid.eq(sg_id)))
        .set((
            item,
            updated_at.eq(diesel::dsl::now),
        ))
        .get_result(db)
}

pub fn delete(db: &mut database::Connection, sg_id: Uuid) -> QueryResult<usize> {
    use crate::schema::statsgroups::dsl::*;
    diesel::delete(statsgroups.filter(sgid.eq(sg_id))).execute(db)
}
