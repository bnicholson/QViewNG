use backend::{database, models::{tournament::Tournament, tournament_admin::{NewTournamentAdmin, TournamentAdmin, TournamentAdminBuilder}, user::User}};

use crate::fixtures;

pub fn get_tour_admin_payload_singular(db: &mut database::Connection) 
    -> (Tournament, User, NewTournamentAdmin) {
    
    let tournament = fixtures::tournaments::seed_tournament(db, "Test Tour");
    let user = fixtures::users::seed_user(db);
    let new_tour_admin = TournamentAdminBuilder::new_default(tournament.tid, user.id)
        .set_role_description("default role (test id 334)")
        .build()
        .unwrap();
    
    (tournament, user, new_tour_admin)
}

pub fn arrange_update_admin_works_integration_test(db: &mut database::Connection) 
    -> (Tournament, User, TournamentAdmin) {
    
    let tournament = fixtures::tournaments::seed_tournament(db, "Test Tour");
    let user = fixtures::users::seed_user(db);
    let tour_admin = TournamentAdminBuilder::new_default(tournament.tid, user.id)
        .build_and_insert(db)
        .unwrap();
    
    (tournament, user, tour_admin)
}
