use actix_web::{Error, HttpMessage, HttpResponse, HttpRequest, post, Result, web::{Data, Json}};
use serde::{Deserialize, Serialize};
use serde_json::json;
use chrono::{DateTime, Utc, Duration};
use diesel::Connection as _;
use rand::seq::SliceRandom;
use uuid::Uuid;

use crate::database::Database;
use crate::{
    auth::{is_rbac_and_abac_authorized, policies::{division::DivisionPolicyResource, PolicyContext, UserContext}},
    models::{self, common::PaginationParams, permission::{AppAction, AppResource},
        roundgroup::NewRoundGroup, poolbracketgroup::NewPoolBracketGroup, pool_bracket::NewPoolBracket,
        round::NewRound, game::NewGame, team_teamgroup::NewTeamTeamgroup},
};

/// Request to auto-create a round-robin schedule for a division. Mirrors the "Create Schedule"
/// dialog: one session (with its own pool group + pools) is created per entry in `session_names`,
/// and each session is one full single round-robin per pool.
#[derive(Debug, Deserialize)]
struct CreateScheduleRequest {
    did: Uuid,
    /// One created session per name (the "# of times each team plays" count).
    session_names: Vec<String>,
    /// How many pools to split the division's teams across.
    num_pools: i64,
    /// Minutes between consecutive rounds (used to space each round's scheduled start).
    round_duration_minutes: i64,
    /// Start time of the first round; when null, rounds are created unscheduled.
    start_time: Option<DateTime<Utc>>,
    /// Ordered room ids to fill the concurrent-game slots each round.
    room_slots: Vec<Uuid>,
    /// When true (default), teams are re-shuffled into new pools for each session's pool group, so
    /// teams face mostly-different opponents session to session. Only matters with 2+ sessions.
    #[serde(default)]
    shuffle_teams: Option<bool>,
}

/// Split `team_ids` across `num_pools` pools (round-robin distribution balances sizes), keeping only
/// pools with 2+ teams (a lone team can't play). Pool sizes depend only on positions, so shuffling
/// the input first changes membership but not the number or sizes of pools.
fn divide_into_pools(team_ids: &[Uuid], num_pools: usize) -> Vec<Vec<Uuid>> {
    let mut pools: Vec<Vec<Uuid>> = vec![Vec::new(); num_pools];
    for (i, id) in team_ids.iter().enumerate() {
        pools[i % num_pools].push(*id);
    }
    pools.into_iter().filter(|p| p.len() >= 2).collect()
}

/// Counts of what was created — everything or nothing (the work runs in one transaction).
#[derive(Debug, Serialize)]
struct CreateScheduleResponse {
    sessions_created: usize,
    pool_groups_created: usize,
    pools_created: usize,
    team_placements_created: usize,
    rounds_created: usize,
    games_created: usize,
}

/// A single 2-team round-robin (circle method) for `team_ids`: one Vec of (left, right) pairings
/// per round. An odd count adds a bye each round. Round count is len-1 (even) or len (odd).
fn round_robin_pairings(team_ids: &[Uuid]) -> Vec<Vec<(Uuid, Uuid)>> {
    if team_ids.len() < 2 { return vec![]; }
    let mut list: Vec<Option<Uuid>> = team_ids.iter().map(|t| Some(*t)).collect();
    if list.len() % 2 == 1 { list.push(None); }
    let n = list.len();
    let half = n / 2;
    let mut rounds = Vec::with_capacity(n - 1);
    for _ in 0..(n - 1) {
        let mut pairs = Vec::with_capacity(half);
        for i in 0..half {
            if let (Some(a), Some(b)) = (list[i], list[n - 1 - i]) {
                pairs.push((a, b));
            }
        }
        rounds.push(pairs);
        // Rotate all but the first entry (standard round-robin rotation).
        let last = list.remove(n - 1);
        list.insert(1, last);
    }
    rounds
}

/// Resolve the division + tournament and check the caller holds `division:<action>` on it.
fn authorize_via_division(
    conn: &mut crate::database::Connection,
    user_ctx: &UserContext,
    division_id: Uuid,
    action: &str,
) -> Result<(), HttpResponse> {
    let division = match models::division::read(conn, division_id) {
        Ok(d) => d,
        Err(_) => return Err(HttpResponse::UnprocessableEntity().json(json!({
            "error": format!("Division with ID {} does not exist", division_id)
        }))),
    };
    let tournament = match models::tournament::read(conn, division.tid) {
        Ok(t) => t,
        Err(_) => return Err(HttpResponse::InternalServerError().finish()),
    };
    let user_is_admin = models::tournament_admin::is_admin(conn, tournament.tid, user_ctx.user_id);
    let policy_ctx = PolicyContext {
        user_ctx: user_ctx.clone(),
        resource: DivisionPolicyResource { tournament, user_is_tournament_admin: user_is_admin },
    };
    let permission = format!("{}:{}", AppResource::Division.as_str(), action);
    if is_rbac_and_abac_authorized(&policy_ctx, &permission, AppResource::Division.as_str()).is_err() {
        return Err(HttpResponse::Unauthorized().finish());
    }
    Ok(())
}

#[post("")]
async fn create(
    db: Data<Database>,
    Json(req_body): Json<CreateScheduleRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let mut conn = db.get_connection().expect("Failed to get connection");
    models::apicalllog::create(&mut conn, &req);

    let user_id = {
        let extensions = req.extensions();
        match extensions.get::<UserContext>() {
            Some(u_ctx) => u_ctx.user_id,
            None => return Ok(HttpResponse::Unauthorized().finish()),
        }
    };

    // Authorize as the parent division (tournament owner or admin), same as pools/rounds/sessions.
    {
        let extensions = req.extensions();
        let user_ctx = extensions.get::<UserContext>().unwrap().clone();
        drop(extensions);
        if let Err(resp) = authorize_via_division(&mut conn, &user_ctx, req_body.did, AppAction::Create.as_str()) {
            return Ok(resp);
        }
    }

    // ── Validate inputs before touching the database (so nothing is created on a bad request) ──
    if req_body.session_names.is_empty() {
        return Ok(HttpResponse::UnprocessableEntity().json(json!({ "error": "At least one session is required." })));
    }
    if req_body.num_pools < 1 {
        return Ok(HttpResponse::UnprocessableEntity().json(json!({ "error": "At least one pool is required." })));
    }

    let num_pools = req_body.num_pools as usize;
    let teams = match models::team::read_all_teams_of_division(&mut conn, req_body.did, &PaginationParams { page: 0, page_size: 500 }) {
        Ok(t) => t,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };
    if teams.len() < 2 {
        return Ok(HttpResponse::UnprocessableEntity().json(json!({ "error": "The division needs at least 2 teams to schedule." })));
    }

    // Base division for validation. Shuffling only permutes which team lands in which pool — pool
    // count and sizes (and therefore round count and per-round concurrency) are identical — so the
    // base division validates every session, shuffled or not.
    let team_ids: Vec<Uuid> = teams.iter().map(|t| t.teamid).collect();
    let base_pools = divide_into_pools(&team_ids, num_pools);
    if base_pools.is_empty() {
        return Ok(HttpResponse::UnprocessableEntity().json(json!({ "error": "Each pool needs at least 2 teams; reduce the number of pools." })));
    }

    let base_pairings: Vec<Vec<Vec<(Uuid, Uuid)>>> = base_pools.iter().map(|p| round_robin_pairings(p)).collect();
    let session_rounds = base_pairings.iter().map(|pp| pp.len()).max().unwrap_or(0);
    if session_rounds == 0 {
        return Ok(HttpResponse::UnprocessableEntity().json(json!({ "error": "No rounds could be generated for the given teams and pools." })));
    }

    // The busiest round's concurrent-game count must fit in the assigned room slots.
    let max_concurrency = (0..session_rounds)
        .map(|r| base_pairings.iter().map(|pp| pp.get(r).map_or(0, |v| v.len())).sum::<usize>())
        .max()
        .unwrap_or(0);
    if req_body.room_slots.len() < max_concurrency {
        return Ok(HttpResponse::UnprocessableEntity().json(json!({
            "error": format!("Not enough rooms: {} concurrent games need at least {} rooms.", max_concurrency, max_concurrency)
        })));
    }

    // Every room used must exist and have a quizmaster (games require one).
    for room_id in &req_body.room_slots {
        match models::room::read(&mut conn, *room_id) {
            Ok(room) if room.quizmaster_id.is_none() => return Ok(HttpResponse::UnprocessableEntity().json(json!({
                "error": format!("Room \"{}\" has no quizmaster; assign one before scheduling.", room.name)
            }))),
            Ok(_) => {}
            Err(_) => return Ok(HttpResponse::UnprocessableEntity().json(json!({
                "error": format!("Room with ID {} does not exist", room_id)
            }))),
        }
    }

    // ── Create everything in one transaction: on any error nothing is persisted ──
    let did = req_body.did;
    let session_names = req_body.session_names.clone();
    let room_slots = req_body.room_slots.clone();
    let round_duration = req_body.round_duration_minutes;
    let start_time = req_body.start_time;
    // Default on: only shuffles when there's more than one session (a single session has nothing to
    // vary against). When off, every session reuses the same pool division.
    let shuffle_teams = req_body.shuffle_teams.unwrap_or(true) && session_names.len() > 1;

    let tx_result = conn.transaction::<CreateScheduleResponse, diesel::result::Error, _>(|conn| {
        let mut counts = CreateScheduleResponse {
            sessions_created: 0, pool_groups_created: 0, pools_created: 0,
            team_placements_created: 0, rounds_created: 0, games_created: 0,
        };
        let mut global_round_index: i64 = 0;
        let mut rng = rand::rng();

        for session_name in &session_names {
            let name = if session_name.trim().is_empty() { "Session".to_string() } else { session_name.trim().to_string() };

            // Each session gets its own pool composition when shuffling; otherwise the base division.
            // Sizes match the base either way, so validation above still holds.
            let (session_pools, session_pairings) = if shuffle_teams {
                let mut order = team_ids.clone();
                order.shuffle(&mut rng);
                let pools = divide_into_pools(&order, num_pools);
                let pairings: Vec<Vec<Vec<(Uuid, Uuid)>>> = pools.iter().map(|p| round_robin_pairings(p)).collect();
                (pools, pairings)
            } else {
                (base_pools.clone(), base_pairings.clone())
            };

            let session = models::roundgroup::create(conn, &NewRoundGroup {
                did, name: name.clone(), creator_userid: user_id, last_modified_userid: user_id,
            })?;
            counts.sessions_created += 1;

            let group = models::poolbracketgroup::create(conn, &NewPoolBracketGroup {
                divisionid: did, name: name.clone(), creator_userid: user_id, last_modified_userid: user_id,
            })?;
            counts.pool_groups_created += 1;

            // This session's pools + team placements.
            let mut pool_ids: Vec<Uuid> = Vec::with_capacity(session_pools.len());
            for (p, teams_in_pool) in session_pools.iter().enumerate() {
                // Pool names must be unique within the division, and every session gets its own
                // "Pool A/B/…"; qualify with the (division-unique) session name so they don't collide.
                let pool = models::pool_bracket::create(conn, &NewPoolBracket {
                    divisionid: did,
                    name: format!("{} - Pool {}", name, (b'A' + p as u8) as char),
                    type_: "pool".to_string(),
                    poolbracketgroupid: Some(group.poolbracketgroupid),
                    creator_userid: user_id,
                    last_modified_userid: user_id,
                })?;
                counts.pools_created += 1;

                let team_group = models::teamgroup::resolve_or_create_for_pool_bracket(conn, pool.pool_bracket_id, user_id)?;
                for team_id in teams_in_pool {
                    models::team_teamgroup::create(conn, &NewTeamTeamgroup {
                        teamid: *team_id,
                        team_group_id: team_group.team_group_id,
                        creator_userid: user_id,
                        last_modified_userid: user_id,
                    })?;
                    counts.team_placements_created += 1;
                }
                pool_ids.push(pool.pool_bracket_id);
            }

            // Rounds run in parallel across pools; games draw rooms from the slots in order.
            for r in 0..session_rounds {
                let start = start_time.map(|s| s + Duration::minutes(global_round_index * round_duration));
                // Round names run sequentially across all sessions (session 1 rounds 1..n, session 2
                // continues at n+1, …), so `global_round_index` — not the per-session `r` — names them.
                let round = models::round::create(conn, &NewRound {
                    roundgroup_id: session.roundgroup_id,
                    name: format!("{}", global_round_index + 1),
                    scheduled_start_time: start,
                    last_modified_user: user_id,
                })?;
                counts.rounds_created += 1;

                let mut slot = 0usize;
                for (p, pool_id) in pool_ids.iter().enumerate() {
                    let pairs = match session_pairings[p].get(r) { Some(v) => v, None => continue };
                    for (leftteamid, rightteamid) in pairs {
                        // Bounds already validated, but guard so we never index past the slots.
                        let room_id = *room_slots.get(slot).ok_or_else(||
                            diesel::result::Error::RollbackTransaction)?;
                        slot += 1;
                        let room = models::room::read(conn, room_id)?;
                        let quizmasterid = room.quizmaster_id.ok_or_else(||
                            diesel::result::Error::RollbackTransaction)?;
                        models::game::create(conn, &NewGame {
                            org: String::new(),
                            roomid: room_id,
                            roundid: round.roundid,
                            ignore: false,
                            ruleset: String::new(),
                            leftteamid: *leftteamid,
                            centerteamid: None,
                            rightteamid: *rightteamid,
                            quizmasterid,
                            contentjudgeid: room.contentjudge_id,
                            clientkey: "api-default-create-sched".to_string(),
                            poolbracket_id: *pool_id,
                            last_modified_user: user_id,
                            creator_id: user_id,
                        })?;
                        counts.games_created += 1;
                    }
                }
                global_round_index += 1;
            }
        }

        Ok(counts)
    });

    match tx_result {
        Ok(counts) => Ok(HttpResponse::Created().json(counts)),
        Err(e) => {
            tracing::error!("Create-schedule transaction failed and was rolled back: {:?}", e);
            Ok(HttpResponse::UnprocessableEntity().json(json!({
                "error": "Failed to create the schedule; no data was created."
            })))
        }
    }
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    scope.service(create)
}
