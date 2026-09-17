use crate::database;
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult, AsChangeset, Insertable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

pub struct TeamGroupBuilder {
    type_: String,
    creator_userid: Option<Uuid>,
    last_modified_userid: Option<Uuid>,
}

impl TeamGroupBuilder {
    pub fn new() -> Self {
        Self {
            type_: "pool".to_string(),
            creator_userid: None,
            last_modified_userid: None,
        }
    }
    pub fn new_default() -> Self {
        Self::new()
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
            type_: self.type_,
            creator_userid: creator,
            last_modified_userid: self.last_modified_userid.unwrap_or(creator),
        })
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<TeamGroup> {
        create(db, &self.build().unwrap())
    }
}

impl Default for TeamGroupBuilder {
    fn default() -> Self {
        Self::new()
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
    #[diesel(column_name = type_)]
    #[serde(rename = "type")]
    pub type_: String,                        // grouping type (e.g. "pool"); required, defaults to "pool"
    pub created_date: DateTime<Utc>,
    pub creator_userid: Uuid,
    pub last_modified_date: DateTime<Utc>,
    pub last_modified_userid: Uuid,
    pub del_fl: bool,                         // soft-delete flag
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::teamgroups)]
pub struct NewTeamGroup {
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
    teamgroups.find(item_id).filter(del_fl.eq(false)).get_result::<TeamGroup>(db).is_ok()
}

pub fn read(db: &mut database::Connection, item_id: Uuid) -> QueryResult<TeamGroup> {
    use crate::schema::teamgroups::dsl::*;
    teamgroups.filter(team_group_id.eq(item_id)).filter(del_fl.eq(false)).first::<TeamGroup>(db)
}

/// Read ignoring the soft-delete flag — used by purge, which must resolve even a soft-deleted row.
pub fn read_including_deleted(db: &mut database::Connection, item_id: Uuid) -> QueryResult<TeamGroup> {
    use crate::schema::teamgroups::dsl::*;
    teamgroups.filter(team_group_id.eq(item_id)).first::<TeamGroup>(db)
}

pub fn read_all(db: &mut database::Connection) -> QueryResult<Vec<TeamGroup>> {
    use crate::schema::teamgroups::dsl::*;
    teamgroups.filter(del_fl.eq(false)).order(created_date).load::<TeamGroup>(db)
}

/// The single team group belonging to the given pool bracket (they are one-to-one). The bracket now
/// owns the FK (`pool_brackets.team_group_id`), so resolve the group through it. Returns NotFound
/// when the bracket has no team group yet, or when its team group is soft-deleted.
pub fn read_of_pool_bracket(db: &mut database::Connection, bracket_id: Uuid) -> QueryResult<TeamGroup> {
    let bracket = crate::models::pool_bracket::read(db, bracket_id)?;
    match bracket.team_group_id {
        Some(tgid) => read(db, tgid),
        None => Err(diesel::result::Error::NotFound),
    }
}

/// Returns the pool bracket's 1-to-1 team group, creating it (inheriting the bracket's `type`) and
/// attaching it to the bracket if it doesn't exist yet. Used when associating the first team with a
/// bracket.
pub fn resolve_or_create_for_pool_bracket(
    db: &mut database::Connection,
    bracket_id: Uuid,
    user_id: Uuid,
) -> QueryResult<TeamGroup> {
    let bracket = crate::models::pool_bracket::read(db, bracket_id)?;
    if let Some(tgid) = bracket.team_group_id {
        if let Ok(existing) = read(db, tgid) {
            return Ok(existing);
        }
    }
    let group = create(db, &NewTeamGroup {
        type_: bracket.type_,
        creator_userid: user_id,
        last_modified_userid: user_id,
    })?;
    crate::models::pool_bracket::set_team_group_id(db, bracket_id, group.team_group_id)?;
    Ok(group)
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

/// Soft delete: hide the team group by setting its `del_fl`.
pub fn delete(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::teamgroups::dsl::*;
    diesel::update(teamgroups.filter(team_group_id.eq(item_id)))
        .set(del_fl.eq(true))
        .execute(db)
}

/// Purge: permanently remove the team group row from the database.
pub fn purge(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::teamgroups::dsl::*;
    diesel::delete(teamgroups.filter(team_group_id.eq(item_id))).execute(db)
}
