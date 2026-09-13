// Import of GameEvents from a QuizMachine-style CSV export (see ExportQuizzes4.csv).
//
// The CSV columns (in order) are:
//   GUID, Tournament Name, Division, Room Name, Round Number, Question Number,
//   Event Number, Team Name, Team Number, Seat Number, Event Code, parm0, parm1, DateTime
//
// Rows are grouped into "games" by the compound key (Tournament, Division, Round, Room)
// and matched against existing Games in the given tournament. The preview is a dry run
// (no writes); commit inserts the events for importable games in a single transaction.

use crate::database;
use crate::models::common::PaginationParams;
use crate::models::{division::Division, game::Game, gameevent::{self, NewGameEvent}, room::{self, Room}, round::Round};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;
use std::collections::{BTreeMap, HashSet};
use uuid::Uuid;

// Event codes that make up the round "initialization" (roster/config). A game that has
// only these — or none at all — is still importable; any in-game (scoring) code means the
// game already has records beyond initialization.
const INIT_CODES: [&str; 8] = ["RM", "QT", "IP", "OP", "TN", "QN", "SC", "SS"];
const VALID_CODES: [&str; 23] = [
    "RM", "QT", "IP", "OP", "TN", "QN", "SC", "SS", "TC", "TE", "NJ", "BC", "BE", "QO",
    "EO", "C-", "A+", "A-", "FC", "F-", "SB", "TO", "DE",
];

// ─── Preview / result shapes (serialized to the client) ───────────────────────

#[derive(Serialize)]
pub struct ImportableGame {
    pub gid: Uuid,
    pub division: String,
    pub room: String,
    pub round: String,
    pub event_count: usize,
}

#[derive(Serialize)]
pub struct GameImportError {
    pub division: String,
    pub room: String,
    pub round: String,
    pub message: String,
}

#[derive(Serialize, Clone)]
pub struct GameRef {
    pub division: String,
    pub room: String,
    pub round: String,
}

#[derive(Serialize)]
pub struct ImportPreview {
    pub importable: Vec<ImportableGame>,   // games that will get events imported
    pub errors: Vec<GameImportError>,      // matched games that can't be imported (with reason)
    pub games_not_found: Vec<GameRef>,     // file games that match no existing game
    pub missing_games: Vec<GameRef>,       // existing games in a referenced room absent from the file
}

// ─── Internal parsing structures ──────────────────────────────────────────────

struct Row {
    question: i32,
    eventnum: i32,
    team_name: String,
    team: i32,
    seat: i32,
    event_code: String,
    parm1: String,
    parm2: String,
    client_ts: String,
}

#[derive(Default)]
struct Group {
    tournament: String,
    division: String,
    room: String,
    round: String,
    rows: Vec<Row>,
    parse_errors: Vec<String>,
}

type Key = (String, String, String, String); // (tournament, division, room, round)

/// Splits a CSV line into fields, treating single-quoted segments as literals so that
/// commas inside quotes are preserved. Surrounding single quotes are stripped.
fn split_fields(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut cur = String::new();
    let mut in_quote = false;
    for ch in line.chars() {
        match ch {
            '\'' => in_quote = !in_quote,
            ',' if !in_quote => {
                fields.push(std::mem::take(&mut cur));
            }
            _ => cur.push(ch),
        }
    }
    fields.push(cur);
    fields
}

fn parse_ts(raw: &str) -> chrono::DateTime<Utc> {
    NaiveDateTime::parse_from_str(raw.trim(), "%Y-%m-%d-%H.%M.%S%.f")
        .map(|ndt| ndt.and_utc())
        .unwrap_or_else(|_| Utc::now())
}

/// Parses the CSV text into groups keyed by (tournament, division, room, round).
fn parse_groups(csv: &str) -> BTreeMap<Key, Group> {
    let mut groups: BTreeMap<Key, Group> = BTreeMap::new();

    for raw_line in csv.lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        let fields = split_fields(line);

        // We need at least the compound-key fields (indices 1..=4) to attribute the row.
        if fields.len() < 5 {
            // Cannot attribute this row to a game; record under an "unknown" group.
            let key = ("".to_string(), "".to_string(), "".to_string(), "".to_string());
            let group = groups.entry(key).or_default();
            group.parse_errors.push(format!("expected 14 columns but found {}", fields.len()));
            continue;
        }

        let tournament = fields[1].trim().to_string();
        let division = fields[2].trim().to_string();
        let room = fields[3].trim().to_string();
        let round = fields[4].trim().to_string();
        let key = (tournament.clone(), division.clone(), room.clone(), round.clone());
        let group = groups.entry(key).or_default();
        group.tournament = tournament;
        group.division = division;
        group.room = room;
        group.round = round;

        if fields.len() < 14 {
            group.parse_errors.push(format!("expected 14 columns but found {}", fields.len()));
            continue;
        }

        let question = match fields[5].trim().parse::<i32>() {
            Ok(v) => v,
            Err(_) => { group.parse_errors.push(format!("question number '{}' is not an integer", fields[5].trim())); continue; }
        };
        let eventnum = match fields[6].trim().parse::<i32>() {
            Ok(v) => v,
            Err(_) => { group.parse_errors.push(format!("event number '{}' is not an integer", fields[6].trim())); continue; }
        };
        let team = match fields[8].trim().parse::<i32>() {
            Ok(v) => v,
            Err(_) => { group.parse_errors.push(format!("team number '{}' is not an integer", fields[8].trim())); continue; }
        };
        let seat = match fields[9].trim().parse::<i32>() {
            Ok(v) => v,
            Err(_) => { group.parse_errors.push(format!("seat number '{}' is not an integer", fields[9].trim())); continue; }
        };
        let event_code = fields[10].trim().to_string();
        if !VALID_CODES.contains(&event_code.as_str()) {
            group.parse_errors.push(format!("unknown event code '{}'", event_code));
            continue;
        }

        group.rows.push(Row {
            question,
            eventnum,
            team_name: fields[7].trim().to_string(),
            team,
            seat,
            event_code,
            parm1: fields[11].trim().to_string(),
            parm2: fields[12].trim().to_string(),
            client_ts: fields[13].trim().to_string(),
        });
    }

    groups
}

// ─── DB lookups ───────────────────────────────────────────────────────────────

fn find_division(db: &mut database::Connection, tournament_id: Uuid, name_str: &str) -> Option<Division> {
    use crate::schema::divisions::dsl as d;
    d::divisions.filter(d::tid.eq(tournament_id)).filter(d::dname.eq(name_str)).first::<Division>(db).ok()
}

fn find_round(db: &mut database::Connection, division_id: Uuid, number_str: &str) -> Option<Round> {
    use crate::schema::rounds::dsl as r;
    r::rounds.filter(r::did.eq(division_id)).filter(r::name.eq(number_str)).first::<Round>(db).ok()
}

fn find_game(db: &mut database::Connection, room_id: Uuid, round_id: Uuid) -> Option<Game> {
    use crate::schema::games::dsl as g;
    g::games.filter(g::roomid.eq(room_id)).filter(g::roundid.eq(round_id)).first::<Game>(db).ok()
}

fn games_in_room(db: &mut database::Connection, room_id: Uuid) -> Vec<Game> {
    use crate::schema::games::dsl as g;
    g::games.filter(g::roomid.eq(room_id)).load::<Game>(db).unwrap_or_default()
}

/// (division name, room name, round name) for an existing game, for display in errors.
fn game_ref(db: &mut database::Connection, game: &Game) -> GameRef {
    use crate::schema::rooms::dsl as rm;
    use crate::schema::rounds::dsl as rd;
    // Division is derived via game -> pool_bracket -> division_session -> division.
    let division = crate::models::game::read_division_of_game(db, game).map(|d| d.dname).unwrap_or_default();
    let room = rm::rooms.filter(rm::roomid.eq(game.roomid)).select(rm::name).first::<String>(db).unwrap_or_default();
    let round = rd::rounds.filter(rd::roundid.eq(game.roundid)).select(rd::name).first::<String>(db).unwrap_or_default();
    GameRef { division, room, round }
}

fn game_has_records_beyond_initialization(db: &mut database::Connection, game_id: Uuid) -> bool {
    let pagination = PaginationParams { page: 0, page_size: PaginationParams::MAX_PAGE_SIZE as i64 };
    let events = gameevent::read_all_gameevents_of_game(db, game_id, &pagination).unwrap_or_default();
    events.iter().any(|e| !INIT_CODES.contains(&e.event.as_str()))
}

// ─── Validation ───────────────────────────────────────────────────────────────

/// Validates a group's rows for sequentiality. Returns an error message if the question
/// numbers are not contiguous from 1, or if any question is missing an event number in
/// the 0..=max sequence (or has duplicates).
fn validate_sequence(group: &Group) -> Option<String> {
    // Question numbers must be contiguous starting at 1.
    let mut questions: Vec<i32> = group.rows.iter().map(|r| r.question).collect();
    questions.sort_unstable();
    questions.dedup();
    for (idx, q) in questions.iter().enumerate() {
        if *q != (idx as i32 + 1) {
            return Some(format!(
                "question numbers are not sequential (a gap was found near question {})",
                if idx == 0 { *q } else { questions[idx - 1] + 1 }
            ));
        }
    }

    // Within each question, event numbers must run 0..=max with no gaps or duplicates.
    let mut by_question: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    for row in &group.rows {
        by_question.entry(row.question).or_default().push(row.eventnum);
    }
    for (question, events) in &by_question {
        let set: HashSet<i32> = events.iter().copied().collect();
        if set.len() != events.len() {
            return Some(format!("question {} has duplicate event numbers", question));
        }
        let max = events.iter().copied().max().unwrap_or(0);
        for e in 0..=max {
            if !set.contains(&e) {
                return Some(format!("question {} is missing event number {}", question, e));
            }
        }
    }

    None
}

// ─── Preview / commit ─────────────────────────────────────────────────────────

/// Builds the dry-run preview plus the concrete insert plan for importable games.
fn build(db: &mut database::Connection, tournament_id: Uuid, csv: &str) -> (ImportPreview, Vec<(Uuid, Vec<NewGameEvent>)>) {
    let tournament_name = {
        use crate::schema::tournaments::dsl as t;
        t::tournaments.filter(t::tid.eq(tournament_id)).select(t::tname).first::<String>(db).unwrap_or_default()
    };

    let groups = parse_groups(csv);

    let mut importable: Vec<ImportableGame> = Vec::new();
    let mut errors: Vec<GameImportError> = Vec::new();
    let mut games_not_found: Vec<GameRef> = Vec::new();
    let mut insert_plan: Vec<(Uuid, Vec<NewGameEvent>)> = Vec::new();

    let mut found_gids: HashSet<Uuid> = HashSet::new();
    let mut referenced_room_ids: HashSet<Uuid> = HashSet::new();

    for (_key, group) in &groups {
        let gref = GameRef { division: group.division.clone(), room: group.room.clone(), round: group.round.clone() };

        // Parse errors take precedence and are reported first.
        if !group.parse_errors.is_empty() {
            errors.push(GameImportError {
                division: gref.division, room: gref.room, round: gref.round,
                message: format!("Parsing error: {}", group.parse_errors.join("; ")),
            });
            continue;
        }

        // Resolve the compound key to an existing game within this tournament.
        if !group.tournament.is_empty() && group.tournament != tournament_name {
            games_not_found.push(gref);
            continue;
        }
        let division = match find_division(db, tournament_id, &group.division) {
            Some(d) => d,
            None => { games_not_found.push(gref); continue; }
        };
        let room = match room::find_by_name_in_tournament(db, &group.room, tournament_id) {
            Ok(r) => r,
            Err(_) => { games_not_found.push(gref); continue; }
        };
        let round = match find_round(db, division.did, &group.round) {
            Some(r) => r,
            None => { games_not_found.push(gref); continue; }
        };
        let game = match find_game(db, room.roomid, round.roundid) {
            Some(g) => g,
            None => { games_not_found.push(gref); continue; }
        };

        found_gids.insert(game.gid);
        referenced_room_ids.insert(room.roomid);

        // The game must not already have records beyond initialization.
        if game_has_records_beyond_initialization(db, game.gid) {
            errors.push(GameImportError {
                division: gref.division, room: gref.room, round: gref.round,
                message: "This game already has game events beyond initialization; import skipped.".to_string(),
            });
            continue;
        }

        // Sequentiality checks.
        if let Some(msg) = validate_sequence(group) {
            errors.push(GameImportError {
                division: gref.division, room: gref.room, round: gref.round, message: msg,
            });
            continue;
        }

        // Importable — build the events to insert.
        let events: Vec<NewGameEvent> = group.rows.iter().map(|row| NewGameEvent {
            gid: game.gid,
            question: row.question,
            eventnum: row.eventnum,
            name: row.team_name.clone(),
            team: row.team,
            quizzer: row.seat,
            event: row.event_code.clone(),
            parm1: row.parm1.clone(),
            parm2: row.parm2.clone(),
            clientts: parse_ts(&row.client_ts),
            serverts: Utc::now(),
            md5digest: String::new(),
            qm_registration_key: None,
            source: Some("sneakernet import".to_string()),
        }).collect();

        importable.push(ImportableGame {
            gid: game.gid,
            division: gref.division,
            room: gref.room,
            round: gref.round,
            event_count: events.len(),
        });
        insert_plan.push((game.gid, events));
    }

    // Games that exist in a referenced room but are absent from the file entirely.
    let mut missing_games: Vec<GameRef> = Vec::new();
    for room_id in &referenced_room_ids {
        for game in games_in_room(db, *room_id) {
            if !found_gids.contains(&game.gid) {
                missing_games.push(game_ref(db, &game));
            }
        }
    }

    (
        ImportPreview { importable, errors, games_not_found, missing_games },
        insert_plan,
    )
}

/// Dry-run: returns the preview without writing anything.
pub fn preview(db: &mut database::Connection, tournament_id: Uuid, csv: &str) -> ImportPreview {
    build(db, tournament_id, csv).0
}

/// Commit: re-validates and inserts events for all importable games in one transaction.
/// Returns the (post-import) preview describing what was imported and what was skipped.
pub fn commit(db: &mut database::Connection, tournament_id: Uuid, csv: &str) -> QueryResult<ImportPreview> {
    let (result, insert_plan) = build(db, tournament_id, csv);

    db.transaction::<_, diesel::result::Error, _>(|conn| {
        use crate::schema::gameevents::dsl::{gid, question, eventnum};
        for (_gid, events) in &insert_plan {
            // Skip any row that already exists (e.g. pre-existing round-initialization
            // events) so the composite primary key never collides.
            diesel::insert_into(crate::schema::gameevents::table)
                .values(events)
                .on_conflict((gid, question, eventnum))
                .do_nothing()
                .execute(conn)?;
        }
        Ok(())
    })?;

    Ok(result)
}
