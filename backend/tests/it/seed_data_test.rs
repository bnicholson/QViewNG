use backend::database::seed_data::seed_one::insert_seed_data_one;
use backend::database::seed_data::system_default_data::insert_system_default_data;
use backend::models::{self, common::PaginationParams};
use diesel::prelude::*;
use uuid::Uuid;
use crate::common::{test_db};

/// Seeds a whole tournament with game events and checks that the generator broke every tie.
///
/// Ignored by default because a full seed takes several minutes. Run it after changing the
/// generator's scoring or overtime logic:
///
/// ```sh
/// ./run-tests.sh  # or:
/// cargo test --test it seed_data_test:: -- --ignored --nocapture --test-threads=1
/// ```
#[test]
#[ignore = "seeds a full tournament; takes several minutes"]
fn seeded_games_never_end_tied() {
    let db = test_db();
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

/// The Experienced division splits its double round-robin across two pool groups: Group 1's three
/// four-team pools (A1/B1/C1, rounds 1-3) hold every pair's first meeting and Group 2's (A2/B2/C2,
/// rounds 4-6) the rematch. Every seeded game is a complete round with all 20 questions answered.
///
/// Ignored by default alongside the tie-break check: a full seed takes several minutes.
#[test]
#[ignore = "seeds a full tournament; takes several minutes"]
fn experienced_division_runs_three_pools_of_double_round_robins() {
    let db = test_db();
    let mut conn = db.get_connection().expect("connect");

    insert_system_default_data(&mut conn);
    insert_seed_data_one(&mut conn, true, true);

    let division_id: Uuid = {
        use backend::schema::divisions::dsl as d;
        d::divisions.filter(d::dname.eq("Experienced")).filter(d::del_fl.eq(false))
            .select(d::did).first(&mut conn).unwrap()
    };

    // Two pool groups, each owning three pools.
    let mut groups = models::poolbracketgroup::read_all_of_division(&mut conn, division_id).unwrap();
    groups.sort_by(|a, b| a.name.cmp(&b.name));
    assert_eq!(
        groups.iter().map(|g| g.name.as_str()).collect::<Vec<_>>(),
        vec!["Group 1", "Group 2"],
        "Experienced should have two pool groups",
    );
    assert_eq!(
        models::pool_bracket::read_all_of_division(&mut conn, division_id).unwrap().len(), 6,
        "three pools per pool group",
    );

    // Round names of the division, by id, to check which rounds each group's games are played in.
    let pg = PaginationParams { page: 0, page_size: PaginationParams::MAX_PAGE_SIZE as i64 };
    let round_names: std::collections::BTreeMap<Uuid, String> =
        models::round::read_all_rounds_of_division(&mut conn, division_id, &pg).unwrap()
            .into_iter().map(|r| (r.roundid, r.name)).collect();
    assert_eq!(round_names.len(), 6, "Experienced should have six rounds");

    let mut total_games = 0;
    // Across both pool groups, every pair of pool-mates must meet exactly twice.
    let mut division_meetings: std::collections::BTreeMap<(Uuid, Uuid), i32> = Default::default();

    for (group, (expected_pools, expected_rounds)) in groups.iter().zip([
        (["Pool A1", "Pool B1", "Pool C1"], ["1", "2", "3"]),
        (["Pool A2", "Pool B2", "Pool C2"], ["4", "5", "6"]),
    ]) {
        let mut pools = models::pool_bracket::read_all_of_poolbracketgroup(&mut conn, group.poolbracketgroupid).unwrap();
        pools.sort_by(|a, b| a.name.cmp(&b.name));
        assert_eq!(
            pools.iter().map(|p| p.name.as_str()).collect::<Vec<_>>(), expected_pools,
            "{} should own three pools", group.name,
        );

        // Every unordered pair of pool-mates meets exactly once within this group's pools.
        let mut group_meetings: std::collections::BTreeMap<(Uuid, Uuid), i32> = Default::default();
        let mut group_rounds: std::collections::BTreeSet<String> = Default::default();

        for pool in &pools {
            let pool_name = &pool.name;
            let team_group = pool.team_group_id.expect("a seeded pool owns a team group");
            let team_count = models::team_teamgroup::read_all_of_team_group(&mut conn, team_group).unwrap().len();
            assert_eq!(team_count, 4, "{} should hold four teams", pool_name);

            let games: Vec<(Uuid, Uuid, Uuid, Uuid)> = {
                use backend::schema::games::dsl as g;
                g::games.filter(g::poolbracket_id.eq(pool.pool_bracket_id)).filter(g::del_fl.eq(false))
                    .select((g::roundid, g::roomid, g::leftteamid, g::rightteamid)).load(&mut conn).unwrap()
            };
            // Four teams: each pool holds one meeting of each of the 6 pairings.
            assert_eq!(games.len(), 6, "{} should schedule 6 games", pool_name);
            total_games += games.len();

            let mut pool_meetings: std::collections::BTreeMap<(Uuid, Uuid), i32> = Default::default();
            // No team plays twice in the same round, and no room hosts two games in a round.
            let mut seen: std::collections::BTreeSet<(Uuid, Uuid)> = Default::default();
            for (round, room, l, r) in &games {
                let key = if l < r { (*l, *r) } else { (*r, *l) };
                *pool_meetings.entry(key).or_insert(0) += 1;
                assert!(seen.insert((*round, *room)), "{}: room double-booked in a round", pool_name);
                assert!(seen.insert((*round, *l)), "{}: team plays twice in a round", pool_name);
                assert!(seen.insert((*round, *r)), "{}: team plays twice in a round", pool_name);
                group_rounds.insert(round_names.get(round).expect("game's round is in the division").clone());
            }
            assert_eq!(pool_meetings.len(), 6, "{}: expected 6 distinct pairings", pool_name);
            for (pair, count) in pool_meetings {
                *group_meetings.entry(pair).or_insert(0) += count;
            }
        }

        assert_eq!(group_meetings.len(), 18, "{}: 6 pairings per pool x 3 pools", group.name);
        for (pair, count) in &group_meetings {
            assert_eq!(*count, 1, "{}: pair {:?} met {} times in one pool group, expected 1", group.name, pair, count);
            *division_meetings.entry(*pair).or_insert(0) += 1;
        }
        assert_eq!(
            group_rounds.iter().map(|s| s.as_str()).collect::<Vec<_>>(), expected_rounds,
            "{} should be played in rounds {:?}", group.name, expected_rounds,
        );
    }
    assert_eq!(total_games, 36, "Experienced should schedule 36 games in all");
    assert_eq!(division_meetings.len(), 18, "the same 18 pairings in both pool groups");
    for (pair, count) in &division_meetings {
        assert_eq!(*count, 2, "pair {:?} met {} times across the two pool groups, expected 2", pair, count);
    }

    // Every game in the tournament is a complete round: questions 1..=20 all present and answered.
    let all_games: Vec<Uuid> = {
        use backend::schema::games::dsl as g;
        g::games.filter(g::del_fl.eq(false)).select(g::gid).load(&mut conn).unwrap()
    };
    let mut checked = 0;
    for id in &all_games {
        let events = models::gameevent::read_all_gameevents_of_game(&mut conn, *id, &pg).unwrap();
        if events.is_empty() { panic!("game {} has no events", id); }
        for q in 1..=20 {
            let for_q: Vec<&str> = events.iter()
                .filter(|e| e.question == q)
                .map(|e| e.event.as_str())
                .collect();
            assert!(!for_q.is_empty(), "game {} is missing question {}", id, q);
            // A question is answered when its toss-up was taken (TC) or missed and sent to a bonus.
            let answered = for_q.iter().any(|e| *e == "TC")
                || (for_q.iter().any(|e| *e == "TE") && for_q.iter().any(|e| *e == "BC" || *e == "BE"));
            assert!(answered, "game {} question {} was not answered: {:?}", id, q, for_q);
        }
        checked += 1;
    }
    println!("\nverified {} games, all 20 questions answered in each", checked);
}

/// Seeded games record quiz outs, error outs and fouls the way QuizMachine does: a QO right after
/// a quizzer's 4th TC and an EO right after their 3rd TE, nobody acts once they are out, and some
/// games build up to a foul out (3 F-s). Every game still scores.
///
/// Ignored by default alongside the other full-seed checks: a full seed takes several minutes.
#[test]
#[ignore = "seeds a full tournament; takes several minutes"]
fn seeded_games_record_outs_and_fouls() {
    let db = test_db();
    let mut conn = db.get_connection().expect("connect");

    insert_system_default_data(&mut conn);
    insert_seed_data_one(&mut conn, true, true);

    let pg = PaginationParams { page: 0, page_size: PaginationParams::MAX_PAGE_SIZE as i64 };
    let game_ids: Vec<Uuid> = {
        use backend::schema::games::dsl::*;
        games.filter(del_fl.eq(false)).select(gid).load(&mut conn).unwrap()
    };

    let (mut quiz_outs, mut error_outs, mut foul_outs, mut quizzer_fouls, mut team_fouls) = (0, 0, 0, 0, 0);
    for id in &game_ids {
        let mut events = models::gameevent::read_all_gameevents_of_game(&mut conn, *id, &pg).unwrap();
        events.sort_by_key(|e| (e.question, e.eventnum));
        assert!(
            models::gameevent::calculate_team_results_for_game(*id, events.clone()).is_ok(),
            "game {} can't be scored", id
        );

        // (team, seat) -> (correct, errors, fouls)
        let mut counts: std::collections::HashMap<(i32, i32), (u32, u32, u32)> = Default::default();
        for (i, e) in events.iter().enumerate() {
            let key = (e.team, e.quizzer);
            let c = counts.entry(key).or_default();
            let out = c.0 >= 4 || c.1 >= 3 || c.2 >= 3;
            match e.event.as_str() {
                "TC" | "TE" | "BC" | "BE" | "F-" => {
                    assert!(!out, "game {}: quizzer {:?} has a {} on question {} after being out", id, key, e.event, e.question)
                }
                _ => {}
            }
            let next_is = |code: &str| events.get(i + 1).is_some_and(|n| n.event == code && (n.team, n.quizzer) == key);
            match e.event.as_str() {
                "TC" => {
                    c.0 += 1;
                    assert_eq!(c.0 == 4, next_is("QO"), "game {}: QO must follow exactly the 4th TC of {:?}", id, key);
                }
                "TE" => {
                    c.1 += 1;
                    assert_eq!(c.1 == 3, next_is("EO"), "game {}: EO must follow exactly the 3rd TE of {:?}", id, key);
                }
                "F-" => {
                    c.2 += 1;
                    quizzer_fouls += 1;
                    if c.2 == 3 { foul_outs += 1; }
                }
                "QO" => quiz_outs += 1,
                "EO" => error_outs += 1,
                "FC" => team_fouls += 1,
                _ => {}
            }
        }
    }

    println!(
        "\nquiz outs: {}, error outs: {}, foul outs: {}, quizzer fouls: {}, team/coach fouls: {}",
        quiz_outs, error_outs, foul_outs, quizzer_fouls, team_fouls
    );
    assert!(quiz_outs > 0 && error_outs > 0, "no quiz outs or no error outs were seeded");
    assert!(foul_outs > 0, "no quizzer fouled out");
    assert!(team_fouls > 0, "no team or coach fouls were seeded");
}

/// Every user the seed creates holds the `member` role, whatever other roles they get.
#[test]
fn every_seeded_user_is_a_member() {
    let db = test_db();
    let mut conn = db.get_connection().expect("connect");

    insert_system_default_data(&mut conn);
    insert_seed_data_one(&mut conn, false, false);

    let member_role = models::role::read_by_name(&mut conn, models::role::AppRole::Member.as_str()).unwrap();
    let without_member: Vec<String> = {
        use backend::schema::{users, users_roles};
        let members = users_roles::table
            .filter(users_roles::role_id.eq(member_role.id))
            .select(users_roles::user_id);
        users::table
            .filter(users::id.ne_all(members))
            .select(users::email)
            .load(&mut conn)
            .unwrap()
    };
    assert!(without_member.is_empty(), "seeded users without the member role: {:?}", without_member);
}
