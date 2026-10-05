use strum::IntoEnumIterator;
use uuid::Uuid;
use chrono::Utc;

use crate::{database, models::{permission::{AppAction, AppResource, AppSpecialPermission, PermissionBuilder}, role::{AppRole, RoleBuilder}, role_permission::RolePermissionBuilder, user::UserBuilder, users_roles::UsersRolesBuilder}};


pub fn default_password() -> String {
    std::env::var("SEED_DATA_COMMON_PASSWORD")
        .unwrap_or_else(|_| "SeedDefault1!".to_string())
}

pub fn insert_system_default_data(db: &mut database::Connection) {
    let start_time_for_db_pop_system_default_data = Utc::now();
    println!("Starting DB Data Population for System Default Data");
    
    init_roles_and_permissions(db);
    add_super_user(db);

    let end_time_for_db_pop_system_default_data = Utc::now();
    let duration_for_db_pop_system_default_data = end_time_for_db_pop_system_default_data.naive_utc() - start_time_for_db_pop_system_default_data.naive_utc();
    println!("Done. DB System Default Data Population Time Duration: {}\n", duration_for_db_pop_system_default_data);
}

fn add_super_user(db: &mut database::Connection) {
    let superuser_fname = match std::env::var("SUPERUSER_FNAME") {
        Ok(p) => p.to_string(),
        Err(_) => "Super".to_string()
    };
    let superuser_lname = match std::env::var("SUPERUSER_LNAME") {
        Ok(p) => p.to_string(),
        Err(_) => "User".to_string()
    };
    let superuser_username = match std::env::var("SUPERUSER_USERNAME") {
        Ok(p) => p.to_string(),
        Err(_) => "superuser".to_string()
    };
    let superuser_pwd = match std::env::var("SUPERUSER_PWD") {
        Ok(p) => p.to_string(),
        Err(_) => default_password().to_string()
    };
    let superuser_email = match std::env::var("SUPERUSER_EMAIL") {
        Ok(p) => p.to_string(),
        Err(_) => "superuser@fakeemail.com".to_string()
    };
    let super_user = UserBuilder::new(&superuser_fname)
        .set_lname(&superuser_lname)
        .set_username(&superuser_username)
        .set_hash_password(&superuser_pwd)
        .set_email(&superuser_email)
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
///  tournament_manager → tournament:create/update/delete, plus the tournament views and the
///                       participant user scope (member covers :read)
///  tournament_admin   → the same minus tournament:create and the participant user scope
///  super_user         → every permission, including `system:administer`
///
/// Authorization never looks at a role name; the role rows exist only to carry these grants.
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

    // ── Permissions that are not a plain resource × action pair ───────────────
    let mut special_ids: HashMap<&str, Uuid> = HashMap::new();
    for special in AppSpecialPermission::iter() {
        let perm = PermissionBuilder::new(special.as_str())
            .resource(special.resource())
            .action(special.action())
            .build_and_insert(db)
            .unwrap();
        special_ids.insert(special.as_str(), perm.id);
    }
    let special = |p: AppSpecialPermission| *special_ids.get(p.as_str()).unwrap();

    let read_ids: Vec<Uuid> = AppResource::iter()
        .map(|r| *perm_ids.get(&(r.as_str(), AppAction::Read.as_str())).unwrap())
        .collect();

    let all_ids: Vec<Uuid> = perm_ids.values().copied().chain(special_ids.values().copied()).collect();

    // The tournament sections that used to be gated on holding the manager/admin role.
    let tour_view_permissions: Vec<Uuid> = vec![
        special(AppSpecialPermission::TournamentReadPairingCode),
        special(AppSpecialPermission::TournamentReadAudit),
        special(AppSpecialPermission::TournamentUpdateSchedule),
    ];

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
        .chain(tour_view_permissions.iter().copied())
        .collect();

    // Managers additionally see the participants of the tournaments they manage.
    let tour_manager_permissions: Vec<Uuid> = tour_manager_permissions
        .into_iter()
        .chain(tour_view_permissions.iter().copied())
        .chain([special(AppSpecialPermission::UserReadTournamentParticipants)])
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
