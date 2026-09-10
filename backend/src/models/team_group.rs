use crate::database;
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult, AsChangeset, Insertable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

pub struct TeamGroupBuilder {
    division_session_id: Uuid,
    name: Option<String>,
    type_: String,
    creator_userid: Option<Uuid>,
    last_modified_userid: Option<Uuid>,
}

impl TeamGroupBuilder {
    pub fn new(division_session_id: Uuid) -> Self {
        Self {
            division_session_id,
            name: None,
            type_: "pool".to_string(),
            creator_userid: None,
            last_modified_userid: None,
        }
    }
    pub fn new_default(division_session_id: Uuid) -> Self {
        Self::new(division_session_id)
    }
    pub fn set_division_session_id(mut self, id: Uuid) -> Self {
        self.division_session_id = id;
        self
    }
    pub fn set_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
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
        let mut errors = Vec::new();
        if self.name.is_none() { errors.push("name is required".to_string()); }
        if self.creator_userid.is_none() { errors.push("creator_userid is required".to_string()); }
        if !errors.is_empty() { return Err(errors); }
        let creator = self.creator_userid.unwrap();
        Ok(NewTeamGroup {
            division_session_id: self.division_session_id,
            name: self.name.unwrap(),
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
#[diesel(table_name = crate::schema::team_groups)]
#[diesel(primary_key(team_group_id))]
pub struct TeamGroup {
    pub team_group_id: Uuid,                  // identifies the team group uniquely
    pub division_session_id: Uuid,            // parent division session
    #[diesel(column_name = type_)]
    #[serde(rename = "type")]
    pub type_: String,                        // grouping type (e.g. "pool"); required, defaults to "pool"
    pub created_date: DateTime<Utc>,
    pub creator_userid: Uuid,
    pub last_modified_date: DateTime<Utc>,
    pub last_modified_userid: Uuid,
    pub name: String,                         // unique within the parent division session
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::team_groups)]
pub struct NewTeamGroup {
    pub division_session_id: Uuid,
    pub name: String,
    #[diesel(column_name = type_)]
    #[serde(rename = "type")]
    pub type_: String,
    pub creator_userid: Uuid,
    pub last_modified_userid: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::team_groups)]
#[diesel(primary_key(team_group_id))]
pub struct TeamGroupChangeset {
    pub division_session_id: Option<Uuid>,
    pub name: Option<String>,
    #[diesel(column_name = type_)]
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

/// Whether a team group named `name_val` already exists in division session `session_id`. When
/// `exclude` is set (e.g. during an update), that group id is ignored so a row doesn't clash with
/// itself.
pub fn name_exists_in_division_session(
    db: &mut database::Connection,
    session_id: Uuid,
    name_val: &str,
    exclude: Option<Uuid>,
) -> QueryResult<bool> {
    use crate::schema::team_groups::dsl::*;
    let mut query = team_groups
        .filter(division_session_id.eq(session_id))
        .filter(name.eq(name_val))
        .into_boxed();
    if let Some(ex) = exclude {
        query = query.filter(team_group_id.ne(ex));
    }
    let count: i64 = query.count().get_result(db)?;
    Ok(count > 0)
}

pub fn create(db: &mut database::Connection, item: &NewTeamGroup) -> QueryResult<TeamGroup> {
    // A team group's name must be unique within its parent division session.
    if name_exists_in_division_session(db, item.division_session_id, &item.name, None)? {
        return Err(diesel::result::Error::QueryBuilderError(
            format!("A team group named \"{}\" already exists in this division session.", item.name).into()
        ));
    }
    use crate::schema::team_groups::dsl::*;
    insert_into(team_groups).values(item).get_result::<TeamGroup>(db)
}

pub fn exists(db: &mut database::Connection, item_id: Uuid) -> bool {
    use crate::schema::team_groups::dsl::*;
    team_groups.find(item_id).get_result::<TeamGroup>(db).is_ok()
}

pub fn read(db: &mut database::Connection, item_id: Uuid) -> QueryResult<TeamGroup> {
    use crate::schema::team_groups::dsl::*;
    team_groups.filter(team_group_id.eq(item_id)).first::<TeamGroup>(db)
}

pub fn read_all(db: &mut database::Connection) -> QueryResult<Vec<TeamGroup>> {
    use crate::schema::team_groups::dsl::*;
    team_groups.order(created_date).load::<TeamGroup>(db)
}

/// All team groups belonging to the given division session.
pub fn read_all_of_division_session(db: &mut database::Connection, session_id: Uuid) -> QueryResult<Vec<TeamGroup>> {
    use crate::schema::team_groups::dsl::*;
    team_groups.filter(division_session_id.eq(session_id)).order(created_date).load::<TeamGroup>(db)
}

pub fn update(db: &mut database::Connection, item_id: Uuid, item: &TeamGroupChangeset, modified_by: Uuid) -> QueryResult<TeamGroup> {
    // Enforce name uniqueness within the (possibly changed) parent division session.
    let existing = read(db, item_id)?;
    let effective_session = item.division_session_id.unwrap_or(existing.division_session_id);
    let effective_name = item.name.clone().unwrap_or(existing.name.clone());
    if name_exists_in_division_session(db, effective_session, &effective_name, Some(item_id))? {
        return Err(diesel::result::Error::QueryBuilderError(
            format!("A team group named \"{}\" already exists in this division session.", effective_name).into()
        ));
    }
    use crate::schema::team_groups::dsl::*;
    diesel::update(team_groups.filter(team_group_id.eq(item_id)))
        .set((
            item,
            last_modified_date.eq(diesel::dsl::now),
            last_modified_userid.eq(modified_by),
        ))
        .get_result(db)
}

pub fn delete(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::team_groups::dsl::*;
    diesel::delete(team_groups.filter(team_group_id.eq(item_id))).execute(db)
}
