use crate::{database, models};
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult, Insertable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

/// Builder for linking one role to one or many permissions.
///
/// Combine multiple `RolePermissionBuilder` values (one per role) to
/// assemble a complete permission tree across all roles:
///
/// ```rust
/// // Grant read + write to editor, read-only to viewer
/// RolePermissionBuilder::new(editor_id)
///     .add(read_id).add(write_id)
///     .build_and_insert(&mut conn)?;
///
/// RolePermissionBuilder::new(viewer_id)
///     .add(read_id)
///     .build_and_insert(&mut conn)?;
/// ```
pub struct RolePermissionBuilder {
    role_id: i32,
    permission_ids: Vec<i32>,
}

impl RolePermissionBuilder {
    pub fn new(role_id: i32) -> Self {
        Self { role_id, permission_ids: vec![] }
    }

    /// Add a permission to this role's tree.
    pub fn add(mut self, permission_id: i32) -> Self {
        self.permission_ids.push(permission_id);
        self
    }

    /// Add multiple permissions at once.
    pub fn add_many(mut self, permission_ids: impl IntoIterator<Item = i32>) -> Self {
        self.permission_ids.extend(permission_ids);
        self
    }

    fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if self.permission_ids.is_empty() {
            errors.push("at least one permission_id is required".to_string());
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }

    /// Insert all role↔permission associations, validating each entity exists.
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<Vec<RolePermission>> {
        self.validate().map_err(|errs| {
            diesel::result::Error::QueryBuilderError(errs.join(", ").into())
        })?;

        if !models::role::exists(db, self.role_id) {
            return Err(diesel::result::Error::QueryBuilderError(
                format!("Role with id {} does not exist", self.role_id).into()
            ));
        }

        let mut inserted = Vec::new();
        for pid in self.permission_ids {
            if !models::permission::exists(db, pid) {
                return Err(diesel::result::Error::QueryBuilderError(
                    format!("Permission with id {} does not exist", pid).into()
                ));
            }
            let record = create(db, NewRolePermission { role_id: self.role_id, permission_id: pid })?;
            inserted.push(record);
        }
        Ok(inserted)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Selectable, ToSchema)]
#[diesel(table_name = crate::schema::roles_permissions)]
#[diesel(primary_key(role_id, permission_id))]
pub struct RolePermission {
    pub role_id: i32,
    pub permission_id: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::roles_permissions)]
pub struct NewRolePermission {
    pub role_id: i32,
    pub permission_id: i32,
}

pub fn create(db: &mut database::Connection, item: NewRolePermission) -> QueryResult<RolePermission> {
    use crate::schema::roles_permissions::dsl::*;
    insert_into(roles_permissions)
        .values(item)
        .get_result::<RolePermission>(db)
}

pub fn read_all_for_role(db: &mut database::Connection, r_id: i32) -> QueryResult<Vec<RolePermission>> {
    use crate::schema::roles_permissions::dsl::*;
    roles_permissions
        .filter(role_id.eq(r_id))
        .load::<RolePermission>(db)
}

pub fn delete(db: &mut database::Connection, r_id: i32, p_id: i32) -> QueryResult<usize> {
    use crate::schema::roles_permissions::dsl::*;
    diesel::delete(
        roles_permissions
            .filter(role_id.eq(r_id))
            .filter(permission_id.eq(p_id))
    ).execute(db)
}

pub fn delete_all_for_role(db: &mut database::Connection, r_id: i32) -> QueryResult<usize> {
    use crate::schema::roles_permissions::dsl::*;
    diesel::delete(roles_permissions.filter(role_id.eq(r_id))).execute(db)
}
