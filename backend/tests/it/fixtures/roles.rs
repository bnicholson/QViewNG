use backend::{database, models::role::{Role, RoleBuilder}};

pub fn seed_role(db: &mut database::Connection, name: &str) -> Role {
    RoleBuilder::new(name)
        .description(&format!("Description for {}", name))
        .build_and_insert(db)
        .unwrap()
}

pub fn seed_roles(db: &mut database::Connection) -> Vec<Role> {
    vec![
        seed_role(db, "admin"),
        seed_role(db, "editor"),
        seed_role(db, "viewer"),
    ]
}
