use crate::database;
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult, AsChangeset, Insertable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

pub struct TeamGroupBuilder {
    pool_bracket_id: Uuid,
    type_: String,
    creator_userid: Option<Uuid>,
    last_modified_userid: Option<Uuid>,
}

impl TeamGroupBuilder {
    pub fn new(pool_bracket_id: Uuid) -> Self {
        Self {
            pool_bracket_id,
            type_: "pool".to_string(),
            creator_userid: None,
            last_modified_userid: None,
        }
    }
    pub fn new_default(pool_bracket_id: Uuid) -> Self {
        Self::new(pool_bracket_id)
    }
    pub fn set_pool_bracket_id(mut self, id: Uuid) -> Self {
        self.pool_bracket_id = id;
        self
    }
    pub fn set_type(mut self, type_: &str) -> Self {
        self.type_ = type_.to_string();
        self
    }
    pub fn set_creator_userid(mut self, user_id: Uuid) -> Self {
        self.creator_userid = Some(user_id);
        self
    }
    pub fn set_last_modified_userid(mut self, user_id: Uuid) -> Self {
        self.last_modified_userid = Some(user_id);
        self
    }
    pub fn build(self) -> Result<NewTeamGroup, Vec<String>> {
        let creator = match self.creator_userid {
            Some(c) => c,
            None => return Err(vec!["creator_userid is required".to_string()]),
        };
        Ok(NewTeamGroup {
            pool_bracket_id: self.pool_bracket_id,
            type_: self.type_,
            creator_userid: creator,
            last_modified_userid: self.last_modified_userid.unwrap_or(creator),
        })
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<TeamGroup> {
        create(db, &self.build().unwrap())
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
#[diesel(table_name = crate::schema::teamgroups)]
#[diesel(primary_key(team_group_id))]
pub struct TeamGroup {
    pub team_group_id: Uuid,                  // identifies the team group uniquely
    pub pool_bracket_id: Uuid,                // parent pool bracket (one-to-one)
    #[diesel(column_name = type_)]
    #[serde(rename = "type")]
    pub type_: String,                        // grouping type (e.g. "pool"); required, defaults to "pool"
    pub created_date: DateTime<Utc>,
    pub creator_userid: Uuid,
    pub last_modified_date: DateTime<Utc>,
    pub last_modified_userid: Uuid,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::teamgroups)]
pub struct NewTeamGroup {
    pub pool_bracket_id: Uuid,
    #[diesel(column_name = type_)]
    #[serde(rename = "type")]
    pub type_: String,
    pub creator_userid: Uuid,
    pub last_modified_userid: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::teamgroups)]
#[diesel(primary_key(team_group_id))]
pub struct TeamGroupChangeset {
    pub pool_bracket_id: Option<Uuid>,
    #[diesel(column_name = type_)]
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

pub fn create(db: &mut database::Connection, item: &NewTeamGroup) -> QueryResult<TeamGroup> {
    use crate::schema::teamgroups::dsl::*;
    insert_into(teamgroups).values(item).get_result::<TeamGroup>(db)
}

pub fn exists(db: &mut database::Connection, item_id: Uuid) -> bool {
    use crate::schema::teamgroups::dsl::*;
    teamgroups.find(item_id).get_result::<TeamGroup>(db).is_ok()
}

pub fn read(db: &mut database::Connection, item_id: Uuid) -> QueryResult<TeamGroup> {
    use crate::schema::teamgroups::dsl::*;
    teamgroups.filter(team_group_id.eq(item_id)).first::<TeamGroup>(db)
}

pub fn read_all(db: &mut database::Connection) -> QueryResult<Vec<TeamGroup>> {
    use crate::schema::teamgroups::dsl::*;
    teamgroups.order(created_date).load::<TeamGroup>(db)
}

/// The single team group belonging to the given pool bracket (they are one-to-one).
pub fn read_of_pool_bracket(db: &mut database::Connection, bracket_id: Uuid) -> QueryResult<TeamGroup> {
    use crate::schema::teamgroups::dsl::*;
    teamgroups.filter(pool_bracket_id.eq(bracket_id)).first::<TeamGroup>(db)
}

pub fn update(db: &mut database::Connection, item_id: Uuid, item: &TeamGroupChangeset, modified_by: Uuid) -> QueryResult<TeamGroup> {
    use crate::schema::teamgroups::dsl::*;
    diesel::update(teamgroups.filter(team_group_id.eq(item_id)))
        .set((
            item,
            last_modified_date.eq(diesel::dsl::now),
            last_modified_userid.eq(modified_by),
        ))
        .get_result(db)
}

pub fn delete(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::teamgroups::dsl::*;
    diesel::delete(teamgroups.filter(team_group_id.eq(item_id))).execute(db)
}
