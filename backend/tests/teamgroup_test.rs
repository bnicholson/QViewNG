
mod common;
mod fixtures;

use backend::database::Database;
use backend::models::division::DivisionBuilder;
use backend::models::division_session::DivisionSessionBuilder;
use backend::models::pool_bracket::PoolBracketBuilder;
use backend::models::teamgroup::{self, TeamGroupBuilder};
use backend::models::tournament::TournamentBuilder;
use backend::models::user::UserBuilder;
use crate::common::{TEST_DB_URL, clean_database};

/// teamgroups has no HTTP service, so its soft-delete/purge is exercised at the model level:
/// `delete` hides the row (del_fl = true) and `purge` removes it permanently.
#[actix_web::test]
async fn delete_soft_deletes_and_purge_removes() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let owner = UserBuilder::new_default("TG Owner").set_hash_password("Pwd123!").build_and_insert(&mut conn).unwrap();
    let tournament = TournamentBuilder::new_default("TG Tour").set_owner_id(owner.id).build_and_insert(&mut conn).unwrap();
    let division = DivisionBuilder::new_default("TG Div", tournament.tid).build_and_insert(&mut conn).unwrap();
    let session = DivisionSessionBuilder::new(division.did)
        .set_name("Pool Play").set_creator_userid(owner.id).build_and_insert(&mut conn).unwrap();
    let bracket = PoolBracketBuilder::new(session.did)
        .set_name("Pool A").set_type("pool").set_creator_userid(owner.id).build_and_insert(&mut conn).unwrap();
    let group = TeamGroupBuilder::new(bracket.pool_bracket_id)
        .set_creator_userid(owner.id).build_and_insert(&mut conn).unwrap();

    // ── delete() is a soft delete: hidden from reads, still present with del_fl = true. ──
    let affected = teamgroup::delete(&mut conn, group.team_group_id).unwrap();
    assert_eq!(affected, 1);
    assert!(teamgroup::read(&mut conn, group.team_group_id).is_err(), "soft-deleted teamgroup should be hidden from read");
    assert!(teamgroup::read_of_pool_bracket(&mut conn, bracket.pool_bracket_id).is_err(), "and hidden from read_of_pool_bracket");
    let raw = teamgroup::read_including_deleted(&mut conn, group.team_group_id).unwrap();
    assert!(raw.del_fl, "row should remain with del_fl = true");

    // ── purge() permanently removes the row. ──
    let purged = teamgroup::purge(&mut conn, group.team_group_id).unwrap();
    assert_eq!(purged, 1);
    assert!(teamgroup::read_including_deleted(&mut conn, group.team_group_id).is_err(), "purged teamgroup row should be gone");
}
