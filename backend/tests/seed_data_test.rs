mod common;

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
/// cargo test --test seed_data_test -- --ignored --nocapture --test-threads=1
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

/// The Experienced division splits its double round-robin across two sessions: Session 1 holds
/// every pair's first meeting and Session 2 the rematch, each session owning its own pool group of
/// three four-team pools. Every seeded game is a complete round with all 20 questions answered.
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

    // Two sessions (roundgroups), each with its own pool group of three pools.
    let sessions: Vec<(Uuid, String)> = {
        use backend::schema::roundgroups::dsl as rg;
        rg::roundgroups.filter(rg::did.eq(division_id)).filter(rg::del_fl.eq(false))
            .select((rg::roundgroup_id, rg::name)).order(rg::name.asc()).load(&mut conn).unwrap()
    };
    assert_eq!(
        sessions.iter().map(|(_, n)| n.as_str()).collect::<Vec<_>>(),
        vec!["Session 1", "Session 2"],
        "Experienced should run two sessions",
    );

    let groups: Vec<(Uuid, String)> = {
        use backend::schema::poolbracketgroups::dsl as pbg;
        pbg::poolbracketgroups.filter(pbg::divisionid.eq(division_id)).filter(pbg::del_fl.eq(false))
            .select((pbg::poolbracketgroupid, pbg::name)).order(pbg::name.asc()).load(&mut conn).unwrap()
    };
    assert_eq!(groups.len(), 2, "Experienced should have two pool groups, got {:?}", groups);

    // A pool references its team group directly, so carry it along for the team count.
    let pools: Vec<(Uuid, String, Option<Uuid>)> = {
        use backend::schema::pool_brackets::dsl as pb;
        pb::pool_brackets.filter(pb::divisionid.eq(division_id)).filter(pb::del_fl.eq(false))
            .select((pb::pool_bracket_id, pb::name, pb::team_group_id))
            .order(pb::name.asc()).load(&mut conn).unwrap()
    };
    assert_eq!(pools.len(), 6, "three pools per session group, got {:?}", pools);

    // Each group owns exactly three of them.
    for (group_id, group_name) in &groups {
        let in_group: i64 = {
            use backend::schema::pool_brackets::dsl as pb;
            pb::pool_brackets.filter(pb::poolbracketgroupid.eq(*group_id)).filter(pb::del_fl.eq(false))
                .count().get_result(&mut conn).unwrap()
        };
        assert_eq!(in_group, 3, "{} should own three pools", group_name);
    }

    let pg = PaginationParams { page: 0, page_size: PaginationParams::MAX_PAGE_SIZE as i64 };
    let mut total_games = 0;
    // Across both sessions, every pair of pool-mates must meet exactly twice.
    let mut division_meetings: std::collections::BTreeMap<(Uuid, Uuid), i32> = Default::default();

    for (pool_id, pool_name, team_group) in &pools {
        let group = team_group.expect("a seeded pool owns a team group");
        let team_count: i64 = {
            use backend::schema::team_teamgroups::dsl as ttg;
            ttg::team_teamgroups.filter(ttg::team_group_id.eq(group))
                .count().get_result(&mut conn).unwrap()
        };
        assert_eq!(team_count, 4, "{} should hold four teams", pool_name);

        let games: Vec<(Uuid, Uuid, Uuid)> = {
            use backend::schema::games::dsl as g;
            g::games.filter(g::poolbracket_id.eq(*pool_id)).filter(g::del_fl.eq(false))
                .select((g::gid, g::leftteamid, g::rightteamid)).load(&mut conn).unwrap()
        };
        // Four teams: each session's pool holds one meeting of each of the 6 pairings.
        assert_eq!(games.len(), 6, "{} should schedule 6 games", pool_name);
        total_games += games.len();

        // Every unordered pair of pool teams meets exactly once within a session's pool.
        let mut meetings: std::collections::BTreeMap<(Uuid, Uuid), i32> = Default::default();
        for (_, l, r) in &games {
            let (a, b) = (*l, *r);
            let key = if a < b { (a, b) } else { (b, a) };
            *meetings.entry(key).or_insert(0) += 1;
        }
        assert_eq!(meetings.len(), 6, "{}: expected 6 distinct pairings", pool_name);
        for (pair, count) in &meetings {
            assert_eq!(*count, 1, "{}: pair {:?} met {} times in one session, expected 1", pool_name, pair, count);
        }
        for pair in meetings.keys() {
            *division_meetings.entry(*pair).or_insert(0) += 1;
        }

        // No team plays twice in the same round, and no room hosts two games in a round.
        let per_round: Vec<(Uuid, Uuid, Uuid, Uuid)> = {
            use backend::schema::games::dsl as g;
            g::games.filter(g::poolbracket_id.eq(*pool_id)).filter(g::del_fl.eq(false))
                .select((g::roundid, g::roomid, g::leftteamid, g::rightteamid)).load(&mut conn).unwrap()
        };
        let mut seen: std::collections::BTreeSet<(Uuid, Uuid)> = Default::default();
        for (round, room, l, r) in &per_round {
            assert!(seen.insert((*round, *room)), "{}: room double-booked in a round", pool_name);
            assert!(seen.insert((*round, *l)), "{}: team plays twice in a round", pool_name);
            assert!(seen.insert((*round, *r)), "{}: team plays twice in a round", pool_name);
        }
    }
    assert_eq!(total_games, 36, "Experienced should schedule 36 games in all");
    assert_eq!(division_meetings.len(), 18, "6 pairings per pool x 3 pools");
    for (pair, count) in &division_meetings {
        assert_eq!(*count, 2, "pair {:?} met {} times across the two sessions, expected 2", pair, count);
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
