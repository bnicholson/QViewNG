use backend::{database, models::permission::{Permission, PermissionBuilder}};

pub fn seed_permission(db: &mut database::Connection, name: &str, resource: &str, action: &str) -> Permission {
    PermissionBuilder::new(name)
        .resource(resource)
        .action(action)
        .build_and_insert(db)
        .unwrap()
}

pub fn seed_permissions(db: &mut database::Connection) -> Vec<Permission> {
    vec![
        seed_permission(db, "post:create", "post", "create"),
        seed_permission(db, "post:read",   "post", "read"),
        seed_permission(db, "post:update", "post", "update"),
        seed_permission(db, "post:delete", "post", "delete"),
        seed_permission(db, "user:read",   "user", "read"),
        seed_permission(db, "user:delete", "user", "delete"),
    ]
}
