use strum::IntoEnumIterator;
use uuid::Uuid;

use crate::{database, models::{permission::{AppAction, AppResource, PermissionBuilder}, role::{AppRole, RoleBuilder}, role_permission::RolePermissionBuilder, user::UserBuilder, users_roles::UsersRolesBuilder}};


pub const DEFAULT_PASSWORD: &str = "Password123!";

pub fn insert_system_default_data(db: &mut database::Connection) {
    init_roles_and_permissions(db);
    add_super_user(db);
}

fn add_super_user(db: &mut database::Connection) {
    let super_user = UserBuilder::new("Super")
        .set_lname("User")
        .set_username("superuser")
        .set_hash_password(DEFAULT_PASSWORD)
        .set_email("superuser@fakeemail.com")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();

    let member_role    = crate::models::role::read_by_name(db, AppRole::Member.as_str()).unwrap();
    let super_user_role = crate::models::role::read_by_name(db, AppRole::SuperUser.as_str()).unwrap();

    UsersRolesBuilder::new(super_user.id)
        .assign(member_role.id)
        .assign(super_user_role.id)
        .build_and_insert(db)
        .unwrap();
}

/// Idempotent baseline setup — creates all canonical permissions and the three
/// application roles. Does NOT assign any roles to users; call separately after
/// inserting users (e.g. via `UsersRolesBuilder`).
///
/// Role permission tree (each role is self-contained):
///
///  member             → :read on every resource
///  tournament_manager → tournament:create/update/delete only (member covers :read)
///  super_user         → full CRUD on every resource
fn init_roles_and_permissions(db: &mut database::Connection) {
    use std::collections::HashMap;

    // ── Build one permission row per AppResource × AppAction variant ──────────
    let mut perm_ids: HashMap<(&str, &str), Uuid> = HashMap::new();

    for resource in AppResource::iter() {
        for action in AppAction::iter() {
            let perm = PermissionBuilder::new(&format!("{}:{}", resource.as_str(), action.as_str()))
                .resource(resource.as_str())
                .action(action.as_str())
                .build_and_insert(db)
                .unwrap();
            perm_ids.insert((resource.as_str(), action.as_str()), perm.id);
        }
    }

    let read_ids: Vec<Uuid> = AppResource::iter()
        .map(|r| *perm_ids.get(&(r.as_str(), AppAction::Read.as_str())).unwrap())
        .collect();

    let all_ids: Vec<Uuid> = perm_ids.values().copied().collect();

    let tour_group_update_id = perm_ids.get(&(AppResource::TournamentGroup.as_str(), AppAction::Update.as_str())).copied();
    let tour_manager_permissions: Vec<Uuid> = [
            AppResource::Tournament,
            AppResource::TournamentGroup,
            AppResource::Division,
            AppResource::Round,
            AppResource::Room,
            AppResource::Game,
            AppResource::Team,
        ].iter().flat_map(|resource| {
            [
                AppAction::Create.as_str(),
                AppAction::Update.as_str(),
                AppAction::Delete.as_str(),
            ].iter().filter_map(|action| {
                perm_ids.get(&(resource.as_str(), *action)).copied()
            }).collect::<Vec<_>>()
        })
        .filter(|id| Some(*id) != tour_group_update_id)
        .collect();

    let tour_create_id = perm_ids.get(&(AppResource::Tournament.as_str(), AppAction::Create.as_str())).copied();
    let tour_admin_permissions: Vec<Uuid> = tour_manager_permissions
        .iter()
        .copied()
        .filter(|id| Some(*id) != tour_create_id)
        .collect();

    // ── Build one role row per AppRole variant ────────────────────────────────
    for app_role in AppRole::iter() {
        let role = RoleBuilder::new(app_role.as_str())
            .description(app_role.description())
            .build_and_insert(db)
            .unwrap();

        let role_perm_ids: Vec<Uuid> = match app_role {
            // member: read-only on every resource
            AppRole::Member => read_ids.clone(),
            // tournament_manager: create/update/delete on all tournament-managed resources (including TournamentGroups)
            AppRole::TournamentManager => tour_manager_permissions.clone(),
            AppRole::TournamentAdmin => tour_admin_permissions.clone(),
            // super_user: full CRUD on everything
            AppRole::SuperUser => all_ids.clone(),
        };

        RolePermissionBuilder::new(role.id)
            .add_many(role_perm_ids)
            .build_and_insert(db)
            .unwrap();
    }
}
