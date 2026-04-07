use backend::{database, models::{create_tournament_applicant::{CreateTournamentApplicant, CreateTournamentApplicantBuilder}, user::{User, UserBuilder}}};

pub fn seed_user(db: &mut database::Connection) -> User {
    UserBuilder::new_default("Applicant User")
        .set_hash_password("ApplicantPwd123!")
        .build_and_insert(db)
        .unwrap()
}

pub fn arrange_create_works_integration_test(db: &mut database::Connection) -> (backend::models::create_tournament_applicant::NewCreateTournamentApplicant, User) {
    let user = seed_user(db);
    let payload = CreateTournamentApplicantBuilder::new_default(user.id, user.id)
        .set_request_context(Some("I would like to create a tournament for my region.".to_string()))
        .build();
    (payload, user)
}

pub fn arrange_get_all_works_integration_test(db: &mut database::Connection) -> (CreateTournamentApplicant, CreateTournamentApplicant) {
    let user = seed_user(db);
    let item_1 = CreateTournamentApplicantBuilder::new_default(user.id, user.id)
        .set_request_context(Some("First applicant request.".to_string()))
        .build_and_insert(db)
        .unwrap();
    let item_2 = CreateTournamentApplicantBuilder::new(user.id, "approved", user.id)
        .set_request_context(Some("Second applicant request.".to_string()))
        .build_and_insert(db)
        .unwrap();
    (item_1, item_2)
}

pub fn arrange_get_by_id_works_integration_test(db: &mut database::Connection) -> CreateTournamentApplicant {
    let user = seed_user(db);
    CreateTournamentApplicantBuilder::new_default(user.id, user.id)
        .set_request_context(Some("Get by ID test request.".to_string()))
        .build_and_insert(db)
        .unwrap()
}

pub fn arrange_update_works_integration_test(db: &mut database::Connection) -> (CreateTournamentApplicant, User) {
    let user = seed_user(db);
    let item = CreateTournamentApplicantBuilder::new_default(user.id, user.id)
        .set_request_context(Some("Update test request.".to_string()))
        .build_and_insert(db)
        .unwrap();
    (item, user)
}

pub fn arrange_delete_works_integration_test(db: &mut database::Connection) -> CreateTournamentApplicant {
    let user = seed_user(db);
    CreateTournamentApplicantBuilder::new_default(user.id, user.id)
        .set_request_context(Some("Delete test request.".to_string()))
        .build_and_insert(db)
        .unwrap()
}
