use crate::database;
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult, Insertable, Identifiable};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Canonical application roles. Every role inserted into the `roles` table
/// must correspond to a variant here.
#[derive(EnumIter)]
pub enum AppRole {
    Member,
    TournamentManager,
    TournamentAdmin,
    SuperUser,
}

impl AppRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            AppRole::Member            => "member",
            AppRole::TournamentManager => "tournament_manager",
            AppRole::TournamentAdmin   => "tournament_admin",
            AppRole::SuperUser         => "super_user",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            AppRole::Member            => "View all resources; no write access",
            AppRole::TournamentManager => "Create, update, and delete tournaments (assign alongside member)",
            AppRole::TournamentAdmin   => "Tournament Manager except for creating Tournaments",
            AppRole::SuperUser         => "Unrestricted access to all resources and actions",
        }
    }
}

/// Builder for a `Role`. Supports composing a permission tree by accumulating
/// permission IDs that will be associated via `roles_permissions` on insert.
///
/// # Example
/// ```rust
/// let role = RoleBuilder::new("admin")
///     .description("Full administrator access")
///     .with_permission(read_perm_id)
///     .with_permission(write_perm_id)
///     .build_and_insert(&mut conn)?;
/// ```
pub struct RoleBuilder {
    name: String,
    description: Option<String>,
    permission_ids: Vec<Uuid>,
}

impl RoleBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: None,
            permission_ids: vec![],
        }
    }

    pub fn description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }

    /// Add a single permission to this role's permission tree.
    pub fn with_permission(mut self, permission_id: Uuid) -> Self {
        self.permission_ids.push(permission_id);
        self
    }

    /// Add multiple permissions at once (useful for composing pre-built permission sets).
    pub fn with_permissions(mut self, permission_ids: impl IntoIterator<Item = Uuid>) -> Self {
        self.permission_ids.extend(permission_ids);
        self
    }

    fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if self.name.trim().is_empty() {
            errors.push("name is required".to_string());
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }

    /// Build into a `(NewRole, Vec<permission_id>)` without touching the database.
    pub fn build(self) -> Result<(NewRole, Vec<Uuid>), Vec<String>> {
        self.validate()?;
        let permission_ids = self.permission_ids.clone();
        Ok((NewRole { name: self.name, description: self.description }, permission_ids))
    }

    /// Insert the role then link all accumulated permissions in one call.
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<Role> {
        let (new_role, perm_ids) = self.build().map_err(|errs| {
            diesel::result::Error::QueryBuilderError(errs.join(", ").into())
        })?;

        let role = create(db, new_role)?;

        for pid in perm_ids {
            use crate::schema::roles_permissions::dsl as rp;
            diesel::insert_into(rp::roles_permissions)
                .values((rp::role_id.eq(role.id), rp::permission_id.eq(pid)))
                .execute(db)?;
        }

        Ok(role)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Identifiable, Selectable, ToSchema)]
#[diesel(table_name = crate::schema::roles)]
#[diesel(primary_key(id))]
pub struct Role {
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub id: Uuid,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::roles)]
pub struct NewRole {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::roles)]
pub struct RoleChangeset {
    pub name: Option<String>,
    pub description: Option<String>,
}

pub fn create(db: &mut database::Connection, item: NewRole) -> QueryResult<Role> {
    use crate::schema::roles::dsl::*;
    insert_into(roles).values(item).get_result::<Role>(db)
}

pub fn exists(db: &mut database::Connection, item_id: Uuid) -> bool {
    use crate::schema::roles::dsl::roles;
    roles.find(item_id).get_result::<Role>(db).is_ok()
}

pub fn read(db: &mut database::Connection, item_id: Uuid) -> QueryResult<Role> {
    use crate::schema::roles::dsl::*;
    roles.filter(id.eq(item_id)).first::<Role>(db)
}

pub fn read_by_name(db: &mut database::Connection, role_name: &str) -> QueryResult<Role> {
    use crate::schema::roles::dsl::*;
    roles.filter(name.eq(role_name)).first::<Role>(db)
}

pub fn read_all(db: &mut database::Connection) -> QueryResult<Vec<Role>> {
    use crate::schema::roles::dsl::*;
    roles.order(name.asc()).load::<Role>(db)
}

pub fn count(db: &mut database::Connection) -> QueryResult<i64> {
    use crate::schema::roles::dsl::*;
    roles.count().get_result(db)
}

pub fn update(db: &mut database::Connection, item_id: Uuid, item: &RoleChangeset) -> QueryResult<Role> {
    use crate::schema::roles::dsl::*;
    diesel::update(roles.filter(id.eq(item_id)))
        .set((item, updated_at.eq(diesel::dsl::now)))
        .get_result(db)
}

pub fn delete(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::roles::dsl::*;
    diesel::delete(roles.filter(id.eq(item_id))).execute(db)
}
