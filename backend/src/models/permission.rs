use crate::database;
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult, Insertable, Identifiable};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

/// Resources that can be the subject of a permission.
#[derive(EnumIter)]
pub enum AppResource {
    Tournament,
    Division,
    Round,
    Room,
    RoomMonitor,
    Game,
    Team,
    User,
}

impl AppResource {
    pub fn as_str(&self) -> &'static str {
        match self {
            AppResource::Tournament   => "tournament",
            AppResource::Division     => "division",
            AppResource::Round        => "round",
            AppResource::Room         => "room",
            AppResource::RoomMonitor  => "roommonitor",
            AppResource::Game         => "game",
            AppResource::Team         => "team",
            AppResource::User         => "user",
        }
    }
}

/// CRUD actions that can be applied to an `AppResource`.
#[derive(EnumIter)]
pub enum AppAction {
    Create,
    Read,
    Update,
    Delete,
}

impl AppAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            AppAction::Create => "create",
            AppAction::Read   => "read",
            AppAction::Update => "update",
            AppAction::Delete => "delete",
        }
    }
}

/// Builder for a `Permission`. The `resource` + `action` fields provide
/// structured decomposition of the `name` (e.g. `name="post:create"`,
/// `resource="post"`, `action="create"`), enabling fine-grained permission
/// tree queries grouped by resource.
///
/// # Example
/// ```rust
/// let perm = PermissionBuilder::new("post:create")
///     .resource("post")
///     .action("create")
///     .build_and_insert(&mut conn)?;
/// ```
pub struct PermissionBuilder {
    name: String,
    resource: Option<String>,
    action: Option<String>,
}

impl PermissionBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            resource: None,
            action: None,
        }
    }

    pub fn resource(mut self, resource: &str) -> Self {
        self.resource = Some(resource.to_string());
        self
    }

    pub fn action(mut self, action: &str) -> Self {
        self.action = Some(action.to_string());
        self
    }

    fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if self.name.trim().is_empty() {
            errors.push("name is required".to_string());
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }

    pub fn build(self) -> Result<NewPermission, Vec<String>> {
        self.validate()?;
        Ok(NewPermission {
            name: self.name,
            resource: self.resource,
            action: self.action,
        })
    }

    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<Permission> {
        let new_perm = self.build().map_err(|errs| {
            diesel::result::Error::QueryBuilderError(errs.join(", ").into())
        })?;
        create(db, new_perm)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Identifiable, Selectable, ToSchema)]
#[diesel(table_name = crate::schema::permissions)]
#[diesel(primary_key(id))]
pub struct Permission {
    pub id: i32,
    pub name: String,
    pub resource: Option<String>,
    pub action: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::permissions)]
pub struct NewPermission {
    pub name: String,
    pub resource: Option<String>,
    pub action: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::permissions)]
pub struct PermissionChangeset {
    pub name: Option<String>,
    pub resource: Option<String>,
    pub action: Option<String>,
}

pub fn create(db: &mut database::Connection, item: NewPermission) -> QueryResult<Permission> {
    use crate::schema::permissions::dsl::*;
    insert_into(permissions).values(item).get_result::<Permission>(db)
}

pub fn exists(db: &mut database::Connection, item_id: i32) -> bool {
    use crate::schema::permissions::dsl::permissions;
    permissions.find(item_id).get_result::<Permission>(db).is_ok()
}

pub fn read(db: &mut database::Connection, item_id: i32) -> QueryResult<Permission> {
    use crate::schema::permissions::dsl::*;
    permissions.filter(id.eq(item_id)).first::<Permission>(db)
}

pub fn read_by_name(db: &mut database::Connection, perm_name: &str) -> QueryResult<Permission> {
    use crate::schema::permissions::dsl::*;
    permissions.filter(name.eq(perm_name)).first::<Permission>(db)
}

pub fn read_all(db: &mut database::Connection) -> QueryResult<Vec<Permission>> {
    use crate::schema::permissions::dsl::*;
    permissions.order(name.asc()).load::<Permission>(db)
}

pub fn read_all_for_resource(db: &mut database::Connection, res: &str) -> QueryResult<Vec<Permission>> {
    use crate::schema::permissions::dsl::*;
    permissions
        .filter(resource.eq(res))
        .order(action.asc())
        .load::<Permission>(db)
}

pub fn update(db: &mut database::Connection, item_id: i32, item: &PermissionChangeset) -> QueryResult<Permission> {
    use crate::schema::permissions::dsl::*;
    diesel::update(permissions.filter(id.eq(item_id)))
        .set((item, updated_at.eq(diesel::dsl::now)))
        .get_result(db)
}

pub fn delete(db: &mut database::Connection, item_id: i32) -> QueryResult<usize> {
    use crate::schema::permissions::dsl::*;
    diesel::delete(permissions.filter(id.eq(item_id))).execute(db)
}
