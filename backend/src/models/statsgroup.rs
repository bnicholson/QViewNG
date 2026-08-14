
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
    pub olympic_points: i32,        // per-game placement points: 2/1/0 for 1st/2nd/3rd
    pub mod_olympic_points: i32,    // modified olympic points (same as olympic for now)
    pub total_points: i32,          // sum of final scores across games
    pub tie_breaker: String,        // manual tie-break rule (blank until set)
}

struct TeamStatAgg {
    games: i32,
    wins: i32,
    losses: i32,
    olympic_points: i32,
    total_points: i32,
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
        for result in results {
            if result.name.trim().is_empty() {
                continue;
            }
            let entry = agg.entry(result.name.clone()).or_insert(TeamStatAgg {
                games: 0, wins: 0, losses: 0, olympic_points: 0, total_points: 0,
            });
            entry.games += 1;
            entry.total_points += result.score;
            if result.rank == 1 {
                entry.wins += 1;
            } else {
                entry.losses += 1;
            }
            // Olympic points: 1st -> 2, 2nd -> 1, 3rd (or worse) -> 0.
            entry.olympic_points += (3 - result.rank).max(0);
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
            mod_olympic_points: a.olympic_points, // same as olympic for now
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
