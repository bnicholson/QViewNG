mod common;

use backend::database::Database;
use backend::database::seed_data::seed_one::insert_seed_data_one;
use backend::database::seed_data::system_default_data::insert_system_default_data;
use backend::models::{self, common::PaginationParams};
use diesel::prelude::*;
use uuid::Uuid;
use crate::common::{TEST_DB_URL, clean_database};

/// Seeds a whole tournament with game events and checks that the generator broke every tie.
///
/// Ignored by default because a full seed takes several minutes. Run it after changing the
/// generator's scoring or overtime logic:
///
/// ```sh
/// ./run-tests.sh  # or:
/// cargo test --test seed_data_test -- --ignored --nocapture --test-threads=1
/// ```
#[test]
#[ignore = "seeds a full tournament; takes several minutes"]
fn seeded_games_never_end_tied() {
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("connect");

    insert_system_default_data(&mut conn);
    insert_seed_data_one(&mut conn, true, true);

    let pg = PaginationParams { page: 0, page_size: PaginationParams::MAX_PAGE_SIZE as i64 };
    let game_ids: Vec<Uuid> = {
        use backend::schema::games::dsl::*;
        games.filter(del_fl.eq(false)).select(gid).load(&mut conn).unwrap()
    };

    let mut scored = 0;
    let mut tied: Vec<String> = Vec::new();
    let mut overtime = 0;
    let mut mismatched: Vec<String> = Vec::new();

    for id in &game_ids {
        let events = models::gameevent::read_all_gameevents_of_game(&mut conn, *id, &pg).unwrap();
        let max_q = events.iter().map(|e| e.question).max().unwrap_or(0);
        let results = match models::gameevent::calculate_team_results_for_game(*id, events) {
            Ok(r) => r,
            Err(_) => continue,
        };
        if results.len() < 2 { continue; }
        scored += 1;
        if max_q > 20 { overtime += 1; }

        let best = results.iter().map(|r| r.rank).min().unwrap();
        let leaders = results.iter().filter(|r| r.rank == best).count();
        if leaders > 1 {
            tied.push(format!("{} ({})", id, results.iter()
                .map(|r| format!("{} {} rank {}", r.name, r.score, r.rank))
                .collect::<Vec<_>>().join(" / ")));
        }

        // W+L must account for every game a team played.
        for a in &results {
            let w = results.iter().filter(|b| b.team != a.team && b.rank > a.rank).count();
            let l = results.iter().filter(|b| b.team != a.team && b.rank < a.rank).count();
            if w + l != results.len() - 1 {
                mismatched.push(format!("{} in {}", a.name, id));
            }
        }
    }

    println!("\nscored games: {}, of which went to overtime: {}", scored, overtime);
    assert!(tied.is_empty(), "games still ending tied:\n  {}", tied.join("\n  "));
    assert!(mismatched.is_empty(), "W+L does not cover every game for:\n  {}", mismatched.join("\n  "));
    assert!(overtime > 0, "no game went to overtime, so the tie-break path was never exercised");
}
