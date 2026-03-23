use crate::{database, models};
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult, Insertable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Builder for assigning one or more roles to a user.
///
/// Combine with `RoleBuilder` / `RolePermissionBuilder` to compose
/// a full RBAC tree in code:
///
/// ```rust
/// UsersRolesBuilder::new(user_id)
///     .assign(admin_role_id)
///     .assign(editor_role_id)
///     .build_and_insert(&mut conn)?;
/// ```
pub struct UsersRolesBuilder {
    user_id: Uuid,
    role_ids: Vec<i32>,
}

impl UsersRolesBuilder {
    pub fn new(user_id: Uuid) -> Self {
        Self { user_id, role_ids: vec![] }
    }

    /// Assign a role to this user.
    pub fn assign(mut self, role_id: i32) -> Self {
        self.role_ids.push(role_id);
        self
    }

    /// Assign multiple roles at once.
    pub fn assign_many(mut self, role_ids: impl IntoIterator<Item = i32>) -> Self {
        self.role_ids.extend(role_ids);
        self
    }

    fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if self.role_ids.is_empty() {
            errors.push("at least one role_id is required".to_string());
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }

    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<Vec<UsersRole>> {
        self.validate().map_err(|errs| {
            diesel::result::Error::QueryBuilderError(errs.join(", ").into())
        })?;

        if !models::user::exists(db, self.user_id) {
            return Err(diesel::result::Error::QueryBuilderError(
                format!("User with id {} does not exist", self.user_id).into()
            ));
        }

        let mut inserted = Vec::new();
        for rid in self.role_ids {
            if !models::role::exists(db, rid) {
                return Err(diesel::result::Error::QueryBuilderError(
                    format!("Role with id {} does not exist", rid).into()
                ));
            }
            let record = create(db, NewUsersRole { user_id: self.user_id, role_id: rid })?;
            inserted.push(record);
        }
        Ok(inserted)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Selectable, ToSchema)]
#[diesel(table_name = crate::schema::users_roles)]
#[diesel(primary_key(user_id, role_id))]
pub struct UsersRole {
    pub user_id: Uuid,
    pub role_id: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::users_roles)]
pub struct NewUsersRole {
    pub user_id: Uuid,
    pub role_id: i32,
}

pub fn create(db: &mut database::Connection, item: NewUsersRole) -> QueryResult<UsersRole> {
    use crate::schema::users_roles::dsl::*;
    insert_into(users_roles)
        .values(item)
        .get_result::<UsersRole>(db)
}

pub fn read_all_for_user(db: &mut database::Connection, uid: Uuid) -> QueryResult<Vec<UsersRole>> {
    use crate::schema::users_roles::dsl::*;
    users_roles.filter(user_id.eq(uid)).load::<UsersRole>(db)
}

pub fn read_all_for_role(db: &mut database::Connection, rid: i32) -> QueryResult<Vec<UsersRole>> {
    use crate::schema::users_roles::dsl::*;
    users_roles.filter(role_id.eq(rid)).load::<UsersRole>(db)
}

pub fn delete(db: &mut database::Connection, uid: Uuid, rid: i32) -> QueryResult<usize> {
    use crate::schema::users_roles::dsl::*;
    diesel::delete(
        users_roles
            .filter(user_id.eq(uid))
            .filter(role_id.eq(rid))
    ).execute(db)
}

pub fn delete_all_for_user(db: &mut database::Connection, uid: Uuid) -> QueryResult<usize> {
    use crate::schema::users_roles::dsl::*;
    diesel::delete(users_roles.filter(user_id.eq(uid))).execute(db)
}
