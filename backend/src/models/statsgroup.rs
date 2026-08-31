
use crate::database;
use crate::models::common::PaginationParams;
use crate::models::game_statsgroup::GameStatsGroup;
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult,AsChangeset,Insertable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use chrono::{Utc,DateTime};

pub struct StatsGroupBuilder {
    name: String,                              // Name of the statsgroup (human readable)
    description: Option<String>,               // Description of the statsgroup
    tournament_id: Uuid,                       // Tournament the statsgroup belongs to (required)
    division_id: Option<Uuid>,                 // Division the statsgroup is scoped to (optional)
}

impl StatsGroupBuilder {
    pub fn new(statsgroup_name: &str, tournament_id: Uuid) -> Self {
        Self {
            name: statsgroup_name.to_string(),
            description: None,
            tournament_id,
            division_id: None,
        }
    }
    pub fn new_default(statsgroup_name: &str, tournament_id: Uuid) -> Self {
        Self {
            name: statsgroup_name.to_string(),
            description: None,
            tournament_id,
            division_id: None,
        }
    }
    pub fn set_name(mut self, statsgroup_name: String) -> Self {
        self.name = statsgroup_name;
        self
    }
    pub fn set_description(mut self, description: Option<String>) -> Self {
        self.description = description;
        self
    }
    pub fn set_tournament_id(mut self, tournament_id: Uuid) -> Self {
        self.tournament_id = tournament_id;
        self
    }
    pub fn set_division_id(mut self, division_id: Option<Uuid>) -> Self {
        self.division_id = division_id;
        self
    }
    pub fn build(self) -> Result<NewStatsGroup, Vec<String>> {
        Ok(
            NewStatsGroup {
                name: self.name,
                description: self.description,
                tournament_id: self.tournament_id,
                division_id: self.division_id,
            }
        )
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<StatsGroup> {
        let new_statsgroup = self.build();
        create(db, &new_statsgroup.unwrap())
    }
}

// #[tsync::tsync]
#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Identifiable,
    ToSchema
)]
#[diesel(table_name = crate::schema::statsgroups)]
#[diesel(primary_key(sgid))]
pub struct StatsGroup {
    pub sgid: Uuid,                                // identifies the statsgroup uniquely
    pub name: String,                              // Name of the statsgroup (human readable)
    pub description: Option<String>,               // Description of the statsgroup
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tournament_id: Uuid,                       // Tournament the statsgroup belongs to (required)
    pub division_id: Option<Uuid>,                 // Division the statsgroup is scoped to (optional)
}

#[derive(
    Insertable,
    Serialize,
    Deserialize,
    Debug
)]
#[diesel(table_name = crate::schema::statsgroups)]
pub struct NewStatsGroup {
    pub name: String,                              // Name of the statsgroup (human readable)
    pub description: Option<String>,               // Description of the statsgroup
    pub tournament_id: Uuid,                       // Tournament the statsgroup belongs to (required)
    #[serde(default)]
    pub division_id: Option<Uuid>,                 // Division the statsgroup is scoped to (optional)
}

// #[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::statsgroups)]
#[diesel(primary_key(sgid))]
pub struct StatsGroupChangeset {
    pub name: String,                              // Name of the statsgroup (human readable)
    pub description: Option<String>,               // Description of the statsgroup
}

pub fn create(db: &mut database::Connection, item: &NewStatsGroup) -> QueryResult<StatsGroup> {
    use crate::schema::statsgroups::dsl::*;
    insert_into(statsgroups).values(item).get_result::<StatsGroup>(db)
}

pub fn exists(db: &mut database::Connection, statsgroupid: Uuid) -> bool {
    use crate::schema::statsgroups::dsl::statsgroups;
    statsgroups
        .find(statsgroupid)
        .get_result::<StatsGroup>(db)
        .is_ok()
}

pub fn read(db: &mut database::Connection, item_id: Uuid) -> QueryResult<StatsGroup> {
    use crate::schema::statsgroups::dsl::*;
    statsgroups.filter(sgid.eq(item_id)).first::<StatsGroup>(db)
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<StatsGroup>> {
    use crate::schema::statsgroups::dsl::*;
    statsgroups
        .order(created_at)
        .limit(pagination.page_size)
        .offset(
            pagination.page
                * std::cmp::max(pagination.page_size, PaginationParams::MAX_PAGE_SIZE as i64),
        )
        .load::<StatsGroup>(db)
}

pub fn count(db: &mut database::Connection) -> QueryResult<i64> {
    use crate::schema::statsgroups::dsl::*;
    statsgroups.count().get_result(db)
}

pub fn read_all_statsgroups_of_tournament(db: &mut database::Connection, tid: Uuid, pagination: &PaginationParams) -> QueryResult<Vec<StatsGroup>> {
    use crate::schema::statsgroups::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    statsgroups
        .filter(tournament_id.eq(tid))
        .order(created_at.asc())
        .limit(page_size)
        .offset(offset_val)
        .load::<StatsGroup>(db)
}

pub fn read_all_statsgroups_of_game(db: &mut database::Connection, game_id: Uuid, pagination: &PaginationParams) -> QueryResult<Vec<StatsGroup>> {
    use crate::schema::games_statsgroups::dsl::*;
    use crate::schema::statsgroups::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    let sg_ids: Vec<Uuid> = 
        games_statsgroups
            .filter(gameid.eq(game_id))
            .load::<GameStatsGroup>(db)
            .unwrap()
            .iter()
            .map(|gsg| gsg.statsgroupid)
            .collect();

    statsgroups
        .filter(sgid.eq_any(sg_ids))
        .order(sgid.asc())
        .limit(page_size)
        .offset(offset_val)
        .load::<StatsGroup>(db)
}

pub fn update(db: &mut database::Connection, sg_id: Uuid, item: &StatsGroupChangeset) -> QueryResult<StatsGroup> {
    use crate::schema::statsgroups::dsl::*;
    diesel::update(statsgroups.filter(sgid.eq(sg_id)))
        .set((
            item,
            updated_at.eq(diesel::dsl::now),
        ))
        .get_result(db)
}

pub fn delete(db: &mut database::Connection, sg_id: Uuid) -> QueryResult<usize> {
    use crate::schema::statsgroups::dsl::*;
    diesel::delete(statsgroups.filter(sgid.eq(sg_id))).execute(db)
}

// ─── Team standings for a statsgroup ──────────────────────────────────────────

/// Aggregated standings row for one team within a statsgroup.
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct TeamStat {
    pub place: i32,                 // 1-based standings rank within the statsgroup
    pub name: String,               // team name
    pub games: i32,                 // number of (scored) games the team played
    pub wins: i32,                  // games placed 1st
    pub losses: i32,                // games not placed 1st
    pub olympic_points: i32,        // Olympic points per round (§12.4): 2-team 8/3, 3-team 10/5/1
    pub mod_olympic_points: i32,    // Modified Olympic points (§12.5): Olympic base adjusted by team score
    pub total_points: i32,          // sum of final scores across games
    pub tie_breaker: String,        // manual tie-break rule (blank until set)
}

struct TeamStatAgg {
    games: i32,
    wins: i32,
    losses: i32,
    olympic_points: i32,
    mod_olympic_points: i32,
    total_points: i32,
}

/// Olympic (§12.4) and Modified Olympic (§12.5) points awarded to one team for one round
/// (game). `num_teams` is the round size (2 or 3), `rank` the team's placement (1 = 1st),
/// `score` its regular team score for the round (overtime questions don't count), and
/// `game_total` the sum of all teams' scores in the round (for the modified "10% of the total
/// points" clause). Modified values are rounded to the nearest whole number.
/// Returns (olympic, modified_olympic).
fn olympic_awards(num_teams: usize, rank: i32, score: i32, game_total: i32) -> (i32, i32) {
    let s = score as f64;
    let total = game_total as f64;
    let round = |x: f64| x.round() as i32;

    match (num_teams, rank) {
        // ── 2-team rounds: base + 0.75 per 10 pts over a threshold ──
        (2, 1) => (8, round(8.0 + 0.75 * (s - 100.0).max(0.0) / 10.0)),
        (2, 2) => (3, round(3.0 + 0.75 * (s - 60.0).max(0.0) / 10.0)),
        // ── 3-team rounds: max( base + 1 per 10 over threshold, 10% of round total − offset ),
        //    floored at the base. ──
        (3, 1) => {
            let by_score = 10.0 + (s - 100.0).max(0.0) / 10.0;
            let by_total = 0.10 * total;
            (10, round(by_score.max(by_total).max(10.0)))
        }
        (3, 2) => {
            let by_score = 5.0 + (s - 60.0).max(0.0) / 10.0;
            let by_total = 0.10 * total - 1.0;
            (5, round(by_score.max(by_total).max(5.0)))
        }
        (3, 3) => {
            let by_score = 1.0 + (s - 30.0).max(0.0) / 10.0;
            let by_total = 0.10 * total - 2.0;
            (1, round(by_score.max(by_total).max(1.0)))
        }
        // Placements beyond the podium / unusual round sizes earn nothing.
        _ => (0, 0),
    }
}

/// Computes team standings for a statsgroup by running the score calculator over
/// each of the statsgroup's games and aggregating per team (keyed by team name).
/// Games whose event stream can't be scored are skipped.
pub fn read_team_stats_of_statsgroup(db: &mut database::Connection, sg_id: Uuid) -> QueryResult<Vec<TeamStat>> {
    use std::collections::HashMap;

    let pagination = PaginationParams { page: 0, page_size: PaginationParams::MAX_PAGE_SIZE as i64 };
    let games = crate::models::game::read_all_games_of_statsgroup(db, sg_id, &pagination)?;

    let mut agg: HashMap<String, TeamStatAgg> = HashMap::new();

    for game in games {
        let events = crate::models::gameevent::read_all_gameevents_of_game(db, game.gid, &pagination)?;
        // Skip games whose event stream is invalid / cannot be scored.
        let results = match crate::models::gameevent::calculate_team_results_for_game(game.gid, events) {
            Ok(r) => r,
            Err(_) => continue,
        };
        // Round size and the round's total team score (for the modified "10% of total" clause).
        let num_teams = results.len();
        let game_total: i32 = results.iter().map(|r| r.score).sum();
        for result in &results {
            if result.name.trim().is_empty() {
                continue;
            }
            let (olympic, mod_olympic) = olympic_awards(num_teams, result.rank, result.score, game_total);
            let entry = agg.entry(result.name.clone()).or_insert(TeamStatAgg {
                games: 0, wins: 0, losses: 0, olympic_points: 0, mod_olympic_points: 0, total_points: 0,
            });
            entry.games += 1;
            entry.total_points += result.score;
            if result.rank == 1 {
                entry.wins += 1;
            } else {
                entry.losses += 1;
            }
            entry.olympic_points += olympic;
            entry.mod_olympic_points += mod_olympic;
        }
    }

    let mut stats: Vec<TeamStat> = agg
        .into_iter()
        .map(|(name, a)| TeamStat {
            place: 0,
            name,
            games: a.games,
            wins: a.wins,
            losses: a.losses,
            olympic_points: a.olympic_points,
            mod_olympic_points: a.mod_olympic_points,
            total_points: a.total_points,
            tie_breaker: String::new(),
        })
        .collect();

    // Standings order: most wins, then most total points (name as a stable final tiebreak).
    stats.sort_by(|a, b| {
        b.wins.cmp(&a.wins)
            .then(b.total_points.cmp(&a.total_points))
            .then(a.name.cmp(&b.name))
    });
    for (idx, stat) in stats.iter_mut().enumerate() {
        stat.place = (idx + 1) as i32;
    }

    Ok(stats)
}

// ─── Individual (quizzer) standings for a statsgroup ──────────────────────────

/// Aggregated individual-stats row for one quizzer within a statsgroup.
/// Only the first ten fields are populated; the remaining detail columns shown
/// in the UI (Errs 16+/5+, Generals, Memory, According, Context, Special) are
/// intentionally left out of this computation for now.
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct IndividualStat {
    pub place: i32,             // 1-based rank within the statsgroup (by score)
    pub individual: String,     // quizzer name
    pub team_name: String,      // the quizzer's team name
    pub games: i32,             // number of (scored) games the quizzer played
    pub score: i32,             // total individual points across games
    pub avg: f64,               // score / games
    pub correct: i32,           // correct tossups
    pub errors: i32,            // erroneous tossups
    pub bonus_pts: i32,         // points from correct bonuses
    pub bonus_attempts: i32,    // correct + erroneous bonus attempts
}

struct IndividualStatAgg {
    games: i32,
    score: i32,
    correct: i32,
    errors: i32,
    bonus_pts: i32,
    bonus_attempts: i32,
}

/// Computes individual (per-quizzer) stats for a statsgroup by running the score
/// calculator over each of the statsgroup's games and aggregating per quizzer
/// (keyed by team name + quizzer name). Games that can't be scored are skipped.
pub fn read_individual_stats_of_statsgroup(db: &mut database::Connection, sg_id: Uuid) -> QueryResult<Vec<IndividualStat>> {
    use std::collections::HashMap;

    let pagination = PaginationParams { page: 0, page_size: PaginationParams::MAX_PAGE_SIZE as i64 };
    let games = crate::models::game::read_all_games_of_statsgroup(db, sg_id, &pagination)?;

    // Keyed by (team name, quizzer name) so quizzers with the same name on
    // different teams are not merged.
    let mut agg: HashMap<(String, String), IndividualStatAgg> = HashMap::new();

    for game in games {
        let events = crate::models::gameevent::read_all_gameevents_of_game(db, game.gid, &pagination)?;
        let results = match crate::models::gameevent::calculate_quizzer_results_for_game(game.gid, events) {
            Ok(r) => r,
            Err(_) => continue,
        };
        for result in results {
            if result.name.trim().is_empty() {
                continue;
            }
            let entry = agg.entry((result.team_name.clone(), result.name.clone())).or_insert(IndividualStatAgg {
                games: 0, score: 0, correct: 0, errors: 0, bonus_pts: 0, bonus_attempts: 0,
            });
            entry.games += 1;
            entry.score += result.score;
            entry.correct += result.correct;
            entry.errors += result.errors;
            entry.bonus_pts += result.bonus_pts;
            entry.bonus_attempts += result.bonus_attempts;
        }
    }

    let mut stats: Vec<IndividualStat> = agg
        .into_iter()
        .map(|((team_name, individual), a)| IndividualStat {
            place: 0,
            individual,
            team_name,
            games: a.games,
            score: a.score,
            avg: if a.games > 0 { a.score as f64 / a.games as f64 } else { 0.0 },
            correct: a.correct,
            errors: a.errors,
            bonus_pts: a.bonus_pts,
            bonus_attempts: a.bonus_attempts,
        })
        .collect();

    // Rank individuals by overall individual points (desc), then fewest errors (asc). Name is
    // only a stable final tiebreak for deterministic display order.
    stats.sort_by(|a, b| {
        b.score.cmp(&a.score)
            .then(a.errors.cmp(&b.errors))
            .then(a.individual.cmp(&b.individual))
    });
    // Competitive ranking: quizzers with identical (score, errors) share a place, and the
    // next distinct quizzer's place is its position in the list.
    let mut current_place = 0i32;
    let mut prev_key: Option<(i32, i32)> = None;
    for (idx, stat) in stats.iter_mut().enumerate() {
        let key = (stat.score, stat.errors);
        if prev_key != Some(key) {
            current_place = (idx + 1) as i32;
            prev_key = Some(key);
        }
        stat.place = current_place;
    }

    Ok(stats)
}

#[cfg(test)]
mod tests {
    use super::olympic_awards;

    // §12.4 — flat Olympic points by placement and round size; the modified value is
    // returned alongside but only the first element is asserted here.
    #[test]
    fn olympic_points_by_placement_and_round_size() {
        // 2-team round: 1st -> 8, 2nd -> 3. (game_total is irrelevant to Olympic points.)
        assert_eq!(olympic_awards(2, 1, 100, 160).0, 8);
        assert_eq!(olympic_awards(2, 2, 60, 160).0, 3);

        // 3-team round: 1st -> 10, 2nd -> 5, 3rd -> 1.
        assert_eq!(olympic_awards(3, 1, 100, 210).0, 10);
        assert_eq!(olympic_awards(3, 2, 60, 210).0, 5);
        assert_eq!(olympic_awards(3, 3, 30, 210).0, 1);

        // Off-podium placements and unusual round sizes earn nothing.
        assert_eq!(olympic_awards(2, 3, 100, 160), (0, 0));
        assert_eq!(olympic_awards(3, 4, 100, 210), (0, 0));
        assert_eq!(olympic_awards(1, 1, 100, 100), (0, 0));
    }

    // §12.5 — Modified Olympic points adjust the base by team score (2-team) or by the
    // greater of a score bonus and 10% of the round total, floored at the base (3-team).
    #[test]
    fn modified_olympic_adjusts_by_score() {
        let modv = |n, r, s, t| olympic_awards(n, r, s, t).1;

        // ── 2-team ──
        // At/under the threshold, no bonus: base only.
        assert_eq!(modv(2, 1, 100, 160), 8);
        assert_eq!(modv(2, 2, 60, 160), 3);
        assert_eq!(modv(2, 1, 80, 140), 8);   // below 100 -> still 8
        // 1st: 8 + 0.75 per 10 over 100. 120 -> 8 + 1.5 = 9.5 -> 10 (rounds up).
        assert_eq!(modv(2, 1, 120, 200), 10);
        // 140 -> 8 + 3.0 = 11 exactly.
        assert_eq!(modv(2, 1, 140, 220), 11);
        // 130 -> 8 + 2.25 = 10.25 -> 10 (rounds down).
        assert_eq!(modv(2, 1, 130, 210), 10);
        // 2nd: 3 + 0.75 per 10 over 60. 100 -> 3 + 3 = 6.
        assert_eq!(modv(2, 2, 100, 160), 6);

        // ── 3-team ──
        // 1st: max(10 + (s-100)/10, 10% of total), floor 10.
        //   s=120,total=200 -> max(12, 20) = 20.
        assert_eq!(modv(3, 1, 120, 200), 20);
        //   s=90,total=80  -> max(10, 8) then floor 10 -> 10.
        assert_eq!(modv(3, 1, 90, 80), 10);
        // 2nd: max(5 + (s-60)/10, 10% of total - 1), floor 5.
        //   s=80,total=200 -> max(7, 19) = 19.
        assert_eq!(modv(3, 2, 80, 200), 19);
        //   s=60,total=50  -> max(5, 4) floor 5 -> 5.
        assert_eq!(modv(3, 2, 60, 50), 5);
        // 3rd: max(1 + (s-30)/10, 10% of total - 2), floor 1.
        //   s=50,total=200 -> max(3, 18) = 18.
        assert_eq!(modv(3, 3, 50, 200), 18);
        //   s=30,total=30  -> max(1, 1) floor 1 -> 1.
        assert_eq!(modv(3, 3, 30, 30), 1);
    }
}
