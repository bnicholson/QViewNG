use backend::{
    database, 
    models::{
        roster::{NewRoster, Roster, RosterBuilder}, roster_coach::{NewRosterCoach, RosterCoach, RosterCoachBuilder}, roster_quizzer::RosterQuizzerBuilder, user::{User, UserBuilder}
    }
};


pub fn arrange_create_works_integration_test(db: &mut database::Connection) -> NewRoster {
    let coach = UserBuilder::new_default("Coach 1")
        .set_hash_password("password123")
        .build_and_insert(db)
        .unwrap();
    RosterBuilder::new("Test Roster 2317", coach.id)
        .set_description(Some("Roster for integration test create.".to_string()))
        .build()
        .unwrap()
}

pub fn arrange_get_all_works_integration_test(db: &mut database::Connection) -> (Roster, Roster) {
    let coach = UserBuilder::new_default("Coach 1")
        .set_hash_password("password123")
        .build_and_insert(db)
        .unwrap();
    (
        RosterBuilder::new("Test Roster 1", coach.id)
            .set_description(Some("This is Roster 1's description.".to_string()))
            .build_and_insert(db)
            .unwrap(),
        RosterBuilder::new("Test Roster 2", coach.id)
            .set_description(Some("This is Roster 2's description.".to_string()))
            .build_and_insert(db)
            .unwrap()
    )
}

pub fn arrange_get_roster_by_id_integration_test(db: &mut database::Connection) -> Roster {
    let coach = UserBuilder::new_default("Coach 1")
        .set_hash_password("password123")
        .build_and_insert(db)
        .unwrap();
    RosterBuilder::new("Test Roster 1", coach.id)
        .set_description(Some("This is Roster 1's description.".to_string()))
        .build_and_insert(db)
        .unwrap();
    RosterBuilder::new("Test Roster 2", coach.id)
        .set_description(Some("This is Roster 2's description.".to_string()))
        .build_and_insert(db)
        .unwrap()
}

pub fn arrange_update_works_integration_test(db: &mut database::Connection) -> Roster {
    let coach = UserBuilder::new_default("Coach 1")
        .set_hash_password("password123")
        .build_and_insert(db)
        .unwrap();
    RosterBuilder::new("Test Roster 1", coach.id)
        .set_description(Some("Roster 1 testing update.".to_string()))
        .build_and_insert(db)
        .unwrap()
}

pub fn arrange_delete_works_integration_test(db: &mut database::Connection) -> Roster {
    let coach = UserBuilder::new_default("Coach 1")
        .set_hash_password("password123")
        .build_and_insert(db)
        .unwrap();
    RosterBuilder::new("Test Roster 1", coach.id)
        .set_description(Some("Roster 1 testing delete.".to_string()))
        .build_and_insert(db)
        .unwrap()
}

pub fn arrange_add_quizzer_to_roster_works_integration_test(db: &mut database::Connection) -> (Roster, User) {
    let coach = UserBuilder::new_default("Coach 1")
        .set_hash_password("password123")
        .build_and_insert(db)
        .unwrap();
    let quizzer = UserBuilder::new_default("Quizzer 1")
        .set_hash_password("password123")
        .build_and_insert(db)
        .unwrap();
    let roster = RosterBuilder::new("Test Roster for adding quizzers", coach.id)
        .set_description(Some("Roster for testing adding quizzers.".to_string()))
        .build_and_insert(db)
        .unwrap();
    (roster, quizzer)
}

pub fn arrange_add_rostercoach_to_roster_works_integration_test(db: &mut database::Connection) -> NewRosterCoach {
    let coach = UserBuilder::new_default("Coach 1")
        .set_hash_password("password123")
        .build_and_insert(db)
        .unwrap();
    let roster = RosterBuilder::new("Roster 1", coach.id)
        .set_description(Some("Roster for testing adding rostercoaches.".to_string()))
        .build_and_insert(db)
        .unwrap();
    RosterCoachBuilder::new_default(coach.id, roster.rosterid)
        .build()
        .unwrap()
}

pub fn arrange_get_all_coaches_of_roster_works_integration_test(db: &mut database::Connection) -> (Roster, User, User) {
    let coach_1 = UserBuilder::new_default("Coach 1")
        .set_hash_password("password123")
        .build_and_insert(db)
        .unwrap();
    let coach_2 = UserBuilder::new_default("Coach 2")
        .set_hash_password("password123")
        .build_and_insert(db)
        .unwrap();
    let roster_1 = RosterBuilder::new_default("Roster 1", coach_1.id)
        .build_and_insert(db)
        .unwrap();
    RosterCoachBuilder::new_default(coach_1.id, roster_1.rosterid)
        .build_and_insert(db)
        .unwrap();
    RosterCoachBuilder::new_default(coach_2.id, roster_1.rosterid)
        .build_and_insert(db)
        .unwrap();

    let coach_3 = UserBuilder::new_default("Coach 3")
        .set_hash_password("password123")
        .build_and_insert(db)
        .unwrap();
    let coach_4 = UserBuilder::new_default("Coach 4")
        .set_hash_password("password123")
        .build_and_insert(db)
        .unwrap();
    let roster_2 = RosterBuilder::new_default("Roster 2", coach_1.id)
        .build_and_insert(db)
        .unwrap();
    RosterCoachBuilder::new_default(coach_3.id, roster_2.rosterid)
        .build_and_insert(db)
        .unwrap();
    RosterCoachBuilder::new_default(coach_4.id, roster_2.rosterid)
        .build_and_insert(db)
        .unwrap();

    (roster_2, coach_3, coach_4)
}

pub fn arrange_remove_coach_from_roster_works_integration_test(db: &mut database::Connection) -> (User, Roster, RosterCoach) {
    let coach = UserBuilder::new_default("Coach 1")
        .set_hash_password("password123")
        .build_and_insert(db)
        .unwrap();
    let roster = RosterBuilder::new_default("Roster 1", coach.id)
        .build_and_insert(db)
        .unwrap();
    let roster_coach = RosterCoachBuilder::new_default(coach.id, roster.rosterid)
            .build_and_insert(db)
            .unwrap();
    (coach, roster, roster_coach)
}

pub fn arrange_get_all_quizzers_of_roster_works_integration_test(db: &mut database::Connection) -> (Roster, User, User) {
    let coach = UserBuilder::new_default("Coach 1")
        .set_hash_password("password123")
        .build_and_insert(db)
        .unwrap();
    let roster = RosterBuilder::new_default("Roster 1", coach.id)
        .build_and_insert(db)
        .unwrap();

    let quizzer_1 = UserBuilder::new_default("Quizzer 1")
        .set_hash_password("password123")
        .build_and_insert(db)
        .unwrap();
    let quizzer_2 = UserBuilder::new_default("Quizzer 2")
        .set_hash_password("password123")
        .build_and_insert(db)
        .unwrap();

    RosterQuizzerBuilder::new_default(quizzer_1.id, roster.rosterid)
        .build_and_insert(db)
        .unwrap();
    RosterQuizzerBuilder::new_default(quizzer_2.id, roster.rosterid)
        .build_and_insert(db)
        .unwrap();

    (roster, quizzer_1, quizzer_2)
}
