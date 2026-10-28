
use crate::cache::Cache;
use crate::database;
use crate::models::gameevent::{CalculatedGame, CalculatedQuizzer, QuizzerGameResult, TeamGameResult};
use std::collections::HashMap;
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
    pub del_fl: bool,
    pub tiebreaker_rules: String,                  // see `TieBreakerRule::parse` ("" = defaults)
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
    use crate::schema::statsgroups::dsl::*;
    statsgroups
        .find(statsgroupid)
        .filter(del_fl.eq(false))
        .get_result::<StatsGroup>(db)
        .is_ok()
}

pub fn read(db: &mut database::Connection, item_id: Uuid) -> QueryResult<StatsGroup> {
    use crate::schema::statsgroups::dsl::*;
    statsgroups.filter(sgid.eq(item_id)).filter(del_fl.eq(false)).first::<StatsGroup>(db)
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<StatsGroup>> {
    use crate::schema::statsgroups::dsl::*;
    statsgroups
        .filter(del_fl.eq(false))
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
    statsgroups.filter(del_fl.eq(false)).count().get_result(db)
}

pub fn read_all_statsgroups_of_tournament(db: &mut database::Connection, tid: Uuid, pagination: &PaginationParams) -> QueryResult<Vec<StatsGroup>> {
    use crate::schema::statsgroups::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    statsgroups
        .filter(tournament_id.eq(tid))
        .filter(del_fl.eq(false))
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
        .filter(del_fl.eq(false))
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

/// Soft delete: mark the statsgroup deleted (excluded from reads) without removing the row.
pub fn delete(db: &mut database::Connection, sg_id: Uuid) -> QueryResult<usize> {
    use crate::schema::statsgroups::dsl::*;
    diesel::update(statsgroups.filter(sgid.eq(sg_id))).set(del_fl.eq(true)).execute(db)
}

/// Purge: permanently remove the statsgroup row from the database.
pub fn purge(db: &mut database::Connection, sg_id: Uuid) -> QueryResult<usize> {
    use crate::schema::statsgroups::dsl::*;
    diesel::delete(statsgroups.filter(sgid.eq(sg_id))).execute(db)
}

// ─── Tie-breaking rules ───────────────────────────────────────────────────────

/// A criterion for ordering two teams that are otherwise level in the standings.
///
/// The stats group stores its criteria as an ordered, comma-separated list of the keys below,
/// each optionally prefixed with `-` when the user has unchecked it. A disabled criterion keeps
/// its slot in the list so that re-enabling it restores its original position.
/// The JSON names are spelled out so that the wire format, the stored form and `key()` are all the
/// same string — a derived `rename_all` would give `team_win_loss` where the key is `team_wl`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum TieBreakerRule {
    #[serde(rename = "team_wl")]
    TeamWinLoss,
    /// Correct tossups on M (Memory Verse) questions: those of type R, Q or V. Bonuses don't count.
    /// Usable only when every round involved has question types (see
    /// [`memory_verse_rule_available`]).
    #[serde(rename = "memory_verse")]
    MemoryVerseCorrect,
    #[serde(rename = "head_to_head")]
    HeadToHead,
    #[serde(rename = "total_points")]
    TotalPoints,
    #[serde(rename = "avg_points")]
    AvgPoints,
    #[serde(rename = "olympic")]
    OlympicPoints,
    #[serde(rename = "mod_olympic")]
    ModOlympicPoints,
}

impl TieBreakerRule {
    pub fn key(&self) -> &'static str {
        match self {
            TieBreakerRule::TeamWinLoss      => "team_wl",
            TieBreakerRule::MemoryVerseCorrect => "memory_verse",
            TieBreakerRule::HeadToHead       => "head_to_head",
            TieBreakerRule::TotalPoints      => "total_points",
            TieBreakerRule::AvgPoints        => "avg_points",
            TieBreakerRule::OlympicPoints    => "olympic",
            TieBreakerRule::ModOlympicPoints => "mod_olympic",
        }
    }

    /// How the criterion reads in the UI and in the "Rank Rules" explanation.
    pub fn label(&self) -> &'static str {
        match self {
            TieBreakerRule::TeamWinLoss      => "Team W-L Record",
            TieBreakerRule::MemoryVerseCorrect => "Memory Verse (M) Correct",
            TieBreakerRule::HeadToHead       => "Head-to-Head W-L",
            TieBreakerRule::TotalPoints      => "Total Points",
            TieBreakerRule::AvgPoints        => "Total Avg Pts",
            TieBreakerRule::OlympicPoints    => "Olympic Pts",
            TieBreakerRule::ModOlympicPoints => "Mod Olympic Pts",
        }
    }

    /// Whether the criterion is checked until a stats group says otherwise. Memory Verse is off by
    /// default: it changes standings only for groups that opt in.
    pub fn enabled_by_default(&self) -> bool {
        !matches!(self, TieBreakerRule::MemoryVerseCorrect)
    }

    /// Where the criterion goes when it is missing from a stored value: right after this one, if
    /// present. Others missing go at the end.
    fn placed_after(&self) -> Option<TieBreakerRule> {
        match self {
            TieBreakerRule::MemoryVerseCorrect => Some(TieBreakerRule::TeamWinLoss),
            _ => None,
        }
    }

    fn from_key(key: &str) -> Option<Self> {
        DEFAULT_TIEBREAKER_ORDER.iter().copied().find(|r| r.key() == key)
    }
}

/// The order criteria are applied in until a stats group saves its own.
pub const DEFAULT_TIEBREAKER_ORDER: [TieBreakerRule; 7] = [
    TieBreakerRule::TeamWinLoss,
    TieBreakerRule::MemoryVerseCorrect,
    TieBreakerRule::HeadToHead,
    TieBreakerRule::TotalPoints,
    TieBreakerRule::AvgPoints,
    TieBreakerRule::OlympicPoints,
    TieBreakerRule::ModOlympicPoints,
];

/// One row of a stats group's tie-breaker configuration: the criterion and whether it is used.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
pub struct TieBreakerSetting {
    pub rule: TieBreakerRule,
    pub enabled: bool,
    /// False when the stats group lacks the data the criterion needs; it is then shown unchecked,
    /// can't be checked, and is skipped when ranking. Filled in on read and ignored on write.
    #[serde(default = "default_true")]
    pub available: bool,
}

fn default_true() -> bool {
    true
}

/// Parses the stored form. An empty or unparseable value yields the default order, each criterion
/// enabled or not per [`TieBreakerRule::enabled_by_default`]. A criterion missing from a stored
/// value (one added after the group was configured) is added with its default: appended, or for
/// Memory Verse, placed right below Team W-L Record.
pub fn parse_tiebreaker_rules(stored: &str) -> Vec<TieBreakerSetting> {
    let mut settings: Vec<TieBreakerSetting> = Vec::new();
    for token in stored.split(',') {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }
        let (enabled, key) = match token.strip_prefix('-') {
            Some(rest) => (false, rest),
            None => (true, token),
        };
        if let Some(rule) = TieBreakerRule::from_key(key) {
            if !settings.iter().any(|s| s.rule == rule) {
                settings.push(TieBreakerSetting { rule, enabled, available: true });
            }
        }
    }
    for rule in DEFAULT_TIEBREAKER_ORDER {
        if settings.iter().any(|s| s.rule == rule) {
            continue;
        }
        let setting = TieBreakerSetting { rule, enabled: rule.enabled_by_default(), available: true };
        let anchor = rule.placed_after().and_then(|a| settings.iter().position(|s| s.rule == a));
        match anchor {
            Some(pos) => settings.insert(pos + 1, setting),
            None => settings.push(setting),
        }
    }
    settings
}

/// Renders settings back into the stored form.
pub fn serialize_tiebreaker_rules(settings: &[TieBreakerSetting]) -> String {
    settings
        .iter()
        .map(|s| if s.enabled { s.rule.key().to_string() } else { format!("-{}", s.rule.key()) })
        .collect::<Vec<_>>()
        .join(",")
}

/// The stats group's criteria, with any whose data is missing marked unavailable and unchecked.
pub fn read_tiebreaker_rules(db: &mut database::Connection, sg_id: Uuid) -> QueryResult<Vec<TieBreakerSetting>> {
    let settings = parse_tiebreaker_rules(&read(db, sg_id)?.tiebreaker_rules);
    apply_availability(db, sg_id, settings)
}

/// Saves the criteria. One the group lacks the data for is saved unchecked whatever was sent.
pub fn update_tiebreaker_rules(
    db: &mut database::Connection,
    sg_id: Uuid,
    settings: &[TieBreakerSetting],
) -> QueryResult<Vec<TieBreakerSetting>> {
    use crate::schema::statsgroups::dsl::*;
    let settings = apply_availability(db, sg_id, settings.to_vec())?;
    let stored = serialize_tiebreaker_rules(&settings);
    let saved = diesel::update(statsgroups.filter(sgid.eq(sg_id)))
        .set((tiebreaker_rules.eq(stored), updated_at.eq(diesel::dsl::now)))
        .get_result::<StatsGroup>(db)?;
    apply_availability(db, sg_id, parse_tiebreaker_rules(&saved.tiebreaker_rules))
}

/// Marks each criterion available or not for the stats group, unchecking the unavailable ones.
fn apply_availability(
    db: &mut database::Connection,
    sg_id: Uuid,
    mut settings: Vec<TieBreakerSetting>,
) -> QueryResult<Vec<TieBreakerSetting>> {
    let memory_verse_available = memory_verse_rule_available(db, sg_id)?;
    for setting in settings.iter_mut() {
        setting.available = match setting.rule {
            TieBreakerRule::MemoryVerseCorrect => memory_verse_available,
            _ => true,
        };
        if !setting.available {
            setting.enabled = false;
        }
    }
    Ok(settings)
}

/// Whether the Memory Verse criterion can be used: every round of every division the stats group
/// covers has question types for all of its questions. A division's own group covers its division;
/// any other group covers the divisions its games are in. A group covering nothing can't use it.
pub fn memory_verse_rule_available(db: &mut database::Connection, sg_id: Uuid) -> QueryResult<bool> {
    let division_ids: Vec<Uuid> = match read(db, sg_id)?.division_id {
        Some(did) => vec![did],
        None => {
            let pagination = PaginationParams { page: 0, page_size: PaginationParams::MAX_PAGE_SIZE as i64 };
            let round_ids: Vec<Uuid> = crate::models::game::read_all_games_of_statsgroup(db, sg_id, &pagination)?
                .into_iter()
                .map(|g| g.roundid)
                .collect();
            use crate::schema::rounds::dsl as r;
            r::rounds
                .filter(r::roundid.eq_any(&round_ids))
                .select(r::did)
                .distinct()
                .load::<Uuid>(db)?
        }
    };
    if division_ids.is_empty() {
        return Ok(false);
    }
    for did in division_ids {
        if !crate::models::round::division_has_complete_question_types(db, did)? {
            return Ok(false);
        }
    }
    Ok(true)
}

/// How many M (Memory Verse) questions -- regulation questions of type R, Q or V -- each team
/// answered correctly on a tossup in one game, keyed by team index. Bonuses don't count. `question_types` is the game's round's
/// stored types; overtime questions have no type and never count.
fn memory_verse_correct_by_team(calculated: &CalculatedGame, question_types: &str) -> HashMap<i32, i32> {
    let types: Vec<char> = question_types.chars().collect();
    crate::models::gameevent::correct_tossups_by_team(calculated)
        .into_iter()
        .map(|(team_idx, questions)| {
            let count = questions
                .iter()
                .filter(|q| **q >= 1 && matches!(types.get((**q - 1) as usize), Some('R' | 'Q' | 'V')))
                .count() as i32;
            (team_idx, count)
        })
        .collect()
}

// ─── Per-question-type breakdown ──────────────────────────────────────────────

/// The question types a stats row breaks its counts down by, in display order. M (Memory Verse)
/// isn't stored on rounds; it is R, Q and V together, so those questions count toward it as well.
pub const STATS_QUESTION_TYPES: [char; 9] = ['G', 'A', 'M', 'R', 'Q', 'V', 'X', 'I', 'S'];

/// One question type's counts for a team or quizzer across the counted games.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, ToSchema)]
pub struct QuestionTypeStat {
    pub code: String,           // one of STATS_QUESTION_TYPES
    pub attempted: i32,         // tossups answered (correct + erroneous)
    pub correct: i32,           // correct tossups
    pub bonus_attempted: i32,   // bonuses answered (correct + erroneous)
    pub bonus_correct: i32,     // correct bonuses
}

/// Running counts, indexed like [`STATS_QUESTION_TYPES`]: [attempted, correct, bonus_attempted, bonus_correct].
#[derive(Default, Clone, Copy)]
struct QuestionTypeTally([[i32; 4]; 9]);

impl QuestionTypeTally {
    /// Counts one question of round type `code` (a stored code, never M) in slot `slot`.
    fn add(&mut self, code: char, slot: usize, correct: bool) {
        let mut bump = |c: char| {
            if let Some(idx) = STATS_QUESTION_TYPES.iter().position(|t| *t == c) {
                self.0[idx][slot] += 1;
                if correct {
                    self.0[idx][slot + 1] += 1;
                }
            }
        };
        bump(code);
        if matches!(code, 'R' | 'Q' | 'V') {
            bump('M');
        }
    }

    fn merge(&mut self, other: &QuestionTypeTally) {
        for (mine, theirs) in self.0.iter_mut().zip(other.0.iter()) {
            for (m, t) in mine.iter_mut().zip(theirs.iter()) {
                *m += t;
            }
        }
    }

    fn to_stats(self) -> Vec<QuestionTypeStat> {
        STATS_QUESTION_TYPES
            .iter()
            .zip(self.0.iter())
            .map(|(code, [attempted, correct, bonus_attempted, bonus_correct])| QuestionTypeStat {
                code: code.to_string(),
                attempted: *attempted,
                correct: *correct,
                bonus_attempted: *bonus_attempted,
                bonus_correct: *bonus_correct,
            })
            .collect()
    }
}

/// Adds one game's tossups and bonuses to each quizzer's tally, keyed by (team name, quizzer
/// name) as the individual standings are. `question_types` is the game's round's stored types;
/// overtime questions, and every question of a round without types, have no type and don't count.
fn tally_question_types(
    calculated: &CalculatedGame,
    question_types: &str,
    tallies: &mut HashMap<(String, String), QuestionTypeTally>,
) {
    let types: Vec<char> = question_types.chars().collect();
    for team in calculated.teams.values() {
        for quizzer in team.quizzers.values() {
            let tally = tallies.entry((team.name.clone(), quizzer.name.clone())).or_default();
            tally_quizzer(quizzer, &types, tally);
        }
    }
}

/// Adds one quizzer's tossups and bonuses from one game to `tally`. `types[i]` is question i+1's type.
fn tally_quizzer(quizzer: &CalculatedQuizzer, types: &[char], tally: &mut QuestionTypeTally) {
    let type_of = |q: i32| if q >= 1 { types.get((q - 1) as usize).copied() } else { None };
    let groups: [(&Vec<i32>, usize, bool); 4] = [
        (&quizzer.correct_tossups, 0, true),
        (&quizzer.errors_on_tossups, 0, false),
        (&quizzer.correct_bonuses, 2, true),
        (&quizzer.errors_on_bonuses, 2, false),
    ];
    for (questions, slot, correct) in groups {
        for code in questions.iter().filter_map(|q| type_of(*q)) {
            tally.add(code, slot, correct);
        }
    }
}

/// Every question type with zero counts, for rows built before the tallies are applied.
fn empty_question_type_stats() -> Vec<QuestionTypeStat> {
    QuestionTypeTally::default().to_stats()
}

/// Each round's stored question types, for the rounds of `games`.
fn read_question_types_by_round(
    db: &mut database::Connection,
    games: &[crate::models::game::Game],
) -> QueryResult<HashMap<Uuid, String>> {
    use crate::schema::rounds::dsl as r;
    let round_ids: Vec<Uuid> = games.iter().map(|g| g.roundid).collect();
    Ok(r::rounds
        .filter(r::roundid.eq_any(&round_ids))
        .select((r::roundid, r::question_types))
        .load::<(Uuid, String)>(db)?
        .into_iter()
        .collect())
}

// ─── Quiz outs, error outs, foul outs and fouls ───────────────────────────────

/// A team's or quizzer's quiz outs, error outs, foul outs and fouls across the counted games.
/// The totals (all quiz outs, all fouls) are left to the client.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq, ToSchema)]
pub struct OutsAndFoulsStat {
    pub quiz_outs_without_errors: i32, // quiz outs with no erroneous tossup; these earn the quiz-out award
    pub quiz_outs_with_errors: i32,
    pub error_outs: i32,
    pub foul_outs: i32,
    pub quizzer_fouls: i32,
    pub team_fouls: i32,               // always 0 for a quizzer
    pub coach_fouls: i32,              // always 0 for a quizzer
}

impl OutsAndFoulsStat {
    fn merge(&mut self, other: &OutsAndFoulsStat) {
        self.quiz_outs_without_errors += other.quiz_outs_without_errors;
        self.quiz_outs_with_errors += other.quiz_outs_with_errors;
        self.error_outs += other.error_outs;
        self.foul_outs += other.foul_outs;
        self.quizzer_fouls += other.quizzer_fouls;
        self.team_fouls += other.team_fouls;
        self.coach_fouls += other.coach_fouls;
    }
}

/// One quizzer's outs and fouls in one game. `foul_out` is the game's fouls-to-foul-out; the
/// calculator doesn't record a foul-out question, so it is read off the foul count.
fn quizzer_outs_and_fouls(quizzer: &CalculatedQuizzer, foul_out: i32) -> OutsAndFoulsStat {
    let quizzed_out = quizzer.question_quizzed_out_on != -1;
    let fouls = quizzer.fouls_received.len() as i32;
    OutsAndFoulsStat {
        quiz_outs_without_errors: (quizzed_out && quizzer.errors_on_tossups.is_empty()) as i32,
        quiz_outs_with_errors: (quizzed_out && !quizzer.errors_on_tossups.is_empty()) as i32,
        error_outs: (quizzer.question_errored_out_on != -1) as i32,
        foul_outs: (foul_out > 0 && fouls >= foul_out) as i32,
        quizzer_fouls: fouls,
        ..Default::default()
    }
}

/// Adds one game's outs and fouls to each quizzer's tally, keyed by (team name, quizzer name),
/// and its team and coach fouls (which belong to no quizzer) to each team's, keyed by team name.
fn tally_outs_and_fouls(
    calculated: &CalculatedGame,
    quizzers: &mut HashMap<(String, String), OutsAndFoulsStat>,
    teams: &mut HashMap<String, OutsAndFoulsStat>,
) {
    for team in calculated.teams.values() {
        for quizzer in team.quizzers.values() {
            quizzers
                .entry((team.name.clone(), quizzer.name.clone()))
                .or_default()
                .merge(&quizzer_outs_and_fouls(quizzer, calculated.options.foul_out));
        }
        let team_tally = teams.entry(team.name.clone()).or_default();
        for (_, is_for_coach) in &team.team_and_coach_fouls_received {
            if *is_for_coach {
                team_tally.coach_fouls += 1;
            } else {
                team_tally.team_fouls += 1;
            }
        }
    }
}

// ─── A division's own stats group ─────────────────────────────────────────────

/// The stats group that mirrors a division: every game of the division, kept in step
/// automatically. Created alongside the division (see `services::division::create`).
///
/// It is told apart from any other stats group scoped to the same division by being the oldest
/// one. Today no other kind exists -- nothing else in the app creates division-scoped groups or
/// edits a group's games -- but a custom group added later (e.g. "Finals") must never be mistaken
/// for the division's own and have its games overwritten.
pub fn read_division_statsgroup_id(db: &mut database::Connection, division_id: Uuid) -> QueryResult<Option<Uuid>> {
    use crate::schema::statsgroups::dsl as sg;
    sg::statsgroups
        .filter(sg::division_id.eq(division_id))
        .filter(sg::del_fl.eq(false))
        .order((sg::created_at.asc(), sg::sgid.asc()))
        .select(sg::sgid)
        .first::<Uuid>(db)
        .optional()
}

/// Makes a division's own stats group hold exactly the division's games: links games scheduled
/// since it was last synced, and unlinks games that moved to another division. Idempotent.
///
/// Returns the group when its games changed, so a caller holding the cache can drop the group's
/// standings at once rather than wait out `AGGREGATE_TTL`. `None` when nothing changed, or the
/// division has no stats group.
pub fn sync_division_statsgroup(db: &mut database::Connection, division_id: Uuid) -> QueryResult<Option<Uuid>> {
    use std::collections::HashSet;
    let Some(sg_id) = read_division_statsgroup_id(db, division_id)? else {
        return Ok(None);
    };

    // The division's games, unpaginated: a large division must not be silently truncated.
    let wanted: HashSet<Uuid> = {
        use crate::schema::{games, pool_brackets};
        games::table
            .inner_join(pool_brackets::table.on(games::poolbracket_id.eq(pool_brackets::pool_bracket_id)))
            .filter(pool_brackets::divisionid.eq(division_id))
            .filter(games::del_fl.eq(false))
            .select(games::gid)
            .load::<Uuid>(db)?
            .into_iter()
            .collect()
    };

    use crate::schema::games_statsgroups::dsl as gs;
    let linked: HashSet<Uuid> = gs::games_statsgroups
        .filter(gs::statsgroupid.eq(sg_id))
        .select(gs::gameid)
        .load::<Uuid>(db)?
        .into_iter()
        .collect();

    let to_link: Vec<_> = wanted.difference(&linked)
        .map(|gid| (gs::gameid.eq(*gid), gs::statsgroupid.eq(sg_id)))
        .collect();
    // A soft-deleted game stays linked: the stats readers already skip deleted games, and keeping
    // the link means an undelete needs no re-sync.
    let deleted: HashSet<Uuid> = {
        use crate::schema::games::dsl as g;
        let extra: Vec<Uuid> = linked.difference(&wanted).copied().collect();
        g::games.filter(g::gid.eq_any(&extra)).filter(g::del_fl.eq(true)).select(g::gid).load::<Uuid>(db)?
            .into_iter().collect()
    };
    let to_unlink: Vec<Uuid> = linked.difference(&wanted).filter(|gid| !deleted.contains(gid)).copied().collect();

    if to_link.is_empty() && to_unlink.is_empty() {
        return Ok(None);
    }
    if !to_link.is_empty() {
        diesel::insert_into(gs::games_statsgroups)
            .values(&to_link)
            .on_conflict_do_nothing()
            .execute(db)?;
    }
    if !to_unlink.is_empty() {
        diesel::delete(
            gs::games_statsgroups
                .filter(gs::statsgroupid.eq(sg_id))
                .filter(gs::gameid.eq_any(&to_unlink)),
        ).execute(db)?;
    }
    Ok(Some(sg_id))
}

/// Every stats group whose standings depend on the division's rounds: the division's own group
/// and any other group holding one of the division's games.
pub fn read_statsgroup_ids_depending_on_division(db: &mut database::Connection, division_id: Uuid) -> QueryResult<Vec<Uuid>> {
    use crate::schema::{games, games_statsgroups, rounds};
    let mut ids: Vec<Uuid> = games_statsgroups::table
        .inner_join(games::table.on(games::gid.eq(games_statsgroups::gameid)))
        .inner_join(rounds::table.on(rounds::roundid.eq(games::roundid)))
        .filter(rounds::did.eq(division_id))
        .select(games_statsgroups::statsgroupid)
        .distinct()
        .load::<Uuid>(db)?;
    if let Some(own) = read_division_statsgroup_id(db, division_id)? {
        if !ids.contains(&own) {
            ids.push(own);
        }
    }
    Ok(ids)
}

/// [`sync_division_statsgroup`] for every division. Run once at startup, so games scheduled
/// before divisions' groups were kept in step automatically are picked up. Returns how many
/// divisions' groups changed.
pub fn sync_all_division_statsgroups(db: &mut database::Connection) -> QueryResult<usize> {
    let division_ids: Vec<Uuid> = {
        use crate::schema::statsgroups::dsl as sg;
        sg::statsgroups
            .filter(sg::del_fl.eq(false))
            .filter(sg::division_id.is_not_null())
            .select(sg::division_id.assume_not_null())
            .distinct()
            .load::<Uuid>(db)?
    };
    let mut changed = 0;
    for division_id in division_ids {
        if sync_division_statsgroup(db, division_id)?.is_some() {
            changed += 1;
        }
    }
    Ok(changed)
}

// ─── Aggregated-standings cache ───────────────────────────────────────────────

/// Cache-format version for the aggregated standings rows.
///
/// Bump when the aggregation logic changes while the row *shape* stays the same: the
/// olympic-point maths, W-L counting, placement rules or sort order. A change that alters the
/// shape of `TeamStat` or `IndividualStat` needs no bump -- a stale entry simply fails to
/// deserialize and is treated as a miss (see backend/src/cache.rs).
const STATS_VERSION: u32 = 3; // 2: only finished games count; 3: individual score excludes bonus points

/// How long aggregated standings live.
///
/// Deliberately short. The per-game `CalculatedGame` entries behind these rows last hours, so
/// recomputing after this expires is cheap: it re-aggregates cached state without touching
/// the gameevents table or the calculator. Sixty seconds is what absorbs a broadcast audience
/// refreshing at once, while capping how long the mutation paths that have no explicit
/// invalidation hook -- a game added to or removed from a stats group, a game soft-deleted --
/// can serve stale rows.
pub const AGGREGATE_TTL: std::time::Duration = std::time::Duration::from_secs(60);

/// Cache key for a stats group's team standings.
fn team_stats_cache_key(sg_id: Uuid) -> String {
    format!(
        "qv:v1:sg:{sg_id}:teamstats:{STATS_VERSION}.{}",
        crate::models::gameevent::CALC_VERSION
    )
}

/// Cache key for a stats group's individual standings.
fn individual_stats_cache_key(sg_id: Uuid) -> String {
    format!(
        "qv:v1:sg:{sg_id}:individualstats:{STATS_VERSION}.{}",
        crate::models::gameevent::CALC_VERSION
    )
}

/// Every aggregate key for one stats group, for write paths that must clear its standings.
///
/// These keys deliberately do **not** encode the tie-breaker configuration, even though the
/// ordering of `TeamStat` rows depends on it. Encoding it would make a rules edit
/// self-invalidating, but it would also mean the game-event write path could not name these
/// keys without first reading every affected group's rules -- a query per group on the
/// hottest write in the system. Tie-breaker edits get an explicit invalidation instead (see
/// `services::statsgroup::update_tiebreaker_rules`), which is a rare operation.
pub fn aggregate_cache_keys(sg_id: Uuid) -> Vec<String> {
    vec![team_stats_cache_key(sg_id), individual_stats_cache_key(sg_id)]
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
    pub avg_points: f64,            // total_points / games (0 when no games were scored)
    /// Correct tossups on M (Memory Verse) questions across the counted games. Always 0 when the
    /// Memory Verse criterion isn't available to the stats group.
    pub memory_verse_correct: i32,
    /// Tossup and bonus counts per question type, in [`STATS_QUESTION_TYPES`] order. Summed over
    /// the team's quizzers; questions without a type don't count.
    pub question_types: Vec<QuestionTypeStat>,
    /// Quiz outs, error outs, foul outs and quizzer fouls summed over the team's quizzers, plus
    /// the team's own team and coach fouls.
    pub outs_and_fouls: OutsAndFoulsStat,
    /// Which criterion placed this team below the one above it. None on the first row.
    pub rank_rule_above: Option<RankRuleExplanation>,
    /// Which criterion placed this team above the one below it. None on the last row.
    pub rank_rule_below: Option<RankRuleExplanation>,
}

/// Why one team sits above another, for the "Rank Rules" column.
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct RankRuleExplanation {
    pub number: i32,            // 1-based position of the criterion in the configured sequence
    pub rule: TieBreakerRule,
    pub label: String,          // the criterion's display label
    pub other_team: String,     // the team on the other side of the comparison
    pub detail: String,         // the values that decided it, e.g. "6-2 vs 4-4"
}

struct TeamStatAgg {
    games: i32,
    wins: i32,
    losses: i32,
    olympic_points: i32,
    mod_olympic_points: i32,
    total_points: i32,
    memory_verse_correct: i32,
}

/// One scored game reduced to the names and ranks needed for head-to-head.
struct GameRanks {
    ranks: Vec<(String, i32)>,  // (team name, placement rank)
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

/// Computes team standings for a statsgroup by running the score calculator over each of the
/// statsgroup's games and aggregating per team (keyed by team name). Only finished games count
/// (see `CalculatedGame::is_finished`); unfinished ones, and games whose event stream can't be
/// scored, are skipped.
///
/// Within a game a team's W-L is counted per opponent, so a 3-team round gives 1st 2 wins,
/// 2nd 1 win and 1 loss, and 3rd 2 losses; a 2-team round gives 1-0 and 0-1.
///
/// Ordering applies the stats group's enabled tie-breaker criteria in their configured order,
/// falling back to team name so the result is stable. Each row also carries which criterion
/// separated it from its neighbours, for the "Rank Rules" column.
pub async fn read_team_stats_of_statsgroup(
    cache: &Cache,
    db: &mut database::Connection,
    sg_id: Uuid,
) -> QueryResult<Vec<TeamStat>> {
    // Fast path: fully aggregated rows. A hit here costs one Valkey round trip and zero
    // database queries. (The caller has already authorized the request -- access checks are
    // never served from cache.)
    let aggregate_key = team_stats_cache_key(sg_id);
    if let Some(cached) = cache.get_json::<Vec<TeamStat>>(&aggregate_key).await {
        return Ok(cached);
    }

    // Unavailable criteria come back unchecked, so they're skipped below.
    let settings = read_tiebreaker_rules(db, sg_id)?;
    let memory_verse_available = settings
        .iter()
        .any(|s| s.rule == TieBreakerRule::MemoryVerseCorrect && s.available);

    let pagination = PaginationParams { page: 0, page_size: PaginationParams::MAX_PAGE_SIZE as i64 };
    let games = crate::models::game::read_all_games_of_statsgroup(db, sg_id, &pagination)?;

    // Each game's round's question types, for the Memory Verse counts and the per-type breakdown.
    let question_types_by_round = read_question_types_by_round(db, &games)?;
    let mut tallies: HashMap<(String, String), QuestionTypeTally> = HashMap::new();
    let mut quizzer_outs: HashMap<(String, String), OutsAndFoulsStat> = HashMap::new();
    let mut team_outs: HashMap<String, OutsAndFoulsStat> = HashMap::new();

    // One cache lookup (or one replay) per game. Games whose event stream can't be scored
    // come back as None and are skipped.
    let mut per_game: Vec<Vec<TeamGameResult>> = Vec::with_capacity(games.len());
    let mut memory_verse_per_game: Vec<HashMap<i32, i32>> = Vec::with_capacity(games.len());
    for game in games {
        if let Some(calculated) =
            crate::models::gameevent::calculate_game_cached(cache, db, game.gid).await?
        {
            // Only finished games count: a game still being played would credit whoever leads
            // it now, and standings would then jump around and reverse during a round.
            if calculated.is_finished() {
                per_game.push(crate::models::gameevent::team_results_from_calculated(&calculated));
                let types = question_types_by_round.get(&game.roundid);
                memory_verse_per_game.push(match types {
                    Some(types) if memory_verse_available => memory_verse_correct_by_team(&calculated, types),
                    _ => HashMap::new(),
                });
                if let Some(types) = types {
                    tally_question_types(&calculated, types, &mut tallies);
                }
                tally_outs_and_fouls(&calculated, &mut quizzer_outs, &mut team_outs);
            }
        }
    }

    let mut stats = aggregate_team_stats(&per_game, &memory_verse_per_game, &settings);
    let mut by_team: HashMap<String, QuestionTypeTally> = HashMap::new();
    for ((team_name, _), tally) in &tallies {
        by_team.entry(team_name.clone()).or_default().merge(tally);
    }
    for ((team_name, _), outs) in &quizzer_outs {
        team_outs.entry(team_name.clone()).or_default().merge(outs);
    }
    for stat in stats.iter_mut() {
        if let Some(tally) = by_team.get(&stat.name) {
            stat.question_types = tally.to_stats();
        }
        if let Some(outs) = team_outs.get(&stat.name) {
            stat.outs_and_fouls = *outs;
        }
    }
    cache.set_json(&aggregate_key, &stats, AGGREGATE_TTL).await;
    Ok(stats)
}

/// The team standings of a division: those of its own stats group (see
/// [`read_division_statsgroup_id`]). Empty when the division has no stats group.
///
/// Served by the public division standings endpoint and the live LIVE Stats page. It reads
/// through the same cache as [`read_team_stats_of_statsgroup`], so the admin Stats Groups panel
/// and the public page always agree.
pub async fn read_team_stats_of_division(
    cache: &Cache,
    db: &mut database::Connection,
    division_id: Uuid,
) -> QueryResult<Vec<TeamStat>> {
    match read_division_statsgroup_id(db, division_id)? {
        Some(sg_id) => read_team_stats_of_statsgroup(cache, db, sg_id).await,
        None => Ok(Vec::new()),
    }
}

/// Aggregates per-game team results into the sorted, placed `TeamStat` rows the endpoint
/// serves. Pure: no database, no cache, no scoring -- which makes the olympic-point and
/// tie-breaker behaviour unit-testable without a game event in sight.
///
/// `memory_verse_per_game[i]` holds game `i`'s Memory Verse counts by team index; a missing game
/// or team counts 0.
fn aggregate_team_stats(
    per_game: &[Vec<TeamGameResult>],
    memory_verse_per_game: &[HashMap<i32, i32>],
    settings: &[TieBreakerSetting],
) -> Vec<TeamStat> {
    use std::collections::HashMap;

    let mut agg: HashMap<String, TeamStatAgg> = HashMap::new();
    let mut game_ranks: Vec<GameRanks> = Vec::with_capacity(per_game.len());

    for (game_idx, results) in per_game.iter().enumerate() {
        let memory_verse = memory_verse_per_game.get(game_idx);
        // Round size and the round's total team score (for the modified "10% of total" clause).
        let num_teams = results.len();
        let game_total: i32 = results.iter().map(|r| r.score).sum();

        game_ranks.push(GameRanks {
            ranks: results
                .iter()
                .filter(|r| !r.name.trim().is_empty())
                .map(|r| (r.name.clone(), r.rank))
                .collect(),
        });

        for result in results {
            if result.name.trim().is_empty() {
                continue;
            }
            let (olympic, mod_olympic) = olympic_awards(num_teams, result.rank, result.score, game_total);
            let entry = agg.entry(result.name.clone()).or_insert(TeamStatAgg {
                games: 0, wins: 0, losses: 0, olympic_points: 0, mod_olympic_points: 0, total_points: 0,
                memory_verse_correct: 0,
            });
            entry.games += 1;
            entry.memory_verse_correct += memory_verse.and_then(|m| m.get(&result.team)).copied().unwrap_or(0);
            entry.total_points += result.score;
            // One win per team this team finished ahead of, one loss per team that finished ahead.
            for other in results {
                if other.team == result.team || other.name.trim().is_empty() {
                    continue;
                }
                if other.rank > result.rank {
                    entry.wins += 1;
                } else if other.rank < result.rank {
                    entry.losses += 1;
                }
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
            avg_points: if a.games > 0 { a.total_points as f64 / a.games as f64 } else { 0.0 },
            memory_verse_correct: a.memory_verse_correct,
            question_types: empty_question_type_stats(),
            outs_and_fouls: OutsAndFoulsStat::default(),
            rank_rule_above: None,
            rank_rule_below: None,
        })
        .collect();

    // Order by the configured criteria, with the team name as the final stable tiebreak.
    stats.sort_by(|a, b| {
        decide_order(a, b, settings, &game_ranks)
            .map(|(_, _, ordering)| ordering)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(a.name.cmp(&b.name))
    });

    // Record which criterion separated each adjacent pair, for the "Rank Rules" column.
    let decisions: Vec<Option<(usize, TieBreakerRule, String, String)>> = stats
        .windows(2)
        .map(|pair| {
            decide_order(&pair[0], &pair[1], settings, &game_ranks).map(|(idx, rule, _)| {
                (idx, rule, describe_rule_values(&pair[0], rule, &game_ranks, &pair[1].name),
                 describe_rule_values(&pair[1], rule, &game_ranks, &pair[0].name))
            })
            // `idx` is the criterion's 0-based slot in the configured sequence.
        })
        .collect();

    for (idx, decision) in decisions.into_iter().enumerate() {
        if let Some((rule_idx, rule, upper_detail, lower_detail)) = decision {
            let number = rule_idx as i32 + 1;
            let upper_name = stats[idx].name.clone();
            let lower_name = stats[idx + 1].name.clone();
            stats[idx].rank_rule_below = Some(RankRuleExplanation {
                number,
                rule,
                label: rule.label().to_string(),
                other_team: lower_name,
                detail: format!("{} vs {}", upper_detail, lower_detail),
            });
            stats[idx + 1].rank_rule_above = Some(RankRuleExplanation {
                number,
                rule,
                label: rule.label().to_string(),
                other_team: upper_name,
                detail: format!("{} vs {}", lower_detail, upper_detail),
            });
        }
    }

    for (idx, stat) in stats.iter_mut().enumerate() {
        stat.place = (idx + 1) as i32;
    }

    stats
}

/// Applies the enabled criteria in order and returns the first one that separates `a` and `b`,
/// as (index into `settings`, the criterion, the ordering). None when every criterion ties.
fn decide_order(
    a: &TeamStat,
    b: &TeamStat,
    settings: &[TieBreakerSetting],
    game_ranks: &[GameRanks],
) -> Option<(usize, TieBreakerRule, std::cmp::Ordering)> {
    for (idx, setting) in settings.iter().enumerate() {
        if !setting.enabled {
            continue;
        }
        let ordering = compare_by_rule(a, b, setting.rule, game_ranks);
        if ordering != std::cmp::Ordering::Equal {
            return Some((idx, setting.rule, ordering));
        }
    }
    None
}

/// Compares two teams by one criterion. Returns Less when `a` ranks ahead of `b`.
fn compare_by_rule(
    a: &TeamStat,
    b: &TeamStat,
    rule: TieBreakerRule,
    game_ranks: &[GameRanks],
) -> std::cmp::Ordering {
    match rule {
        // More wins first; on equal wins, fewer losses first.
        TieBreakerRule::TeamWinLoss => b.wins.cmp(&a.wins).then(a.losses.cmp(&b.losses)),
        TieBreakerRule::MemoryVerseCorrect => b.memory_verse_correct.cmp(&a.memory_verse_correct),
        TieBreakerRule::HeadToHead => {
            let (a_wins, b_wins) = head_to_head(&a.name, &b.name, game_ranks);
            b_wins.cmp(&a_wins)
        }
        TieBreakerRule::TotalPoints => b.total_points.cmp(&a.total_points),
        TieBreakerRule::AvgPoints => b.avg_points.partial_cmp(&a.avg_points).unwrap_or(std::cmp::Ordering::Equal),
        TieBreakerRule::OlympicPoints => b.olympic_points.cmp(&a.olympic_points),
        TieBreakerRule::ModOlympicPoints => b.mod_olympic_points.cmp(&a.mod_olympic_points),
    }
}

/// Wins for each side across only the games both teams played in.
fn head_to_head(a_name: &str, b_name: &str, game_ranks: &[GameRanks]) -> (i32, i32) {
    let mut a_wins = 0;
    let mut b_wins = 0;
    for game in game_ranks {
        let a_rank = game.ranks.iter().find(|(n, _)| n == a_name).map(|(_, r)| *r);
        let b_rank = game.ranks.iter().find(|(n, _)| n == b_name).map(|(_, r)| *r);
        if let (Some(ar), Some(br)) = (a_rank, b_rank) {
            if ar < br {
                a_wins += 1;
            } else if br < ar {
                b_wins += 1;
            }
        }
    }
    (a_wins, b_wins)
}

/// The value that `rule` compared for `stat`, rendered for the explanation dialog.
fn describe_rule_values(
    stat: &TeamStat,
    rule: TieBreakerRule,
    game_ranks: &[GameRanks],
    opponent: &str,
) -> String {
    match rule {
        TieBreakerRule::TeamWinLoss => format!("{}-{}", stat.wins, stat.losses),
        TieBreakerRule::MemoryVerseCorrect => stat.memory_verse_correct.to_string(),
        TieBreakerRule::HeadToHead => {
            let (mine, theirs) = head_to_head(&stat.name, opponent, game_ranks);
            format!("{}-{}", mine, theirs)
        }
        TieBreakerRule::TotalPoints => stat.total_points.to_string(),
        TieBreakerRule::AvgPoints => format!("{:.1}", stat.avg_points),
        TieBreakerRule::OlympicPoints => stat.olympic_points.to_string(),
        TieBreakerRule::ModOlympicPoints => stat.mod_olympic_points.to_string(),
    }
}

// ─── Individual (quizzer) standings for a statsgroup ──────────────────────────

/// Aggregated individual-stats row for one quizzer within a statsgroup.
/// The UI's Errs 16+/5+ column is intentionally left out of this computation for now.
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct IndividualStat {
    pub place: i32,             // 1-based rank within the statsgroup (by score)
    pub individual: String,     // quizzer name
    pub team_name: String,      // the quizzer's team name
    pub games: i32,             // number of (scored) games the quizzer played
    pub score: i32,             // individual points across games, excluding bonus points (see bonus_pts)
    pub avg: f64,               // score / games
    pub correct: i32,           // correct tossups
    pub errors: i32,            // erroneous tossups
    pub bonus_pts: i32,         // points from correct bonuses
    pub bonus_attempts: i32,    // correct + erroneous bonus attempts
    /// Tossup and bonus counts per question type, in [`STATS_QUESTION_TYPES`] order. Questions
    /// without a type don't count.
    pub question_types: Vec<QuestionTypeStat>,
    /// Quiz outs, error outs, foul outs and fouls; the team and coach fouls are always 0.
    pub outs_and_fouls: OutsAndFoulsStat,
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
pub async fn read_individual_stats_of_statsgroup(
    cache: &Cache,
    db: &mut database::Connection,
    sg_id: Uuid,
) -> QueryResult<Vec<IndividualStat>> {
    // Fast path: fully aggregated rows, as in `read_team_stats_of_statsgroup`.
    let aggregate_key = individual_stats_cache_key(sg_id);
    if let Some(cached) = cache.get_json::<Vec<IndividualStat>>(&aggregate_key).await {
        return Ok(cached);
    }

    let pagination = PaginationParams { page: 0, page_size: PaginationParams::MAX_PAGE_SIZE as i64 };
    let games = crate::models::game::read_all_games_of_statsgroup(db, sg_id, &pagination)?;
    let question_types_by_round = read_question_types_by_round(db, &games)?;
    let mut tallies: HashMap<(String, String), QuestionTypeTally> = HashMap::new();
    let mut quizzer_outs: HashMap<(String, String), OutsAndFoulsStat> = HashMap::new();
    // Team and coach fouls belong to no quizzer, so the individual rows leave them out.
    let mut team_outs: HashMap<String, OutsAndFoulsStat> = HashMap::new();

    // Note this shares the cache with the team-stats endpoint: whichever of the two is
    // requested first warms every game, and the second is served without touching the
    // gameevents table or the calculator at all.
    let mut per_game: Vec<Vec<QuizzerGameResult>> = Vec::with_capacity(games.len());
    for game in games {
        if let Some(calculated) =
            crate::models::gameevent::calculate_game_cached(cache, db, game.gid).await?
        {
            // Only finished games count: a game still being played would credit whoever leads
            // it now, and standings would then jump around and reverse during a round.
            if calculated.is_finished() {
                per_game.push(crate::models::gameevent::quizzer_results_from_calculated(&calculated));
                if let Some(types) = question_types_by_round.get(&game.roundid) {
                    tally_question_types(&calculated, types, &mut tallies);
                }
                tally_outs_and_fouls(&calculated, &mut quizzer_outs, &mut team_outs);
            }
        }
    }

    let mut stats = aggregate_individual_stats(&per_game);
    for stat in stats.iter_mut() {
        let key = (stat.team_name.clone(), stat.individual.clone());
        if let Some(tally) = tallies.get(&key) {
            stat.question_types = tally.to_stats();
        }
        if let Some(outs) = quizzer_outs.get(&key) {
            stat.outs_and_fouls = *outs;
        }
    }
    cache.set_json(&aggregate_key, &stats, AGGREGATE_TTL).await;
    Ok(stats)
}

/// Aggregates per-game quizzer results into the sorted, placed `IndividualStat` rows the
/// endpoint serves. Pure: no database, no cache, no scoring.
fn aggregate_individual_stats(per_game: &[Vec<QuizzerGameResult>]) -> Vec<IndividualStat> {
    use std::collections::HashMap;

    // Keyed by (team name, quizzer name) so quizzers with the same name on
    // different teams are not merged.
    let mut agg: HashMap<(String, String), IndividualStatAgg> = HashMap::new();

    for results in per_game {
        for result in results {
            if result.name.trim().is_empty() {
                continue;
            }
            let entry = agg.entry((result.team_name.clone(), result.name.clone())).or_insert(IndividualStatAgg {
                games: 0, score: 0, correct: 0, errors: 0, bonus_pts: 0, bonus_attempts: 0,
            });
            entry.games += 1;
            // A quizzer's individual score leaves out bonus points; those are reported on their
            // own in `bonus_pts`. The per-game score includes them, so take them back out here.
            entry.score += result.score - result.bonus_pts;
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
            question_types: empty_question_type_stats(),
            outs_and_fouls: OutsAndFoulsStat::default(),
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

    stats
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

#[cfg(test)]
mod aggregation_tests {
    use super::*;

    fn team(team: i32, name: &str, score: i32, rank: i32) -> TeamGameResult {
        TeamGameResult { team, name: name.to_string(), score, rank }
    }

    fn quizzer(team_name: &str, name: &str, score: i32, correct: i32, errors: i32) -> QuizzerGameResult {
        QuizzerGameResult {
            team_name: team_name.to_string(),
            name: name.to_string(),
            score,
            correct,
            errors,
            bonus_pts: 0,
            bonus_attempts: 0,
        }
    }

    /// W-L is counted per opponent, so a 2-team round gives 1-0 and 0-1 and a 3-team round
    /// gives 2-0, 1-1 and 0-2. This is the rule the tie-breakers sort on, so it is the one
    /// the refactor most needed to preserve.
    #[test]
    fn wins_and_losses_are_counted_per_opponent() {
        let settings = parse_tiebreaker_rules("");

        let two_team = vec![vec![team(0, "Red", 100, 1), team(1, "Blue", 80, 2)]];
        let stats = aggregate_team_stats(&two_team, &[], &settings);
        let red = stats.iter().find(|s| s.name == "Red").unwrap();
        let blue = stats.iter().find(|s| s.name == "Blue").unwrap();
        assert_eq![(red.wins, red.losses), (1, 0)];
        assert_eq![(blue.wins, blue.losses), (0, 1)];

        let three_team = vec![vec![
            team(0, "Red", 100, 1),
            team(1, "Blue", 80, 2),
            team(2, "Green", 60, 3),
        ]];
        let stats = aggregate_team_stats(&three_team, &[], &settings);
        for (name, expected) in [("Red", (2, 0)), ("Blue", (1, 1)), ("Green", (0, 2))] {
            let row = stats.iter().find(|s| s.name == name).unwrap();
            assert_eq![(row.wins, row.losses), expected, "{name}"];
        }
    }

    /// Rows accumulate across games and `place` is assigned 1..n in sorted order.
    #[test]
    fn totals_accumulate_across_games_and_places_are_assigned() {
        let settings = parse_tiebreaker_rules("");
        let per_game = vec![
            vec![team(0, "Red", 100, 1), team(1, "Blue", 80, 2)],
            vec![team(0, "Red", 60, 2), team(1, "Blue", 90, 1)],
        ];

        let stats = aggregate_team_stats(&per_game, &[], &settings);
        assert_eq![stats.len(), 2];
        assert_eq![stats.iter().map(|s| s.place).collect::<Vec<_>>(), vec![1, 2]];

        let red = stats.iter().find(|s| s.name == "Red").unwrap();
        assert_eq![red.games, 2];
        assert_eq![red.total_points, 160];
        assert_eq![(red.wins, red.losses), (1, 1)];
        assert_eq![red.avg_points, 80.0];
    }

    /// Unnamed teams and quizzers are placeholders in a partially set-up game and must not
    /// become rows -- nor inflate a real team's W-L.
    #[test]
    fn blank_names_are_excluded_everywhere() {
        let settings = parse_tiebreaker_rules("");
        let per_game = vec![vec![
            team(0, "Red", 100, 1),
            team(1, "   ", 0, 2),
        ]];

        let stats = aggregate_team_stats(&per_game, &[], &settings);
        assert_eq![stats.len(), 1];
        assert_eq![stats[0].name, "Red"];
        assert_eq![(stats[0].wins, stats[0].losses), (0, 0),
            "a blank opponent must not count as a win"];

        let individuals = aggregate_individual_stats(&vec![vec![
            quizzer("Red", "Jacob", 40, 2, 0),
            quizzer("Red", "", 0, 0, 0),
        ]]);
        assert_eq![individuals.len(), 1];
        assert_eq![individuals[0].individual, "Jacob"];
    }

    /// Same quizzer name on two different teams must stay two rows.
    #[test]
    fn identically_named_quizzers_on_different_teams_are_not_merged() {
        let per_game = vec![vec![
            quizzer("Red", "Alex", 40, 2, 0),
            quizzer("Blue", "Alex", 20, 1, 0),
        ]];
        let stats = aggregate_individual_stats(&per_game);
        assert_eq![stats.len(), 2];
        assert_eq![stats[0].score, 40];
        assert_eq![stats[0].team_name, "Red"];
        assert_eq![stats[1].team_name, "Blue"];
    }

    /// Individuals level on (score, errors) share a place, and the next distinct quizzer's
    /// place is its position in the list -- not the next integer.
    #[test]
    fn individual_places_are_competitive() {
        let per_game = vec![vec![
            quizzer("Red", "Ann", 40, 2, 0),
            quizzer("Red", "Bob", 40, 2, 0),
            quizzer("Red", "Cal", 20, 1, 0),
        ]];
        let stats = aggregate_individual_stats(&per_game);
        assert_eq![
            stats.iter().map(|s| (s.individual.as_str(), s.place)).collect::<Vec<_>>(),
            vec![("Ann", 1), ("Bob", 1), ("Cal", 3)]
        ];
    }

    /// Bonus points are reported in `bonus_pts` but kept out of the individual score, so they
    /// don't affect the average or the placings either.
    #[test]
    fn individual_score_excludes_bonus_points() {
        let with_bonus = |name: &str, score: i32, bonus_pts: i32| QuizzerGameResult {
            bonus_pts,
            bonus_attempts: bonus_pts / 10,
            ..quizzer("Red", name, score, 2, 0)
        };
        // Ann: 40 tossup points + 20 bonus. Bob: 50 tossup points, no bonus.
        let stats = aggregate_individual_stats(&vec![vec![with_bonus("Ann", 60, 20), with_bonus("Bob", 50, 0)]]);
        assert_eq![
            stats.iter().map(|s| (s.individual.as_str(), s.score, s.bonus_pts, s.place)).collect::<Vec<_>>(),
            vec![("Bob", 50, 0, 1), ("Ann", 40, 20, 2)]
        ];
        assert_eq![stats[1].avg, 40.0];
    }

    /// Individual counts sum across games.
    #[test]
    fn individual_counts_accumulate_across_games() {
        let per_game = vec![
            vec![quizzer("Red", "Jacob", 40, 2, 1)],
            vec![quizzer("Red", "Jacob", 60, 3, 0)],
        ];
        let stats = aggregate_individual_stats(&per_game);
        assert_eq![stats.len(), 1];
        assert_eq![stats[0].games, 2];
        assert_eq![stats[0].score, 100];
        assert_eq![stats[0].correct, 5];
        assert_eq![stats[0].errors, 1];
        assert_eq![stats[0].avg, 50.0];
    }

    /// The two aggregate keys must be distinct, name their stats group, and carry both
    /// version tokens -- a collision between them would serve team rows as individual rows.
    #[test]
    fn aggregate_cache_keys_are_distinct_and_versioned() {
        let sg_id = Uuid::new_v4();
        let keys = aggregate_cache_keys(sg_id);

        assert_eq![keys.len(), 2];
        assert_ne![keys[0], keys[1], "team and individual keys must not collide"];

        for key in &keys {
            assert![key.contains(&sg_id.to_string()), "{key} should name its stats group"];
            assert![key.contains(&STATS_VERSION.to_string()), "{key} should carry STATS_VERSION"];
            assert![
                key.contains(&crate::models::gameevent::CALC_VERSION.to_string()),
                "{key} should carry CALC_VERSION, since the rows derive from calculated games"
            ];
        }

        // Different stats groups must never share a key.
        assert_ne![aggregate_cache_keys(Uuid::new_v4())[0], keys[0]];
    }

    /// `TeamStat` is the only cached payload with a nested enum (`TieBreakerRule`, inside the
    /// optional `RankRuleExplanation`s), so it is the one most likely to break serialization.
    #[test]
    fn team_stat_rows_survive_a_json_round_trip() {
        let settings = parse_tiebreaker_rules("");
        let rows = aggregate_team_stats(
            &vec![vec![team(0, "Red", 100, 1), team(1, "Blue", 80, 2)]],
            &[],
            &settings,
        );

        // Guard against a vacuous test: the pair must actually have been separated.
        assert![rows[0].rank_rule_below.is_some(), "expected a rank-rule explanation"];
        assert![rows[1].rank_rule_above.is_some(), "expected a rank-rule explanation"];

        let json = serde_json::to_string(&rows).expect("TeamStat rows must serialize");
        let revived: Vec<TeamStat> =
            serde_json::from_str(&json).expect("TeamStat rows must deserialize");

        assert_eq![revived.len(), rows.len()];
        for (before, after) in rows.iter().zip(revived.iter()) {
            assert_eq![before.name, after.name];
            assert_eq![before.place, after.place];
            assert_eq![(before.wins, before.losses), (after.wins, after.losses)];
            assert_eq![before.avg_points, after.avg_points];
            // The nested explanation, including its enum rule, has to survive intact.
            match (&before.rank_rule_below, &after.rank_rule_below) {
                (Some(b), Some(a)) => {
                    assert_eq![b.rule, a.rule];
                    assert_eq![b.number, a.number];
                    assert_eq![b.label, a.label];
                    assert_eq![b.other_team, a.other_team];
                    assert_eq![b.detail, a.detail];
                }
                (None, None) => {}
                _ => panic!["rank_rule_below changed presence across the round trip"],
            }
        }
    }

    /// `IndividualStat` rows round-trip too.
    #[test]
    fn individual_stat_rows_survive_a_json_round_trip() {
        let rows = aggregate_individual_stats(&vec![vec![
            quizzer("Red", "Jacob", 40, 2, 1),
            quizzer("Blue", "Audrey", 60, 3, 0),
        ]]);
        assert_eq![rows.len(), 2];

        let json = serde_json::to_string(&rows).unwrap();
        let revived: Vec<IndividualStat> = serde_json::from_str(&json).unwrap();

        for (before, after) in rows.iter().zip(revived.iter()) {
            assert_eq![before.individual, after.individual];
            assert_eq![before.team_name, after.team_name];
            assert_eq![before.place, after.place];
            assert_eq![before.score, after.score];
            assert_eq![before.avg, after.avg];
        }
    }

    /// End-to-end through a real Valkey: store standings, read them back, invalidate the way
    /// the write paths do, confirm both keys are gone. Opt-in; see `crate::cache` for the
    /// container one-liner.
    #[actix_web::test]
    #[ignore = "needs a running Valkey; set VALKEY_TEST_URL"]
    async fn standings_round_trip_and_invalidate_through_valkey() {
        let Ok(url) = std::env::var("VALKEY_TEST_URL") else {
            eprintln!("VALKEY_TEST_URL not set; skipping.");
            return;
        };
        let cache = Cache::connect(&url).await;
        assert![cache.is_enabled(), "expected to connect to Valkey at {url}"];

        let sg_id = Uuid::new_v4();
        let settings = parse_tiebreaker_rules("");
        let team_rows = aggregate_team_stats(
            &vec![vec![team(0, "Red", 100, 1), team(1, "Blue", 80, 2)]],
            &[],
            &settings,
        );
        let individual_rows = aggregate_individual_stats(&vec![vec![
            quizzer("Red", "Jacob", 40, 2, 0),
        ]]);

        let team_key = team_stats_cache_key(sg_id);
        let individual_key = individual_stats_cache_key(sg_id);

        // Cold.
        assert![cache.get_json::<Vec<TeamStat>>(&team_key).await.is_none()];

        cache.set_json(&team_key, &team_rows, AGGREGATE_TTL).await;
        cache.set_json(&individual_key, &individual_rows, AGGREGATE_TTL).await;

        let revived_teams: Vec<TeamStat> = cache
            .get_json(&team_key)
            .await
            .expect("team standings should read back");
        assert_eq![
            revived_teams.iter().map(|r| (r.name.clone(), r.place)).collect::<Vec<_>>(),
            team_rows.iter().map(|r| (r.name.clone(), r.place)).collect::<Vec<_>>()
        ];
        assert![
            revived_teams[0].rank_rule_below.is_some(),
            "the rank-rule explanation must survive Valkey, not just serde"
        ];

        let revived_individuals: Vec<IndividualStat> = cache
            .get_json(&individual_key)
            .await
            .expect("individual standings should read back");
        assert_eq![revived_individuals.len(), 1];
        assert_eq![revived_individuals[0].individual, "Jacob"];

        // What both write paths do: one call clears every aggregate key for the group.
        cache.invalidate(&aggregate_cache_keys(sg_id)).await;
        assert![
            cache.get_json::<Vec<TeamStat>>(&team_key).await.is_none(),
            "team standings should have been invalidated"
        ];
        assert![
            cache.get_json::<Vec<IndividualStat>>(&individual_key).await.is_none(),
            "individual standings should have been invalidated"
        ];
    }

    /// A stats group with no scoreable games yields no rows rather than panicking -- the
    /// `windows(2)` tie-breaker pass has to tolerate 0 and 1 rows.
    #[test]
    fn empty_and_single_row_inputs_are_safe() {
        let settings = parse_tiebreaker_rules("");
        assert![aggregate_team_stats(&[], &[], &settings).is_empty()];
        assert![aggregate_individual_stats(&[]).is_empty()];

        let one = vec![vec![team(0, "Red", 100, 1)]];
        let stats = aggregate_team_stats(&one, &[], &settings);
        assert_eq![stats.len(), 1];
        assert![stats[0].rank_rule_above.is_none()];
        assert![stats[0].rank_rule_below.is_none()];
    }
}

#[cfg(test)]
mod tiebreaker_tests {
    use super::*;

    fn stat(name: &str, wins: i32, losses: i32, total: i32, games: i32, oly: i32, mod_oly: i32) -> TeamStat {
        TeamStat {
            place: 0,
            name: name.to_string(),
            games,
            wins,
            losses,
            olympic_points: oly,
            mod_olympic_points: mod_oly,
            total_points: total,
            avg_points: if games > 0 { total as f64 / games as f64 } else { 0.0 },
            memory_verse_correct: 0,
            question_types: empty_question_type_stats(),
            outs_and_fouls: OutsAndFoulsStat::default(),
            rank_rule_above: None,
            rank_rule_below: None,
        }
    }

    #[test]
    fn default_order_is_the_documented_sequence() {
        let settings = parse_tiebreaker_rules("");
        let keys: Vec<&str> = settings.iter().map(|s| s.rule.key()).collect();
        assert_eq!(keys, vec!["team_wl", "memory_verse", "head_to_head", "total_points", "avg_points", "olympic", "mod_olympic"]);
        assert!(!settings[1].enabled, "Memory Verse starts unchecked");
        assert!(settings.iter().filter(|s| s.rule != TieBreakerRule::MemoryVerseCorrect).all(|s| s.enabled));
    }

    #[test]
    fn memory_verse_is_added_unchecked_below_team_wl_for_groups_saved_before_it_existed() {
        let stored = "total_points,-team_wl,head_to_head,avg_points,olympic,mod_olympic";
        let settings = parse_tiebreaker_rules(stored);
        let keys: Vec<&str> = settings.iter().map(|s| s.rule.key()).collect();
        assert_eq!(keys, vec!["total_points", "team_wl", "memory_verse", "head_to_head", "avg_points", "olympic", "mod_olympic"]);
        assert!(!settings[2].enabled);
    }

    #[test]
    fn memory_verse_counts_decide_and_explain_the_order() {
        let mut red = stat("Red", 2, 2, 100, 4, 10, 10);
        let mut blue = stat("Blue", 2, 2, 100, 4, 10, 10);
        red.memory_verse_correct = 1;
        blue.memory_verse_correct = 3;
        let settings = parse_tiebreaker_rules("team_wl,memory_verse");
        let (idx, rule, ordering) = decide_order(&red, &blue, &settings, &[]).unwrap();
        assert_eq!(rule, TieBreakerRule::MemoryVerseCorrect);
        assert_eq!(idx + 1, 2);
        assert_eq!(ordering, std::cmp::Ordering::Greater, "Blue got more M questions right");
        assert_eq!(describe_rule_values(&blue, rule, &[], "Red"), "3");
    }

    #[test]
    fn memory_verse_counts_add_up_across_games_by_team_index() {
        let per_game = vec![
            vec![TeamGameResult { team: 0, name: "Red".into(), score: 100, rank: 1 }, TeamGameResult { team: 1, name: "Blue".into(), score: 80, rank: 2 }],
            vec![TeamGameResult { team: 0, name: "Blue".into(), score: 100, rank: 1 }, TeamGameResult { team: 1, name: "Red".into(), score: 80, rank: 2 }],
        ];
        let memory = vec![HashMap::from([(0, 1), (1, 2)]), HashMap::from([(0, 1)])];
        let stats = aggregate_team_stats(&per_game, &memory, &parse_tiebreaker_rules(""));
        let red = stats.iter().find(|s| s.name == "Red").unwrap();
        let blue = stats.iter().find(|s| s.name == "Blue").unwrap();
        assert_eq!(red.memory_verse_correct, 1);
        assert_eq!(blue.memory_verse_correct, 3);
    }

    #[test]
    fn disabled_criteria_keep_their_slot_through_a_round_trip() {
        let stored = "total_points,-team_wl,-memory_verse,head_to_head,avg_points,olympic,mod_olympic";
        let settings = parse_tiebreaker_rules(stored);
        assert_eq!(settings[0].rule, TieBreakerRule::TotalPoints);
        assert_eq!(settings[1].rule, TieBreakerRule::TeamWinLoss);
        assert!(!settings[1].enabled, "the unchecked criterion stays in position 2");
        assert_eq!(serialize_tiebreaker_rules(&settings), stored);
    }

    #[test]
    fn a_criterion_absent_from_a_stored_value_is_appended_with_its_default() {
        let settings = parse_tiebreaker_rules("total_points,team_wl");
        let keys: Vec<&str> = settings.iter().map(|s| s.rule.key()).collect();
        assert_eq!(keys, vec!["total_points", "team_wl", "memory_verse", "head_to_head", "avg_points", "olympic", "mod_olympic"]);
        assert!(settings.iter().all(|s| s.enabled == s.rule.enabled_by_default()));
    }

    #[test]
    fn reordering_changes_which_team_leads() {
        // Red has the better record; Blue has more points.
        let red = stat("Red", 4, 0, 100, 4, 32, 32);
        let blue = stat("Blue", 2, 2, 200, 4, 22, 22);

        let by_record = parse_tiebreaker_rules("team_wl,total_points");
        let (_, rule, ordering) = decide_order(&red, &blue, &by_record, &[]).unwrap();
        assert_eq!(rule, TieBreakerRule::TeamWinLoss);
        assert_eq!(ordering, std::cmp::Ordering::Less, "Red leads on record");

        let by_points = parse_tiebreaker_rules("total_points,team_wl");
        let (_, rule, ordering) = decide_order(&red, &blue, &by_points, &[]).unwrap();
        assert_eq!(rule, TieBreakerRule::TotalPoints);
        assert_eq!(ordering, std::cmp::Ordering::Greater, "with points first, Blue leads");
    }

    #[test]
    fn an_unchecked_criterion_is_skipped() {
        let red = stat("Red", 4, 0, 100, 4, 32, 32);
        let blue = stat("Blue", 2, 2, 200, 4, 22, 22);
        // W-L would decide, but it is unchecked, so Total Points does.
        let settings = parse_tiebreaker_rules("-team_wl,total_points");
        let (idx, rule, ordering) = decide_order(&red, &blue, &settings, &[]).unwrap();
        assert_eq!(rule, TieBreakerRule::TotalPoints);
        // Memory Verse (unchecked) is slotted in below Team W-L, so Total Points is rule 3.
        assert_eq!(idx + 1, 3, "Total Points is rule number 3 in this sequence");
        assert_eq!(ordering, std::cmp::Ordering::Greater);
    }

    #[test]
    fn head_to_head_counts_only_shared_games() {
        let games = vec![
            GameRanks { ranks: vec![("Red".into(), 1), ("Blue".into(), 2)] },
            GameRanks { ranks: vec![("Blue".into(), 1), ("Red".into(), 2)] },
            GameRanks { ranks: vec![("Blue".into(), 1), ("Red".into(), 2)] },
            // Red beats a third team — irrelevant to Red vs Blue.
            GameRanks { ranks: vec![("Red".into(), 1), ("Green".into(), 2)] },
        ];
        assert_eq!(head_to_head("Red", "Blue", &games), (1, 2));

        let red = stat("Red", 2, 2, 0, 4, 0, 0);
        let blue = stat("Blue", 2, 2, 0, 4, 0, 0);
        let settings = parse_tiebreaker_rules("head_to_head");
        let (_, rule, ordering) = decide_order(&red, &blue, &settings, &games).unwrap();
        assert_eq!(rule, TieBreakerRule::HeadToHead);
        assert_eq!(ordering, std::cmp::Ordering::Greater, "Blue won the series 2-1");
    }

    #[test]
    fn teams_level_on_every_criterion_are_not_separated() {
        let a = stat("A", 2, 2, 100, 4, 10, 10);
        let b = stat("B", 2, 2, 100, 4, 10, 10);
        assert!(decide_order(&a, &b, &parse_tiebreaker_rules(""), &[]).is_none());
    }
}

#[cfg(test)]
mod question_type_tests {
    use super::*;

    fn quizzer(correct: &[i32], errors: &[i32], bonus_correct: &[i32], bonus_errors: &[i32]) -> CalculatedQuizzer {
        CalculatedQuizzer {
            name: "Ann".to_string(),
            correct_tossups: correct.to_vec(),
            errors_on_tossups: errors.to_vec(),
            correct_bonuses: bonus_correct.to_vec(),
            errors_on_bonuses: bonus_errors.to_vec(),
            fouls_received: vec![],
            question_quizzed_out_on: -1,
            question_errored_out_on: -1,
            question_fouled_out_on: -1,
        }
    }

    fn counts(stats: &[QuestionTypeStat], code: &str) -> (i32, i32, i32, i32) {
        let s = stats.iter().find(|s| s.code == code).unwrap();
        (s.attempted, s.correct, s.bonus_attempted, s.bonus_correct)
    }

    /// Tossups and bonuses land on their question's type; R, Q and V also count toward M; and
    /// overtime questions (past 20) have no type and are left out.
    #[test]
    fn counts_land_on_each_questions_type() {
        // Q1-11 G, Q12-15 A, then Q R V X S, and Q21+ is overtime.
        let types: Vec<char> = "GGGGGGGGGGGAAAAQRVXS".chars().collect();
        let mut tally = QuestionTypeTally::default();
        tally_quizzer(&quizzer(&[1, 2, 16, 21], &[12, 18], &[3, 17], &[13]), &types, &mut tally);
        let stats = tally.to_stats();

        assert_eq![stats.iter().map(|s| s.code.as_str()).collect::<String>(), "GAMRQVXIS"];
        assert_eq![counts(&stats, "G"), (2, 2, 1, 1)];
        assert_eq![counts(&stats, "A"), (1, 0, 1, 0)];
        assert_eq![counts(&stats, "Q"), (1, 1, 0, 0)];
        assert_eq![counts(&stats, "R"), (0, 0, 1, 1)];
        assert_eq![counts(&stats, "V"), (1, 0, 0, 0)];
        assert_eq![counts(&stats, "M"), (2, 1, 1, 1)];
        assert_eq![counts(&stats, "X"), (0, 0, 0, 0)];
    }

    /// A round without question types contributes nothing.
    #[test]
    fn untyped_round_counts_nothing() {
        let mut tally = QuestionTypeTally::default();
        tally_quizzer(&quizzer(&[1, 2], &[3], &[4], &[]), &[], &mut tally);
        assert_eq![tally.to_stats(), empty_question_type_stats()];
    }
}

#[cfg(test)]
mod outs_and_fouls_tests {
    use super::*;

    fn quizzer(errors: &[i32], fouls: &[i32], quizzed_out_on: i32, errored_out_on: i32) -> CalculatedQuizzer {
        CalculatedQuizzer {
            name: "Ann".to_string(),
            correct_tossups: vec![],
            errors_on_tossups: errors.to_vec(),
            correct_bonuses: vec![],
            errors_on_bonuses: vec![],
            fouls_received: fouls.to_vec(),
            question_quizzed_out_on: quizzed_out_on,
            question_errored_out_on: errored_out_on,
            question_fouled_out_on: -1,
        }
    }

    /// A quiz out counts as with or without errors by whether the quizzer erred on a tossup.
    #[test]
    fn quiz_outs_split_on_errors() {
        let clean = quizzer_outs_and_fouls(&quizzer(&[], &[], 8, -1), 3);
        assert_eq![(clean.quiz_outs_without_errors, clean.quiz_outs_with_errors), (1, 0)];
        let erred = quizzer_outs_and_fouls(&quizzer(&[3], &[], 9, -1), 3);
        assert_eq![(erred.quiz_outs_without_errors, erred.quiz_outs_with_errors), (0, 1)];
        let none = quizzer_outs_and_fouls(&quizzer(&[3], &[], -1, -1), 3);
        assert_eq![none, OutsAndFoulsStat::default()];
    }

    /// Fouls are counted one by one, and reaching the game's foul-out count is a foul out.
    #[test]
    fn fouls_and_foul_outs() {
        let two = quizzer_outs_and_fouls(&quizzer(&[], &[2, 5], -1, -1), 3);
        assert_eq![(two.quizzer_fouls, two.foul_outs), (2, 0)];
        let three = quizzer_outs_and_fouls(&quizzer(&[], &[2, 5, 7], -1, -1), 3);
        assert_eq![(three.quizzer_fouls, three.foul_outs), (3, 1)];
    }

    #[test]
    fn error_outs() {
        let out = quizzer_outs_and_fouls(&quizzer(&[1, 2, 3], &[], -1, 3), 3);
        assert_eq![(out.error_outs, out.quiz_outs_with_errors), (1, 0)];
    }

    /// Merging adds every count, which is how a team's row sums its quizzers' rows.
    #[test]
    fn merge_adds_every_count() {
        let mut total = OutsAndFoulsStat { team_fouls: 1, ..Default::default() };
        total.merge(&quizzer_outs_and_fouls(&quizzer(&[], &[4], 10, -1), 3));
        total.merge(&quizzer_outs_and_fouls(&quizzer(&[1, 2, 3], &[], -1, 3), 3));
        assert_eq![
            total,
            OutsAndFoulsStat {
                quiz_outs_without_errors: 1,
                quiz_outs_with_errors: 0,
                error_outs: 1,
                foul_outs: 0,
                quizzer_fouls: 1,
                team_fouls: 1,
                coach_fouls: 0,
            }
        ];
    }
}
