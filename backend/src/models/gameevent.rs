
use std::collections::HashMap;
use chrono::DateTime;
use diesel::{insert_into, prelude::*, upsert::on_constraint};
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::{cache::Cache, database, models::{common::PaginationParams}};
use utoipa::ToSchema;

const DEFAULT_QUESTIONS_PER_GAME: i32 = 20;
const DEFAULT_SUBSTITUTION_SEAT: i32 = 4;
const DEFAULT_INTERIM_SUBSTITUTION_SEAT: i32 = 1000;
const DEFAULT_QUIZ_OUT: i32 = 4;
const DEFAULT_ERROR_OUT: i32 = 3;
const DEFAULT_FOUL_OUT: i32 = 3;
const DEFAULT_2_TEAM_TIMEOUTS: i32 = 3;
const DEFAULT_3_TEAM_TIMEOUTS: i32 = 2;
const DEFAULT_TEAM_ERROR_BEGIN_DEDUCTION_COUNT: i32 = 5;
const DEFAULT_INDIVIDUAL_ERROR_BEGIN_DEDUCTION_COUNT: i32 = 3;
const DEFAULT_POINT_AWARD_FOR_CORRECT_TOSSUP: i32 = 20;
const DEFAULT_POINT_AWARD_FOR_QUIZZING_OUT: i32 = 10;
const DEFAULT_POINT_DEDUCATED_FOR_ERROR_ON_TOSSUP: i32 = 10;
const DEFAULT_POINT_AWARD_FOR_CORRECT_BONUS: i32 = 10;
const DEFAULT_START_ERROR_ZONE_DEDUCTIONS: i32 = 16;
const DEFAULT_COUNT_OF_TEAM_ERRORS_THAT_BEGIN_TEAM_POINT_DEDUCTIONS: i32 = 5;
const DEFAULT_THIRD_FOURTH_AND_FIFTH_PERSON_BONUS_AWARD_AMOUNT: i32 = 10;
const DEFAULT_ATTEMPT_TRY_WHEN_DEDUCATIONS_BEGIN_FOR_OVERRULED_CHALLENGES_BY_A_TEAM: i32 = 2;
const DEFAULT_OVERRULED_CHALLENGE_POINT_DEDUCTION_AMOUNT: i32 = 10;
const DEFAULT_FOUL_COUNT_WHERE_TEAM_POINT_DEDUCTIONS_BEGIN: i32 = 2;
const DEFAULT_TEAM_FOUL_DEDUCTION_AMOUNT: i32 = 10;

#[derive(Clone)]
struct QuizzerForGameEventCalculator {
    name: String,
    correct_tossups: Vec<i32>,  // count = # of TCs, actual number indicates the question it was received on
    errors_on_tossups: Vec<i32>,  // count = # of TEs, actual number indicates the question it was received on
    correct_bonuses: Vec<i32>,  // count = # of BCs, actual number indicates the question it was received on
    errors_on_bonuses: Vec<i32>,  // count = # of BEs, actual number indicates the question it was received on
    fouls_received: Vec<i32>,  // count = # of F-s, actual number indicates the question it was received on
    question_quizzed_out_on: i32,
    question_errored_out_on: i32,
    question_fouled_out_on: i32,
}
impl QuizzerForGameEventCalculator {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            correct_tossups: vec![],
            errors_on_tossups: vec![],
            correct_bonuses: vec![],
            errors_on_bonuses: vec![],
            fouls_received: vec![],
            question_quizzed_out_on: -1,
            question_errored_out_on: -1,
            question_fouled_out_on: -1,
        }
    }
}

#[derive(Clone)]
struct TeamForGameEventCalculator {
    name: String,
    score: i32,
    timeouts_taken: Vec<i32>,  // count = # of TOs, actual number indicates the question it was done on
    overruled_challenges: Vec<i32>,  // i32 = question # where overruled challenge happened
    team_and_coach_fouls_received: Vec<(i32, bool)>,  // count = # of FCs, i32 = question #, bool means "is for coach" (because "FC" = "Fould Coach"): true = coach, false = team
    quizzers: HashMap<i32, QuizzerForGameEventCalculator>, 
    captain: (i32, bool),  // i32 = idx of current seat number; bool means "can stil act as captain", and if false then cocaptain becomes operating captain but (data remains more static)
    cocaptain: (i32, bool),  // i32 = idx of current seat number; bool means "can stil act as cocaptain"; if this becomes false, it should also mean captain is false and new captain and cocaptain are about to be specified in the event stream
    substitutions: Vec<(i32, i32)>,  // Vec<(question_num, seat_num)>
    rank: i32,  // 1st, 2nd or 3rd place team in the Game
}
impl TeamForGameEventCalculator {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            score: 0,
            timeouts_taken: vec![],
            overruled_challenges: vec![],
            team_and_coach_fouls_received: vec![],
            quizzers: HashMap::new(), 
            captain: (-1, false),
            cocaptain: (-1, false),
            substitutions: vec![],
            rank: -1,
        }
    }
    pub fn errors_result_in_team_point_deduction(self) -> bool {
        let mut team_tossup_error_count = 0;
        for (_, quizzer) in self.quizzers {
            team_tossup_error_count += quizzer.errors_on_tossups.iter().count();
        }
        return team_tossup_error_count >= (DEFAULT_COUNT_OF_TEAM_ERRORS_THAT_BEGIN_TEAM_POINT_DEDUCTIONS as usize);
    }
    pub fn quizzers_with_at_least_one_correct_tossup(self) -> i32 {
        let mut team_correct_tossup_count = 0;
        for (_, quizzer) in self.quizzers {
            if quizzer.correct_tossups.iter().count() > 0 {
                team_correct_tossup_count += 1;
            }
        }
        team_correct_tossup_count
    }
    pub fn count_of_all_fouls_received_by_the_team(self) -> i32 {
        let mut foul_counter = self.team_and_coach_fouls_received.iter().count() as i32;
        for (_, quizzer) in self.quizzers {
            foul_counter += quizzer.fouls_received.iter().count() as i32
        }
        foul_counter
    }
}

struct OptionsForGameEventCalculator {
    is_tournament: bool,
    quiz_out: i32,
    error_out: i32,
    foul_out: i32,
    team_error_begin_deduction_count: i32,
    individual_begin_deduction_count: i32,
    point_award_for_correct_tossup: i32,
    point_award_for_quizzing_out: i32,
    point_deduction_for_error_on_tossup: i32,
    point_award_for_correct_bonus: i32,
    start_error_zone_deductions: i32,
    third_fourth_and_fifth_person_bonus_award_amount: i32,
    attempt_try_when_deductions_begin_for_overruled_challenges_by_a_team: i32,
    overruled_challenge_point_deduction_amount: i32,
    foul_count_where_team_point_deductions_begin: i32,
    team_foul_deduction_amount: i32,
    individual_error_begin_deduction_count: i32,
}
impl OptionsForGameEventCalculator {
    pub fn new() -> Self {
        Self {
            is_tournament: true,
            quiz_out: DEFAULT_QUIZ_OUT,
            error_out: DEFAULT_ERROR_OUT,
            foul_out: DEFAULT_FOUL_OUT,
            team_error_begin_deduction_count: DEFAULT_TEAM_ERROR_BEGIN_DEDUCTION_COUNT,
            individual_begin_deduction_count: DEFAULT_TEAM_ERROR_BEGIN_DEDUCTION_COUNT,
            point_award_for_correct_tossup: DEFAULT_POINT_AWARD_FOR_CORRECT_TOSSUP,
            point_award_for_quizzing_out: DEFAULT_POINT_AWARD_FOR_QUIZZING_OUT,
            point_deduction_for_error_on_tossup: DEFAULT_POINT_DEDUCATED_FOR_ERROR_ON_TOSSUP,
            point_award_for_correct_bonus: DEFAULT_POINT_AWARD_FOR_CORRECT_BONUS,
            start_error_zone_deductions: DEFAULT_START_ERROR_ZONE_DEDUCTIONS,
            third_fourth_and_fifth_person_bonus_award_amount: DEFAULT_THIRD_FOURTH_AND_FIFTH_PERSON_BONUS_AWARD_AMOUNT,
            attempt_try_when_deductions_begin_for_overruled_challenges_by_a_team: DEFAULT_ATTEMPT_TRY_WHEN_DEDUCATIONS_BEGIN_FOR_OVERRULED_CHALLENGES_BY_A_TEAM,
            overruled_challenge_point_deduction_amount: DEFAULT_OVERRULED_CHALLENGE_POINT_DEDUCTION_AMOUNT,
            foul_count_where_team_point_deductions_begin: DEFAULT_FOUL_COUNT_WHERE_TEAM_POINT_DEDUCTIONS_BEGIN,
            team_foul_deduction_amount: DEFAULT_TEAM_FOUL_DEDUCTION_AMOUNT,
            individual_error_begin_deduction_count: DEFAULT_INDIVIDUAL_ERROR_BEGIN_DEDUCTION_COUNT
        }
    }
}

// #[derive(Clone)]
struct GameEventCalculator {
    game_id: Uuid,
    current_question: i32,
    teams: HashMap<i32, TeamForGameEventCalculator>,
    options: OptionsForGameEventCalculator,
    // use_cache: bool,
    // cache: Vec<GameEventCalculator>,
    game_events: Vec<GameEvent>,
}
impl GameEventCalculator {
    pub fn new(
        game_id: Uuid, 
        game_events: Vec<GameEvent>, 
    ) -> Self {
        Self {
            game_id,
            teams: HashMap::new(),
            current_question: 1,
            options: OptionsForGameEventCalculator::new(),
            // use_cache: false,
            // cache: vec![],
            game_events,
        }
    }
    // pub fn set_use_cache(self, use_cache: bool) -> Self {
    //     Self {
    //         use_cache,
    //         ..self
    //     }
    // }
    pub fn calculate_current_game_scores_and_counts(self) -> Result<Self, Vec<String>> {
        let mut errors: Vec<String> = vec![];
        
        let mut mut_self = GameEventCalculator::new(self.game_id, self.game_events.clone());

        // if self.use_cache {
        //     let clone_of_mut_self = mut_self.clone();
        //     mut_self.cache.push(clone_of_mut_self);
        // }
        let mut game_events = self.game_events.clone();
        game_events.sort();  // << VERY IMPORTANT
        for game_event in game_events.iter() {
            if mut_self.teams.contains_key(&0) && mut_self.teams.contains_key(&1) {
                println!["TOP of Calc: Question: {}, Event Code: {}, Left Team: {}, Right Team: {}", &game_event.question, &game_event.event, mut_self.teams[&0].score, mut_self.teams[&1].score];
            }
            else {
                println!["TOP of Calc: Question = {}, EventNum = {}, Event = {}", game_event.question, game_event.eventnum, game_event.event];
            }

            mut_self = Self::calc_new_score_from_game_state(
                mut_self,
                game_event,
                &self.options,
                &mut errors,
            )?;
        }

        if errors.len() > 0 {
            return Err(errors);
        }

        Ok(mut_self)
    }
    /// Applies one game event to the running calculation and returns the advanced state.
    ///
    /// Extracted from `calculate_current_game_scores_and_counts`, whose loop calls this once
    /// per event. Three details carry over from when it was inline:
    ///
    /// - `errors` accumulates across events. Some events (QT, OP) record a problem and let the
    ///   calculation carry on — the caller only fails at the end — while the ones that cannot
    ///   continue return `Err` carrying everything gathered so far.
    /// - `options_at_entry` is the *caller's* options, which is what the error-zone check read
    ///   inline. It is deliberately not `mut_self.options`: IP and OP events mutate the running
    ///   options, so reading those here could change the result.
    /// - Where the inline code used `continue` to skip the rest of an event's handling, this
    ///   returns the state early. The loop body had nothing after the match, so that is the
    ///   same thing.
    pub fn calc_new_score_from_game_state(
        mut mut_self: Self,
        game_event: &GameEvent,
        options_at_entry: &OptionsForGameEventCalculator,
        errors: &mut Vec<String>,
    ) -> Result<Self, Vec<String>> {
        // Guard: any in-game event that operates on an existing team/quizzer must find
        // them first. If not, exit immediately rather than panicking on an unwrap deeper
        // in the handler. (RM/QT/IP/OP/TN/NJ/A+/A-/DE create or don't touch teams.)
        let event_code = game_event.event.as_str();
        let requires_team = matches!(
            event_code,
            "TC" | "TE" | "BC" | "BE" | "QO" | "EO" | "C-" | "FC" | "F-" | "SB" | "TO" | "SC" | "SS" | "QN"
        );
        if requires_team && !mut_self.teams.contains_key(&game_event.team) {
            errors.push(format!("Team not found: {}", game_event.team));
            return Err(std::mem::take(errors));
        }
        let requires_quizzer = matches!(event_code, "TC" | "TE" | "BC" | "BE" | "QO" | "EO" | "F-");
        if requires_quizzer {
            let quizzer_exists = mut_self
                .teams
                .get(&game_event.team)
                .map_or(false, |team| team.quizzers.contains_key(&game_event.quizzer));
            if !quizzer_exists {
                errors.push(format!("Quizzer not found: seat {} on team {}", game_event.quizzer, game_event.team));
                return Err(std::mem::take(errors));
            }
        }

        match string_to_gameeventcode(game_event.event.as_str()) {
            GameEventCode::RM => {
                // no impact
                mut_self.current_question = game_event.question;
            },
            GameEventCode::QT => {
                // could check: if 'N' then good, else throw error
                if game_event.name != "N" {
                    errors.push(format!["Game type is something other than 'Nazarene' but rules are implemented only for 'Nazarene'. Specified organization: {}", game_event.name]);
                }
            },
            GameEventCode::IP => {
                if game_event.name == "Practice".to_string() {
                    mut_self.options.is_tournament = false;
                }
            },
            GameEventCode::OP => {
                match game_event.name.as_str() {
                    "QuizOut" => {
                        mut_self.options.quiz_out = game_event.quizzer;
                    },
                    "ErrorOut" => {
                        mut_self.options.error_out = game_event.quizzer;
                    },
                    "FoulOut" => {
                        mut_self.options.foul_out = game_event.quizzer;
                    },
                    "QuizzerDeduct" => {
                        mut_self.options.team_error_begin_deduction_count = game_event.quizzer;
                    },
                    "TeamDeduct" => {
                        mut_self.options.individual_begin_deduction_count = game_event.quizzer;
                    },
                    "" => {
                        errors.push("GameEvent code/type 'OP' was specified but option name provided was blank.".to_string());
                    }
                    _ => {

                        errors.push(format!["GameEvent code/type 'OP' was specified but option name provided was not found among acceptable options. Option specified: '{}'", game_event.name]);
                    }
                }
            },
            GameEventCode::TN => {
                let mut new_team = TeamForGameEventCalculator::new();
                new_team.name = game_event.name.to_string();
                mut_self.teams.insert(game_event.team, new_team);
            },
            GameEventCode::QN => {
                // QN is used (1) during round initialization when the quizzer doesn't exist and (2) when 
                // the quizzer is being substituted.
                // QN game_event is going to need to check the team for current quizzer names; 
                // if name is not found, then create the quizzer and assign them to the seat; 
                // if they already exist, copy them to the seat specified and delete the quizzer from 
                // their previous seat

                let quizzer_idx: Option<i32> = mut_self
                    .teams
                    .get(&game_event.team)
                    .and_then(|team| {
                        team.quizzers
                            .iter()
                            .find(|(_, quizzer)| quizzer.name == game_event.name)
                            .map(|(idx, _)| idx.clone())
                    });

                match quizzer_idx {
                    None => {
                        let new_quizzer = QuizzerForGameEventCalculator::new(&game_event.name);
                        mut_self
                            .teams.get_mut(&game_event.team).unwrap()
                            .quizzers.insert(game_event.quizzer, new_quizzer);
                    }
                    Some(idx) => {
                        let team = mut_self
                            .teams.get_mut(&game_event.team).unwrap();

                        let quizzer = match team.quizzers.remove(&idx) {
                            None => {
                                errors.push(format!(
                                    "Tried to change quizzer's seat location for substitution but quizzer \
                                    wasn't found at that seat. Team: '{}', Quizzer seat: '{}', Question: '{}'",
                                    game_event.team, game_event.quizzer, game_event.question
                                ));
                                return Err(std::mem::take(errors));
                            }
                            Some(q) => q,
                        };

                        team.quizzers.insert(game_event.quizzer, quizzer);
                    }
                }
            },
            GameEventCode::SC => {
                mut_self
                    .teams.get_mut(&game_event.team).unwrap()
                    .captain = (game_event.quizzer, true);
            },
            GameEventCode::SS => {
                mut_self
                    .teams.get_mut(&game_event.team).unwrap()
                    .cocaptain = (game_event.quizzer, true);
            },
            GameEventCode::TC => {
                let original_quizzers_with_at_least_one_correct_tossup = match mut_self.teams.get(&game_event.team) {
                    Some(team) => team.clone().quizzers_with_at_least_one_correct_tossup(),
                    None => {
                        errors.push(format!("Team not found: {}", game_event.team));
                        return Err(std::mem::take(errors));
                    }
                };

                if mut_self.current_question > DEFAULT_QUESTIONS_PER_GAME {

                    let tie_exists = {
                        let mut scores: Vec<i32> = mut_self.teams.values().map(|t: &TeamForGameEventCalculator| t.score).collect();
                        scores.sort();
                        scores.windows(2).any(|w| w[0] == w[1])
                    };
                    if !tie_exists {
                        errors.push("In the absence of a tied score after regulation questions, a TC event was found which is invalid.".to_string());
                        return Err(std::mem::take(errors));
                    }

                    let award = mut_self.options.point_award_for_correct_tossup;
                    if let Some(team) = mut_self.teams.get_mut(&game_event.team) {
                        team.score += award;
                    }
                    mut_self = mut_self.update_team_rankings_using_competitive_ranking();
                    if let Some(team) = mut_self.teams.get_mut(&game_event.team) {
                        team.score -= award;  // remove so that rankings show not tied but score still shows as tied
                    }
                    
                    mut_self
                        .teams.get_mut(&game_event.team).unwrap()
                        .quizzers.get_mut(&game_event.quizzer).unwrap()
                        .correct_tossups
                        .push(game_event.question);
                    mut_self.current_question = game_event.question + 1;
                    return Ok(mut_self);
                }

                // for quizzer:
                mut_self
                    .teams.get_mut(&game_event.team).unwrap()
                    .quizzers.get_mut(&game_event.quizzer).unwrap()
                    .correct_tossups
                    .push(game_event.question);
                // QO will be handled by QO game_event; don't do anything for it here.

                // for team:
                let award = mut_self.options.point_award_for_correct_tossup;
                if let Some(team) = mut_self.teams.get_mut(&game_event.team) {
                    team.score += award;
                }
                // 3rd, 4th, and 5th person bonuses:
                let new_quizzers_with_at_least_one_correct_tossup = match mut_self.teams.get(&game_event.team) {
                    Some(team) => team.clone().quizzers_with_at_least_one_correct_tossup(),
                    None => {
                        errors.push(format!("Team not found: {}", game_event.team));
                        return Err(std::mem::take(errors));
                    }
                };
                let third_fourth_fifth_person_bonus_award_amount = mut_self.options.third_fourth_and_fifth_person_bonus_award_amount;
                let is_third_person_bonus = original_quizzers_with_at_least_one_correct_tossup == 2 && new_quizzers_with_at_least_one_correct_tossup == 3;
                let is_fourth_person_bonus = original_quizzers_with_at_least_one_correct_tossup == 3 && new_quizzers_with_at_least_one_correct_tossup == 4;
                let is_fifth_person_bonus = original_quizzers_with_at_least_one_correct_tossup == 4 && new_quizzers_with_at_least_one_correct_tossup == 5;
                let is_third_fourth_or_fifth_person_bonus = is_third_person_bonus || is_fourth_person_bonus || is_fifth_person_bonus;
                if is_third_fourth_or_fifth_person_bonus {
                    if let Some(team) = mut_self.teams.get_mut(&game_event.team) {
                        team.score += third_fourth_fifth_person_bonus_award_amount;
                    }
                }

                // for game:
                mut_self.current_question = game_event.question + 1;
                // if self.use_cache {
                //     let clone_of_mut_self = mut_self.clone();
                //     mut_self.cache.push(clone_of_mut_self);
                // }
            },
            GameEventCode::TE => {

                if mut_self.current_question > DEFAULT_QUESTIONS_PER_GAME {

                    let tie_exists = {
                        let mut scores: Vec<i32> = mut_self.teams.values().map(|t: &TeamForGameEventCalculator| t.score).collect();
                        scores.sort();
                        scores.windows(2).any(|w| w[0] == w[1])
                    };
                    if !tie_exists {
                        errors.push("In the absence of a tied score after regulation questions, a TE event was found which is invalid.".to_string());
                        return Err(std::mem::take(errors));
                    }

                    let award = mut_self.options.point_deduction_for_error_on_tossup;
                    if let Some(team) = mut_self.teams.get_mut(&game_event.team) {
                        team.score -= award;
                    }
                    mut_self = mut_self.update_team_rankings_using_competitive_ranking();
                    if let Some(team) = mut_self.teams.get_mut(&game_event.team) {
                        team.score += award;  // remove so that rankings show not tied but score still shows as tied
                    }

                    mut_self
                        .teams.get_mut(&game_event.team).unwrap()
                        .quizzers.get_mut(&game_event.quizzer).unwrap()
                        .errors_on_tossups
                        .push(game_event.question);
                    mut_self.current_question = game_event.question + 1;
                    return Ok(mut_self);
                }

                // for quizzer:
                mut_self
                    .teams.get_mut(&game_event.team).unwrap()
                    .quizzers.get_mut(&game_event.quizzer).unwrap()
                    .errors_on_tossups
                    .push(game_event.question);
                let quizzer_error_count = mut_self
                    .teams.get_mut(&game_event.team).unwrap()
                    .quizzers.get_mut(&game_event.quizzer).unwrap()
                    .errors_on_tossups
                    .iter().count();
                // EO will be handled by EO game_event; don't do anything for it here.
                
                // for team:
                let team_errors_cause_deduction = match mut_self.teams.get(&game_event.team) {
                    Some(team) => team.clone().errors_result_in_team_point_deduction(),
                    None => {
                        errors.push(format!("Team not found: {}", game_event.team));
                        return Err(std::mem::take(errors));
                    }
                };
                if game_event.question >= options_at_entry.start_error_zone_deductions
                    || team_errors_cause_deduction
                    || quizzer_error_count >= mut_self.options.individual_error_begin_deduction_count as usize {
                    let deduction = mut_self.options.point_deduction_for_error_on_tossup;
                    if let Some(team) = mut_self.teams.get_mut(&game_event.team) {
                        team.score -= deduction;
                    }
                }

                // for game:
                mut_self.current_question = game_event.question;

                // if self.use_cache {
                //     let clone_of_mut_self = mut_self.clone();
                //     mut_self.cache.push(clone_of_mut_self);
                // }
            },
            GameEventCode::NJ => {
                // for quizzer:
                // for team:
                // for game:
                mut_self.current_question = game_event.question + 1;
                // if self.use_cache {
                //     let clone_of_mut_self = mut_self.clone();
                //     mut_self.cache.push(clone_of_mut_self);
                // }
            },
            GameEventCode::BC => {
                // for quizzer:
                mut_self
                    .teams.get_mut(&game_event.team).unwrap()
                    .quizzers.get_mut(&game_event.quizzer).unwrap()
                    .correct_bonuses
                    .push(game_event.question);

                // for team:
                let award = mut_self.options.point_award_for_correct_bonus;
                if let Some(team) = mut_self.teams.get_mut(&game_event.team) {
                    team.score += award;
                }

                // for game:
                mut_self.current_question = game_event.question + 1;
                
                // if self.use_cache {
                //     let clone_of_mut_self = mut_self.clone();
                //     mut_self.cache.push(clone_of_mut_self);
                // }
            },
            GameEventCode::BE => {
                // for quizzer:
                mut_self
                    .teams.get_mut(&game_event.team).unwrap()
                    .quizzers.get_mut(&game_event.quizzer).unwrap()
                    .errors_on_bonuses
                    .push(game_event.question);

                // for team: no change
                
                // for game:
                mut_self.current_question = game_event.question + 1;
                
                // if self.use_cache {
                //     let clone_of_mut_self = mut_self.clone();
                //     mut_self.cache.push(clone_of_mut_self);
                // }
            },
            GameEventCode::QO => {
                // for quizzer:
                mut_self
                    .teams.get_mut(&game_event.team).unwrap()
                    .quizzers.get_mut(&game_event.quizzer).unwrap()
                    .question_quizzed_out_on = game_event.question;
                // award points if no errors while quizzing-out (QO w/o):
                let award = mut_self.options.point_award_for_quizzing_out;
                let is_quiz_out_without_error = match mut_self.teams.get(&game_event.team) {
                    Some(team) => team.quizzers[&game_event.quizzer].errors_on_tossups.iter().count() == 0,
                    None => {
                        errors.push(format!("Team not found: {}", game_event.team));
                        return Err(std::mem::take(errors));
                    }
                };
                println!["is_quiz_out_without_error: {}", is_quiz_out_without_error];
                if is_quiz_out_without_error {
                    if let Some(team) = mut_self.teams.get_mut(&game_event.team) {
                        team.score += award;
                    }
                }
                
                // for team:
                // substitutions are handled by SB events; do not handle SBs here
                let captain = mut_self
                    .teams.get_mut(&game_event.team).unwrap()
                    .captain;
                let cocaptain = mut_self
                    .teams.get_mut(&game_event.team).unwrap()
                    .cocaptain;
                if captain.0 == game_event.quizzer {
                    mut_self
                        .teams.get_mut(&game_event.team).unwrap()
                        .captain = (game_event.quizzer, false);
                }
                else if cocaptain.0 == game_event.quizzer {
                    mut_self
                        .teams.get_mut(&game_event.team).unwrap()
                        .cocaptain = (game_event.quizzer, false);
                }

                // for game:
                mut_self.current_question = game_event.question + 1;
                
                // if self.use_cache {
                //     let clone_of_mut_self = mut_self.clone();
                //     mut_self.cache.push(clone_of_mut_self);
                // }
            },
            GameEventCode::EO => {
                // for quizzer:
                mut_self
                    .teams.get_mut(&game_event.team).unwrap()
                    .quizzers.get_mut(&game_event.quizzer).unwrap()
                    .question_errored_out_on = game_event.question;

                // for team:
                // Error-outs do not accrue any deductions beyond the regular individual error deduction.
                // Individual error point deduction starts on a certain number of errors received by the quizzer.
                // It is normally on error number 3 that the deducation is received.

                let captain_seat_idx = mut_self
                    .teams.get_mut(&game_event.team).unwrap()
                    .captain.0;
                let cocaptain_seat_idx = mut_self
                    .teams.get_mut(&game_event.team).unwrap()
                    .cocaptain.0;
                if captain_seat_idx == game_event.quizzer {
                    mut_self
                        .teams.get_mut(&game_event.team).unwrap()
                        .captain = (game_event.quizzer, false);
                }
                else if cocaptain_seat_idx == game_event.quizzer {
                    mut_self
                        .teams.get_mut(&game_event.team).unwrap()
                        .cocaptain = (game_event.quizzer, false);
                }
                
                // for game:
                mut_self.current_question = game_event.question + 1;
                
                // if self.use_cache {
                //     let clone_of_mut_self = mut_self.clone();
                //     mut_self.cache.push(clone_of_mut_self);
                // }
            },
            GameEventCode::Cminus => {
                // for quizzer:

                // for team:
                // not considered a individual or team error, however on 
                // 2nd failed (overruled) challenge point deductions begin.
                mut_self
                    .teams.get_mut(&game_event.team).unwrap()
                    .overruled_challenges.push(game_event.question);
                let overruled_challenges_count = mut_self
                    .teams.get_mut(&game_event.team).unwrap()
                    .overruled_challenges.iter().count();
                if overruled_challenges_count >= (mut_self.options.attempt_try_when_deductions_begin_for_overruled_challenges_by_a_team as usize) {
                    mut_self
                        .teams.get_mut(&game_event.team).unwrap()
                        .score -= mut_self.options.overruled_challenge_point_deduction_amount;
                }

                // for game:
                mut_self.current_question = game_event.question + 1;
                
                // if self.use_cache {
                //     let clone_of_mut_self = mut_self.clone();
                //     mut_self.cache.push(clone_of_mut_self);
                // }
            },
            GameEventCode::Aplus => {
                // need to mark invalid game events with type 'DE' so that they are no longer included in calculations
                // for calculations makring past events as 'DE' would have already been done

                // for quizzer:
                // for team:
                // for game:
                mut_self.current_question = game_event.question;
                
                // if self.use_cache {
                //     let clone_of_mut_self = mut_self.clone();
                //     mut_self.cache.push(clone_of_mut_self);
                // }
            },
            GameEventCode::Aminus => {
                // nothing happens; no change

                // for quizzer:
                // for team:
                // for game:
                mut_self.current_question = game_event.question + 1;
                
                // if self.use_cache {
                //     let clone_of_mut_self = mut_self.clone();
                //     mut_self.cache.push(clone_of_mut_self);
                // }
            },
            GameEventCode::FC => {
                // for quizzer:

                // for team:
                let is_for_coach = if game_event.quizzer == 5 { true } else { false };  // false = foul on team
                mut_self
                    .teams.get_mut(&game_event.team).unwrap()
                    .team_and_coach_fouls_received.push((game_event.question, is_for_coach));
                
                if let Some(team) = mut_self.teams.get_mut(&game_event.team) {
                    if team.clone().count_of_all_fouls_received_by_the_team() >= mut_self.options.foul_count_where_team_point_deductions_begin {
                        team.score -= mut_self.options.team_foul_deduction_amount;
                    }
                }

                // for game:
                
                // if self.use_cache {
                //     let clone_of_mut_self = mut_self.clone();
                //     mut_self.cache.push(clone_of_mut_self);
                // }
            },
            GameEventCode::Fminus => {
                // for quizzer:
                mut_self
                    .teams.get_mut(&game_event.team).unwrap()
                    .quizzers.get_mut(&game_event.quizzer).unwrap()
                    .fouls_received.push(game_event.question);
                let quizzer_has_fouled_out = mut_self
                    .teams.get_mut(&game_event.team).unwrap()
                    .quizzers.get_mut(&game_event.quizzer).unwrap()
                    .fouls_received.iter().count() >= (DEFAULT_FOUL_OUT as usize);
                if quizzer_has_fouled_out {
                    let is_captain = mut_self
                        .teams.get_mut(&game_event.team).unwrap()
                        .captain.0 == game_event.quizzer;
                    if is_captain {
                        mut_self
                            .teams.get_mut(&game_event.team).unwrap()
                            .captain = (game_event.quizzer, false);
                    }
                    else {
                        let is_cocaptain = mut_self
                            .teams.get_mut(&game_event.team).unwrap()
                            .cocaptain.0 == game_event.quizzer;
                        if is_cocaptain {
                            mut_self
                                .teams.get_mut(&game_event.team).unwrap()
                                .cocaptain = (game_event.quizzer, false);
                        }
                    }
                }

                // for team:
                if let Some(team) = mut_self.teams.get_mut(&game_event.team) {
                    if team.clone().count_of_all_fouls_received_by_the_team() >= mut_self.options.foul_count_where_team_point_deductions_begin {
                        team.score -= mut_self.options.team_foul_deduction_amount;
                    }
                }

                // for game:
                
                // if self.use_cache {
                //     let clone_of_mut_self = mut_self.clone();
                //     mut_self.cache.push(clone_of_mut_self);
                // }
            },
            GameEventCode::SB => {
                // for quizzer:
                let quizzer_option = mut_self
                    .teams.get_mut(&game_event.team).unwrap()
                    .quizzers.remove(&game_event.quizzer);
                match quizzer_option {
                    Some(quizzer) => {
                        mut_self
                            .teams.get_mut(&game_event.team).unwrap()
                            .quizzers.insert(DEFAULT_INTERIM_SUBSTITUTION_SEAT, quizzer.clone());
                    },
                    None => {
                        errors.push(format!["Tried to change quizzer's seat location for substitution but quizzer wasn't found at that seat. Team: '{}', Quizzer seat: '{}', Question: '{}'", game_event.team, game_event.quizzer, game_event.question]);
                        return Err(std::mem::take(errors));
                    },
                }

                mut_self
                    .teams.get_mut(&game_event.team).unwrap()
                    .substitutions.push((game_event.question, game_event.quizzer));

                // for team:

                // for game:

                // if self.use_cache {
                //     let clone_of_mut_self = mut_self.clone();
                //     mut_self.cache.push(clone_of_mut_self);
                // }
            },
            GameEventCode::TO => {
                // for quizzer:

                // for team:
                mut_self
                    .teams.get_mut(&game_event.team).unwrap()
                    .timeouts_taken.push(game_event.question);

                // for game:
                
                // if self.use_cache {
                //     let clone_of_mut_self = mut_self.clone();
                //     mut_self.cache.push(clone_of_mut_self);
                // }
            },
            GameEventCode::DE => {
                // do nothing/ignore these events (*they are now purely historical)
            },
        }

        Ok(mut_self)
    }
    fn update_team_rankings_using_competitive_ranking(mut self) -> Self {
        let scores: Vec<i32> = self.teams.iter().map(| (_, t) | t.score).collect();
        for (_, team) in self.teams.iter_mut() {
            team.rank = (scores.iter().filter(|&&s| s > team.score).count() + 1) as i32;
        }
        self
    }
    // Keeping in case useful in the future (because it was written and then discovered to be not needed presently)
    // fn update_team_rankings_using_dense_ranking(mut self) -> Self {
    //     // alternative to competitive ranking = Dense ranking:
    //     let mut teams: Vec<(i32, TeamForGameEventCalculator)> = self.teams
    //         .iter()
    //         .map(|(&k, v)| (k, v.clone()))
    //         .collect();
    //     teams.sort_by(|a, b| b.1.score.cmp(&a.1.score));
            
    //     let mut current_rank = 1;
    //     for idx in 0..teams.len() {
    //         if idx > 0 && teams[idx].1.score != teams[idx - 1].1.score {
    //             current_rank += 1;
    //         }
    //         teams[idx].1.rank = current_rank;
    //     }
    //     for (idx, team) in &teams {
    //         if let Some(original) = self.teams.get_mut(&idx) {
    //             original.rank = team.rank;
    //         }
    //     }
    //     self
    // }
}

// ============================================================================
// Cacheable calculated-game state
//
// `GameEventCalculator` and its `Team*`/`Quizzer*`/`Options*` helpers are private to this
// module and carry `game_events` (the whole input stream) alongside the state they derive
// from it. The `Calculated*` mirrors below are the public, serializable *derived* state with
// the input stream dropped, so one of these can be cached per game and then projected into
// the narrower `TeamGameResult` / `QuizzerGameResult` views without re-reading events or
// re-running the calculator.
//
// Conversions are inherent `to_calculated()` methods on the private types rather than `From`
// impls, because `impl From<&PrivateType> for PublicType` would leak a private type into a
// public interface.
// ============================================================================

/// Cache-format version for `CalculatedGame`.
///
/// Bump this whenever the scoring logic changes, any `DEFAULT_*` constant at the top of this
/// file changes, or any `Calculated*` struct below changes shape. It is part of the cache key
/// (see [`CalculatedGame::cache_key`]), so bumping it orphans previously cached entries
/// instead of deserializing stale state against new rules.
pub const CALC_VERSION: u32 = 1;

/// One quizzer's fully-derived state at the end of a game.
///
/// The `Vec<i32>` fields hold the *question numbers* each thing happened on, not just counts,
/// which is what a broadcast overlay needs ("one tossup from quizzing out", "errored out on
/// Q14"). `QuizzerGameResult` throws this detail away; this type keeps it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculatedQuizzer {
    pub name: String,
    pub correct_tossups: Vec<i32>,
    pub errors_on_tossups: Vec<i32>,
    pub correct_bonuses: Vec<i32>,
    pub errors_on_bonuses: Vec<i32>,
    pub fouls_received: Vec<i32>,
    /// -1 when it never happened.
    pub question_quizzed_out_on: i32,
    /// -1 when it never happened.
    pub question_errored_out_on: i32,
    /// -1 when it never happened.
    pub question_fouled_out_on: i32,
}

impl QuizzerForGameEventCalculator {
    fn to_calculated(&self) -> CalculatedQuizzer {
        CalculatedQuizzer {
            name: self.name.clone(),
            correct_tossups: self.correct_tossups.clone(),
            errors_on_tossups: self.errors_on_tossups.clone(),
            correct_bonuses: self.correct_bonuses.clone(),
            errors_on_bonuses: self.errors_on_bonuses.clone(),
            fouls_received: self.fouls_received.clone(),
            question_quizzed_out_on: self.question_quizzed_out_on,
            question_errored_out_on: self.question_errored_out_on,
            question_fouled_out_on: self.question_fouled_out_on,
        }
    }
}

/// One team's fully-derived state at the end of a game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculatedTeam {
    pub name: String,
    pub score: i32,
    pub timeouts_taken: Vec<i32>,
    pub overruled_challenges: Vec<i32>,
    /// `(question_num, is_for_coach)` -- true = coach foul, false = team foul.
    pub team_and_coach_fouls_received: Vec<(i32, bool)>,
    /// Keyed by seat index. Note that `serde_json` renders integer map keys as strings
    /// (`{"0": {...}}`); it round-trips back to `i32` correctly.
    pub quizzers: HashMap<i32, CalculatedQuizzer>,
    /// `(seat_idx, can_still_act_as_captain)`.
    pub captain: (i32, bool),
    /// `(seat_idx, can_still_act_as_cocaptain)`.
    pub cocaptain: (i32, bool),
    /// `(question_num, seat_num)` pairs.
    pub substitutions: Vec<(i32, i32)>,
    /// Placement within the game. -1 when the calculator never assigned one (see
    /// [`CalculatedGame::overtime_ranks_assigned`]).
    pub rank: i32,
}

impl TeamForGameEventCalculator {
    fn to_calculated(&self) -> CalculatedTeam {
        CalculatedTeam {
            name: self.name.clone(),
            score: self.score,
            timeouts_taken: self.timeouts_taken.clone(),
            overruled_challenges: self.overruled_challenges.clone(),
            team_and_coach_fouls_received: self.team_and_coach_fouls_received.clone(),
            quizzers: self
                .quizzers
                .iter()
                .map(|(seat, quizzer)| (*seat, quizzer.to_calculated()))
                .collect(),
            captain: self.captain,
            cocaptain: self.cocaptain,
            substitutions: self.substitutions.clone(),
            rank: self.rank,
        }
    }
}

/// The scoring options actually in force for a game.
///
/// These are *not* constants: they start from the `DEFAULT_*` values and are then overridden
/// by `IP` and `OP` events in the stream (see `calc_new_score_from_game_state`), so they are
/// per-game derived state and must be cached alongside the scores. `CALC_VERSION` guards the
/// case where a `DEFAULT_*` fallback changes while cached entries still hold the old value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculatedOptions {
    pub is_tournament: bool,
    pub quiz_out: i32,
    pub error_out: i32,
    pub foul_out: i32,
    pub team_error_begin_deduction_count: i32,
    pub individual_begin_deduction_count: i32,
    pub point_award_for_correct_tossup: i32,
    pub point_award_for_quizzing_out: i32,
    pub point_deduction_for_error_on_tossup: i32,
    pub point_award_for_correct_bonus: i32,
    pub start_error_zone_deductions: i32,
    pub third_fourth_and_fifth_person_bonus_award_amount: i32,
    pub attempt_try_when_deductions_begin_for_overruled_challenges_by_a_team: i32,
    pub overruled_challenge_point_deduction_amount: i32,
    pub foul_count_where_team_point_deductions_begin: i32,
    pub team_foul_deduction_amount: i32,
    pub individual_error_begin_deduction_count: i32,
}

impl OptionsForGameEventCalculator {
    fn to_calculated(&self) -> CalculatedOptions {
        CalculatedOptions {
            is_tournament: self.is_tournament,
            quiz_out: self.quiz_out,
            error_out: self.error_out,
            foul_out: self.foul_out,
            team_error_begin_deduction_count: self.team_error_begin_deduction_count,
            individual_begin_deduction_count: self.individual_begin_deduction_count,
            point_award_for_correct_tossup: self.point_award_for_correct_tossup,
            point_award_for_quizzing_out: self.point_award_for_quizzing_out,
            point_deduction_for_error_on_tossup: self.point_deduction_for_error_on_tossup,
            point_award_for_correct_bonus: self.point_award_for_correct_bonus,
            start_error_zone_deductions: self.start_error_zone_deductions,
            third_fourth_and_fifth_person_bonus_award_amount: self
                .third_fourth_and_fifth_person_bonus_award_amount,
            attempt_try_when_deductions_begin_for_overruled_challenges_by_a_team: self
                .attempt_try_when_deductions_begin_for_overruled_challenges_by_a_team,
            overruled_challenge_point_deduction_amount: self
                .overruled_challenge_point_deduction_amount,
            foul_count_where_team_point_deductions_begin: self
                .foul_count_where_team_point_deductions_begin,
            team_foul_deduction_amount: self.team_foul_deduction_amount,
            individual_error_begin_deduction_count: self.individual_error_begin_deduction_count,
        }
    }
}

/// A game's complete derived state after the calculator has replayed its event stream.
///
/// This is the unit of caching: it is the output of the one expensive operation (one DB read
/// of the game's events + one full replay), it is invalidated by exactly one write path
/// (`create_update_game_event`), and every consumer below is a cheap in-memory projection of
/// it. It deliberately does *not* carry `game_events` -- that is the input, it already lives
/// in Postgres, and it is far larger than the state derived from it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculatedGame {
    pub game_id: Uuid,
    pub current_question: i32,
    /// Keyed by team index within the game (0/1/2). See the note on
    /// [`CalculatedTeam::quizzers`] about integer map keys in JSON.
    pub teams: HashMap<i32, CalculatedTeam>,
    pub options: CalculatedOptions,
}

impl CalculatedGame {
    /// Cache key for one game's calculated state, versioned by [`CALC_VERSION`].
    pub fn cache_key(game_id: Uuid) -> String {
        format!("qv:v1:game:{}:calc:{}", game_id, CALC_VERSION)
    }

    /// True when the calculator assigned real placement ranks itself.
    ///
    /// During overtime (questions past regulation) it decides a winner and assigns ranks even
    /// though the displayed score stays tied. When it did not, placement has to be derived
    /// from the scores instead -- see [`team_results_from_calculated`].
    pub fn overtime_ranks_assigned(&self) -> bool {
        self.teams.values().any(|team| team.rank >= 1)
    }
}

impl GameEventCalculator {
    /// Projects the finished calculator into its serializable derived state, dropping the
    /// input event stream.
    fn to_calculated(&self) -> CalculatedGame {
        CalculatedGame {
            game_id: self.game_id,
            current_question: self.current_question,
            teams: self
                .teams
                .iter()
                .map(|(idx, team)| (*idx, team.to_calculated()))
                .collect(),
            options: self.options.to_calculated(),
        }
    }
}

/// Runs the score calculator over a single game's events and returns the full derived state.
///
/// This is the function worth putting a cache in front of: everything below is a projection
/// of its result. Returns Err (with the calculator's messages) if the event stream is invalid
/// / cannot be scored.
pub fn calculate_game(
    game_id: Uuid,
    game_events: Vec<GameEvent>,
) -> Result<CalculatedGame, Vec<String>> {
    let calculator = GameEventCalculator::new(game_id, game_events);
    let calculated = calculator.calculate_current_game_scores_and_counts()?;
    Ok(calculated.to_calculated())
}

/// How long a cached [`CalculatedGame`] lives.
///
/// This is a backstop, not the correctness mechanism -- the write path invalidates a game's
/// entry as soon as a new event for it lands (see `services::gameevent::write`). The TTL
/// covers the cases invalidation cannot: an invalidation lost to a Valkey blip, and reclaiming
/// memory for games nobody is looking at any more.
///
/// Six hours is chosen so that a tournament day stays warm end to end while a lost
/// invalidation still self-heals the same day rather than persisting indefinitely.
pub const CALCULATED_GAME_TTL: std::time::Duration = std::time::Duration::from_secs(6 * 60 * 60);

/// [`calculate_game`], but served from the Valkey cache when possible.
///
/// On a hit this does **no** database work at all -- it skips the game's event read as well as
/// the replay, which is the whole point: a stats group with 50 games goes from 51 queries plus
/// 50 replays down to one query.
///
/// Return values follow the three real outcomes:
///   * `Ok(Some(..))` -- calculated state, from cache or freshly computed
///   * `Ok(None)` -- the event stream cannot be scored, so the caller should skip this game
///     (the same signal the uncached path gives via `Err`)
///   * `Err(..)` -- a genuine database failure, which should propagate
///
/// A cache failure is never an error here; it degrades to the uncached path.
pub async fn calculate_game_cached(
    cache: &Cache,
    db: &mut database::Connection,
    game_id: Uuid,
) -> QueryResult<Option<CalculatedGame>> {
    let key = CalculatedGame::cache_key(game_id);

    if let Some(hit) = cache.get_json::<CalculatedGame>(&key).await {
        return Ok(Some(hit));
    }

    let pagination = PaginationParams {
        page: 0,
        page_size: PaginationParams::MAX_PAGE_SIZE as i64,
    };
    let events = read_all_gameevents_of_game(db, game_id, &pagination)?;

    match calculate_game(game_id, events) {
        Ok(calculated) => {
            cache.set_json(&key, &calculated, CALCULATED_GAME_TTL).await;
            Ok(Some(calculated))
        }
        Err(messages) => {
            // An unscoreable stream is normal for a game still being set up or one with
            // corrupt events; the stats readers skip it. Not cached: it may well become
            // scoreable as more events arrive.
            log::debug!(
                "Game {game_id} could not be scored, skipping it in stats: {:?}",
                messages
            );
            Ok(None)
        }
    }
}

/// A single team's outcome for one game, extracted from the score calculator.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamGameResult {
    pub team: i32,      // team index within the game (0/1/2)
    pub name: String,   // team name (from the TN event)
    pub score: i32,     // final score
    pub rank: i32,      // competitive placement rank (1 = 1st place)
}

/// Projects each team's final score and placement rank out of already-calculated state.
///
/// Pure -- no DB access, no replay -- so it is safe to call per request against a cached
/// [`CalculatedGame`].
pub fn team_results_from_calculated(calculated: &CalculatedGame) -> Vec<TeamGameResult> {
    // During overtime the calculator assigns real ranks itself; honor those. Otherwise the
    // game ended in regulation, so compute competitive ranking from the scores.
    let overtime_ranks_assigned = calculated.overtime_ranks_assigned();
    let scores: Vec<i32> = calculated.teams.values().map(|team| team.score).collect();
    calculated
        .teams
        .iter()
        .map(|(team_idx, team)| TeamGameResult {
            team: *team_idx,
            name: team.name.clone(),
            score: team.score,
            rank: if overtime_ranks_assigned {
                team.rank
            } else {
                scores.iter().filter(|&&s| s > team.score).count() as i32 + 1
            },
        })
        .collect()
}

/// Runs the score calculator over a single game's events and returns each team's final score
/// and placement rank. Returns Err (with the calculator's messages) if the event stream is
/// invalid / cannot be scored.
///
/// Convenience wrapper over [`calculate_game`] + [`team_results_from_calculated`], kept so
/// existing callers are unchanged. Callers that need both team *and* quizzer results should
/// call [`calculate_game`] once and project twice, rather than calling this and
/// [`calculate_quizzer_results_for_game`], which replays the stream twice.
pub fn calculate_team_results_for_game(
    game_id: Uuid,
    game_events: Vec<GameEvent>,
) -> Result<Vec<TeamGameResult>, Vec<String>> {
    Ok(team_results_from_calculated(&calculate_game(game_id, game_events)?))
}

/// A single quizzer's individual results for one game, extracted from the calculator.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuizzerGameResult {
    pub team_name: String,     // the quizzer's team name
    pub name: String,          // the quizzer's name
    pub score: i32,            // individual points: tossups + bonuses + quiz-out
    pub correct: i32,          // correct tossups
    pub errors: i32,           // erroneous tossups
    pub bonus_pts: i32,        // points from correct bonuses
    pub bonus_attempts: i32,   // correct + erroneous bonus attempts
}

/// Projects each quizzer's individual counts and points out of already-calculated state.
///
/// Pure -- no DB access, no replay -- so it is safe to call per request against a cached
/// [`CalculatedGame`].
pub fn quizzer_results_from_calculated(calculated: &CalculatedGame) -> Vec<QuizzerGameResult> {
    let tossup_award = calculated.options.point_award_for_correct_tossup;
    let bonus_award = calculated.options.point_award_for_correct_bonus;
    let quiz_out_award = calculated.options.point_award_for_quizzing_out;

    let mut results = Vec::new();
    for (_team_idx, team) in calculated.teams.iter() {
        for (_seat, quizzer) in team.quizzers.iter() {
            let correct = quizzer.correct_tossups.len() as i32;
            let errors = quizzer.errors_on_tossups.len() as i32;
            let bonus_correct = quizzer.correct_bonuses.len() as i32;
            let bonus_errors = quizzer.errors_on_bonuses.len() as i32;
            let bonus_pts = bonus_correct * bonus_award;
            // A quiz-out only awards points when it happens without an erroneous tossup.
            let quizzed_out_without_error =
                quizzer.question_quizzed_out_on != -1 && quizzer.errors_on_tossups.is_empty();
            let quiz_out_pts = if quizzed_out_without_error { quiz_out_award } else { 0 };

            results.push(QuizzerGameResult {
                team_name: team.name.clone(),
                name: quizzer.name.clone(),
                score: correct * tossup_award + bonus_pts + quiz_out_pts,
                correct,
                errors,
                bonus_pts,
                bonus_attempts: bonus_correct + bonus_errors,
            });
        }
    }
    results
}

/// Runs the score calculator over a single game's events and returns each quizzer's
/// individual counts and points. Returns Err if the event stream cannot be scored.
///
/// Convenience wrapper over [`calculate_game`] + [`quizzer_results_from_calculated`], kept so
/// existing callers are unchanged.
pub fn calculate_quizzer_results_for_game(
    game_id: Uuid,
    game_events: Vec<GameEvent>,
) -> Result<Vec<QuizzerGameResult>, Vec<String>> {
    Ok(quizzer_results_from_calculated(&calculate_game(game_id, game_events)?))
}

#[derive(Clone, Debug)]
struct TeamForGameEventStreamBuilder {
    name: String,
    quizzers: [String; 6],
}
impl TeamForGameEventStreamBuilder {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            quizzers: ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
        }
    }
    pub fn get_quizzer_seat(self, quizzer_name: &str) -> Result<usize, String> {
        for (idx, name) in self.quizzers.iter().enumerate() {
            if name == quizzer_name {
                return Ok(idx);
            }
        }
        Err(format!["Quizzer name '{}' not found on team '{}'. Quizzers: {}", quizzer_name, self.name, self.quizzers.join(", ")])
    }
}

#[derive(Clone, Debug)]
pub struct GameEventStreamBuilder {
    gid: Uuid,
    correct_toss_ups_to_quiz_out: i32,
    erroneous_toss_ups_to_error_out: i32,
    fouls_to_foul_out: i32,
    maximum_timeouts_per_team_when_two_teams_playing: i32,
    maximum_timeouts_per_team_when_three_teams_playing: i32,
    teams: [TeamForGameEventStreamBuilder; 3],
    events: Vec<NewGameEvent>
}

impl GameEventStreamBuilder {
    pub fn new(game_id: Uuid) -> Self {
        Self {
            gid: game_id,
            correct_toss_ups_to_quiz_out: DEFAULT_QUIZ_OUT,
            erroneous_toss_ups_to_error_out: DEFAULT_ERROR_OUT,
            fouls_to_foul_out: DEFAULT_FOUL_OUT,
            maximum_timeouts_per_team_when_two_teams_playing: DEFAULT_2_TEAM_TIMEOUTS,
            maximum_timeouts_per_team_when_three_teams_playing: DEFAULT_3_TEAM_TIMEOUTS,
            teams: [TeamForGameEventStreamBuilder::new(), TeamForGameEventStreamBuilder::new(), TeamForGameEventStreamBuilder::new()],
            events: vec![]
        }
    }
    pub fn set_gid(mut self, val: Uuid) -> Self {
        self.gid = val;
        self
    }
    pub fn then_add_RM(self, name: &str) -> Self {

        let mut prev_question = 1;
        let mut prev_eventnum = -1;
        let prev_event_result = self.clone().get_last_event();
        if prev_event_result.is_ok() {
            let prev_event = prev_event_result.unwrap();
            prev_question = prev_event.question;
            prev_eventnum = prev_event.eventnum;
        }

        let mut new_events = self.events.clone();
        new_events.push(
            GameEventBuilder::new_default(self.gid)
                .set_event(Some(GameEventCode::RM))
                .set_question(Some(prev_question))
                .set_eventnum(Some(prev_eventnum + 1))
                .set_name(Some(name.to_string()))
                .set_team(Some(0))
                .set_quizzer(Some(0))
                .build()
                .unwrap()
        );
        Self {
            events: new_events,
            ..self
        }
    }
    pub fn then_add_QT(self, name: &str) -> Self {

        let mut prev_question = 1;
        let mut prev_eventnum = -1;
        let prev_event_result = self.clone().get_last_event();
        if prev_event_result.is_ok() {
            let prev_event = prev_event_result.unwrap();
            prev_question = prev_event.question;
            prev_eventnum = prev_event.eventnum;
        }

        let mut new_events = self.events.clone();
        new_events.push(
            GameEventBuilder::new_default(self.gid)
                .set_event(Some(GameEventCode::QT))
                .set_question(Some(prev_question))
                .set_eventnum(Some(prev_eventnum + 1))
                .set_name(Some(name.to_string()))
                .set_team(Some(0))
                .set_quizzer(Some(0))
                .build()
                .unwrap()
        );
        Self {
            events: new_events,
            ..self
        }
    }
    pub fn then_add_TN(self, name: &str, team: i32) -> Result<Self, Vec<String>> {
        // let mut errors: Vec<String> = vec![];

        let mut prev_question = 1;
        let mut prev_eventnum = -1;
        let prev_event_result = self.clone().get_last_event();
        if prev_event_result.is_ok() {
            let prev_event = prev_event_result.unwrap();
            prev_question = prev_event.question;
            prev_eventnum = prev_event.eventnum;
        }

        let mut new_events = self.events.clone();
        new_events.push(
            GameEventBuilder::new_default(self.gid)
            .set_event(Some(GameEventCode::TN))
            .set_question(Some(prev_question))
            .set_eventnum(Some(prev_eventnum + 1))
            .set_name(Some(name.to_string()))
            .set_team(Some(team))
            .set_quizzer(Some(team + 1))
            .build()
            .unwrap()
        );

        // if errors.len() > 0 {
        //     return Err(errors);
        // }
        Ok(
            Self {
                events: new_events,
                ..self
            }
        )
    }
    pub fn then_add_QN_plus_if_SC_or_SS(self, name: &str, team: i32, seat_number: i32, is_captain: bool, is_cocaptain: bool) -> Result<Self, Vec<String>> {
        let mut errors: Vec<String> = vec![];
        let prev_event = self.clone().get_last_event().unwrap();
        let prev_question = prev_event.question;
        let prev_eventnum = prev_event.eventnum;

        let mut new_events = self.events.clone();
        new_events.push(
            GameEventBuilder::new_default(self.gid)
            .set_event(Some(GameEventCode::QN))
            .set_question(Some(prev_question))
            .set_eventnum(Some(prev_eventnum + 1))
            .set_name(Some(name.to_string()))
            .set_team(Some(team))
            .set_quizzer(Some(seat_number))
            .build()
            .unwrap()
        );

        if is_captain && is_cocaptain {
            errors.push(format!["QN for '{}' for seat '{}' on team '{}' was indicated as both captain and cocaptain. QN can be only captain, cocaptain, or neither (not both).", name, seat_number, team]);
        }

        if is_captain {
            new_events.push(
                GameEventBuilder::new_default(self.gid)
                .set_event(Some(GameEventCode::SC))
                .set_question(Some(prev_question))
                .set_eventnum(Some(prev_eventnum + 2))
                .set_name(Some(name.to_string()))
                .set_team(Some(team))
                .set_quizzer(Some(seat_number))
                .build()
                .unwrap()
            );
        }
        else if is_cocaptain {
            new_events.push(
                GameEventBuilder::new_default(self.gid)
                .set_event(Some(GameEventCode::SS))
                .set_question(Some(prev_question))
                .set_eventnum(Some(prev_eventnum + 2))
                .set_name(Some(name.to_string()))
                .set_team(Some(team))
                .set_quizzer(Some(seat_number))
                .build()
                .unwrap()
            );
        }

        let mut new_quizzers = self.teams[team as usize].quizzers.clone();
        new_quizzers[seat_number as usize] = name.to_string();
        let mut new_teams = self.teams.clone();
        new_teams[team as usize].quizzers = new_quizzers;

        if errors.len() > 0 {
            return Err(errors);
        }

        Ok(
            Self {
                teams: new_teams,
                events: new_events,
                ..self
            }
        )
    }

    // *These two have been commented out because they were going to be re-introduced out of necessity 
    // for indicating new captains and cocaptains mid-game, however QuizMachine currently does not 
    // record replacement captains and cocaptains past the first ones. So, this code is not needed for
    // passing the "have at least one specified acting captain at all times" validation check.
    
    // pub fn then_add_SC(self, name: &str, team: i32, seat_number: i32) -> Result<Self, Vec<String>> {
    // }
    // pub fn then_add_SS(self, name: &str, team: i32, seat_number: i32) -> Result<Self, Vec<String>> {
    // }

    pub fn then_add_TC(self, name: &str, team: i32) -> Result<Self, Vec<String>> {
        let mut errors: Vec<String> = vec![];
        
        let prev_event = self.clone().get_last_event().unwrap();
        let prev_question = prev_event.question;
        let prev_eventnum = prev_event.eventnum;
        let prev_event = prev_event.event;
        let quizzer_seat_idx_result = self.teams[team as usize].clone().get_quizzer_seat(name);
        if quizzer_seat_idx_result.is_err() {
            errors.push(quizzer_seat_idx_result.unwrap_err());
            return Err(errors);
        }
        let quizzer_seat_idx = quizzer_seat_idx_result.unwrap();

        let mut new_question = prev_question + 1;
        let mut new_eventnum = 0;
        if prev_question == 1 && (prev_event == "QN" || prev_event == "SC" || prev_event == "SS") || prev_event == "TO" || prev_event == "F-" || prev_event == "FC"|| prev_event == "DE" {
            new_question = prev_question;
            new_eventnum = prev_eventnum + 1;
        }

        let mut new_events = self.events.clone();
        new_events.push(
            GameEventBuilder::new_default(self.gid)
                .set_event(Some(GameEventCode::TC))
                .set_question(Some(new_question))
                .set_eventnum(Some(new_eventnum))
                .set_name(Some(name.to_string()))
                .set_team(Some(team))
                .set_quizzer(Some(quizzer_seat_idx as i32))
                .build()
                .unwrap(),
        );

        let mut TC_count = 1;  // = 1 because next iter below doesn't include the currently-added TC event record
        for event in self.events.iter() {
            if event.event == "TC".to_string() && event.name == name {
                TC_count += 1;
            }
        }
        if TC_count > self.correct_toss_ups_to_quiz_out {
            errors.push(format!["Quizzer '{}' received more correct toss-up rulings than they are elligible to receive. TC Maximum: '{}', TCs Received: '{}'.", name, self.correct_toss_ups_to_quiz_out, TC_count]);
        }
        if TC_count == self.correct_toss_ups_to_quiz_out {
            new_events.push(
                GameEventBuilder::new_default(self.gid)
                    .set_event(Some(GameEventCode::QO))
                    .set_question(Some(new_question))
                    .set_eventnum(Some(new_eventnum + 1))
                    .set_name(Some(name.to_string()))
                    .set_team(Some(team))
                    .set_quizzer(Some(quizzer_seat_idx as i32))
                    .build()
                    .unwrap(),
            );
        }

        if errors.len() > 0 {
            return Err(errors);
        }
        Ok(
            Self {
                events: new_events,
                ..self
            }
        )
    }
    pub fn then_add_TE_and_bonuses(self, name: &str, team: i32, left_team_bonus_is_correct: bool, right_team_bonus_is_correct: bool) -> Result<Self, Vec<String>> {
        let mut errors: Vec<String> = vec![];
        let prev_event = self.clone().get_last_event().unwrap();
        let prev_question = prev_event.question;
        let prev_eventnum = prev_event.eventnum;
        let prev_event = prev_event.event;
        let quizzer_seat_idx_result = self.teams[team as usize].clone().get_quizzer_seat(name);
        if quizzer_seat_idx_result.is_err() {
            errors.push(quizzer_seat_idx_result.unwrap_err());
            return Err(errors);
        }
        let quizzer_seat_idx = quizzer_seat_idx_result.unwrap();

        let mut new_question = prev_question + 1;
        let mut new_eventnum = 0;
        if prev_question == 1 && (prev_event == "QN" || prev_event == "SC" || prev_event == "SS") || prev_event == "TO" || prev_event == "F-" || prev_event == "FC" || prev_event == "DE" {
            new_question = prev_question;
            new_eventnum = prev_eventnum + 1;
        }

        let mut new_events = self.events.clone();
        new_events.push(
            GameEventBuilder::new_default(self.gid)
                .set_event(Some(GameEventCode::TE))
                .set_question(Some(new_question))
                .set_eventnum(Some(new_eventnum))
                .set_name(Some(name.to_string()))
                .set_team(Some(team))
                .set_quizzer(Some(quizzer_seat_idx as i32))
                .build()
                .unwrap(),
        );

        // check for error out and insert event if necessary
        let mut TE_count = 1;
        for event in self.events.iter() {
            if event.event == "TE".to_string() && event.name == name {
                TE_count += 1;
            }
        }
        if TE_count > self.erroneous_toss_ups_to_error_out {
            errors.push(format!["Quizzer '{}' received more erroneous toss-up rulings than they are elligible to receive. TE Maximum: '{}', TEs Received: '{}'.", name, self.erroneous_toss_ups_to_error_out, TE_count]);
        }
        if TE_count == self.erroneous_toss_ups_to_error_out {
            new_eventnum += 1;
            new_events.push(
                GameEventBuilder::new_default(self.gid)
                    .set_event(Some(GameEventCode::EO))
                    .set_question(Some(new_question))
                    .set_eventnum(Some(new_eventnum))
                    .set_name(Some(name.to_string()))
                    .set_team(Some(team))
                    .set_quizzer(Some(quizzer_seat_idx as i32))
                    .build()
                    .unwrap(),
            );
        }

        // handle bonus rulings
        let mut bonus_teams: Vec<usize> = vec![];
        for num in 0..3 {
            if num != team { bonus_teams.push(num as usize); }
        }
        let (left_team_idx, right_team_idx) = match team {
            0 => (1, 2),
            1 => (0, 2),
            2 => (0, 1),
            _ => {
                errors.push("Team index was not 0, 1 or 2 when trying to determine left and right team indexes for creating bonus records.".to_string());
                return Err(errors);
            },
        };
        // left team bonus:
        let left_team_bonus_quizzer_exists = 
            self.teams[bonus_teams[0]].clone().quizzers[quizzer_seat_idx] != "".to_string();
        if left_team_bonus_quizzer_exists {
            let game_event_code_for_left_team_bonus = 
                if left_team_bonus_is_correct { GameEventCode::BC } else { GameEventCode::BE };
            new_eventnum += 1;
            new_events.push(
                GameEventBuilder::new_default(self.gid)
                    .set_event(Some(game_event_code_for_left_team_bonus))
                    .set_question(Some(new_question))
                    .set_eventnum(Some(new_eventnum))
                    .set_name(Some(name.to_string()))
                    .set_team(Some(left_team_idx))
                    .set_quizzer(Some(quizzer_seat_idx as i32))
                    .build()
                    .unwrap(),
            );
        }
        // right team bonus:
        let right_team_bonus_quizzer_exists = 
            self.teams[bonus_teams[1]].clone().quizzers[quizzer_seat_idx] != "".to_string();
        if right_team_bonus_quizzer_exists {
            let game_event_code_for_left_team_bonus = 
                if right_team_bonus_is_correct { GameEventCode::BC } else { GameEventCode::BE };
            new_eventnum += 1;
            new_events.push(
                GameEventBuilder::new_default(self.gid)
                    .set_event(Some(game_event_code_for_left_team_bonus))
                    .set_question(Some(new_question))
                    .set_eventnum(Some(new_eventnum))
                    .set_name(Some(name.to_string()))
                    .set_team(Some(right_team_idx))
                    .set_quizzer(Some(quizzer_seat_idx as i32))
                    .build()
                    .unwrap(),
            );
        }

        if errors.len() > 0 {
            return Err(errors);
        }
        Ok(
            Self {
                events: new_events,
                ..self
            }
        )
    }

    pub fn then_add_NJ(self) -> Result<Self, Vec<String>> {
        // We add a record inidicating that no quizzers jumped after full question was
        // read and 5-second timer ran out.

        let errors: Vec<String> = vec![];
        let prev_event = self.clone().get_last_event().unwrap();
        let prev_question = prev_event.question;
        let prev_eventnum = prev_event.eventnum;
        let prev_event = prev_event.event;

        let mut new_question = prev_question + 1;
        let mut new_eventnum = 0;
        if prev_question == 1 && (prev_event == "QN" || prev_event == "SC" || prev_event == "SS") || prev_event == "TO" || prev_event == "F-" || prev_event == "FC"|| prev_event == "DE" {
            new_question = prev_question;
            new_eventnum = prev_eventnum + 1;
        }

        let mut new_events = self.events.clone();
        new_events.push(
            GameEventBuilder::new_default(self.gid)
                .set_event(Some(GameEventCode::NJ))
                .set_question(Some(new_question))
                .set_eventnum(Some(new_eventnum))
                .set_name(Some("No Jump".to_string()))
                .set_team(Some(0))
                .set_quizzer(Some(0))
                .build()
                .unwrap(),
        );
        if errors.len() > 0 {
            return Err(errors);
        }
        Ok(
            Self {
                events: new_events,
                ..self
            }
        )
    }
    pub fn then_challenge_accepted(mut self, name: &str, team: i32, left_team_bonus_is_correct: bool, right_team_bonus_is_correct: bool) -> Result<Self, Vec<String>> {
        // There is no GameEventCode for accepted challenges; instead we update past records and write new 
        // records that will be used inplace of the old records for score calculations.
        // Challenges are only applied to toss-up questions. (Challeneges on bonuses are currently modified using the fix questions feature of QuizMachine.)
        
        // We update all events of the current question as having GameEventCode::DE,
        // then insert the toss-up rulling correction (TC if it was TE and TE if it was TC) and proceed as normal.
        
        let mut errors: Vec<String> = vec![];
        
        // Find the most previous "TC" or "TE" game event. This is our starting point.
        // Capture the question number.
        let mut game_event_question_of_interest = 0;
        let mut replacement_game_event_type = "".to_string();
        for game_event in self.events.iter().rev() {
            if game_event.event == "TC".to_string() {
                replacement_game_event_type = "TE".to_string();
                game_event_question_of_interest = game_event.question;
                break;
            }
            if game_event.event == "TE".to_string() {
                replacement_game_event_type = "TC".to_string();
                game_event_question_of_interest = game_event.question;
                break;
            }
        }

        // Mark all game events of this question and thereafterward as GameEventCode::DE.
        let mut new_events: Vec<NewGameEvent> = vec![];
        for game_event in self.events.iter() {
            if game_event.question < game_event_question_of_interest {
                new_events.push(game_event.clone());
            }
            else {
                let is_ruling_event = game_event.event == "TC".to_string() || game_event.event == "TE".to_string() || game_event.event == "BC".to_string() || game_event.event == "BE".to_string();
                if !is_ruling_event {
                    new_events.push(game_event.clone());
                    continue;
                }
                let updated_event = NewGameEvent {
                    event: "DE".to_string(),
                    ..game_event.clone()
                };
                new_events.push(updated_event);
            }
        }

        // Persist updated events before adding more events:
        self.events = new_events;

        // Add replacement "TE" or "TC" respectively.
        let mut record_addition_result: Result<Self, Vec<String>> = Err(vec!["This is just for initialization of variable.".to_string()]);
        if replacement_game_event_type == "TC".to_string() {
            record_addition_result = self.then_add_TC(name, team);
        }
        else {
            let self_clone = self.clone();
            let last_event_option = self_clone.events.iter().last();
            if last_event_option.is_some() {
                let last_event = last_event_option.unwrap();
                record_addition_result = self.then_add_TE_and_bonuses(last_event.name.as_str(), last_event.team, left_team_bonus_is_correct, right_team_bonus_is_correct);
            }
            else {
                errors.push("Could not get last event when adding replacement record for challenge.".to_string());
                return Err(errors);
            }
        }
        if record_addition_result.is_err() {
            if let Err(addition_errors) = record_addition_result.clone() {
                errors.extend(addition_errors);
            }
        }

        if errors.len() > 0 {
            return Err(errors);
        }
        record_addition_result  // at this point it is 'Ok'
    }
    pub fn then_add_Cminus(self, name: &str, team: i32) -> Result<Self, Vec<String>> {
        let mut errors: Vec<String> = vec![];
        let prev_event = self.clone().get_last_event().unwrap();
        let prev_question = prev_event.question;
        let prev_eventnum = prev_event.eventnum;
        let quizzer_seat_idx_result = self.teams[team as usize].clone().get_quizzer_seat(name);
        if quizzer_seat_idx_result.is_err() {
            errors.push(quizzer_seat_idx_result.unwrap_err());
            return Err(errors);
        }
        let quizzer_seat_idx = quizzer_seat_idx_result.unwrap();

        let prev_event_code = self.events.last().unwrap().event.as_str();
        match prev_event_code {
            "TC" | "TE" | "BC" | "BE" => {},
            _ => {
                errors.push(format!["'C-' event can only come after a 'TC', 'TE', 'BC' or 'BE' event. Previous event = '{}'.", prev_event_code]);
            },
        }

        let mut new_events = self.events.clone();
        new_events.push(
            GameEventBuilder::new_default(self.gid)
                .set_event(Some(GameEventCode::Cminus))
                .set_question(Some(prev_question))
                .set_eventnum(Some(prev_eventnum + 1))
                .set_name(Some(name.to_string()))
                .set_team(Some(team))
                .set_quizzer(Some(quizzer_seat_idx as i32))
                .build()
                .unwrap(),
        );
        if errors.len() > 0 {
            return Err(errors);
        }
        Ok(
            Self {
                events: new_events,
                ..self
            }
        )
    }
    pub fn then_add_Aplus(self, name: &str, team: i32) -> Result<Self, Vec<String>> {
        let errors: Vec<String> = vec![];

        // Essentially, reset the question. That looks like marking all events for the question
        // as "DE", ready for a new "TC", "TE" or "NJ".

        // Find the most previous "TC", "TE" or "NJ" game event. This is our starting point.
        // Capture the question number.
        let mut game_event_question_of_interest = 0;
        for game_event in self.events.iter().rev() {
            if game_event.event == "TC".to_string()
                || game_event.event == "TE".to_string()
                || game_event.event == "NJ".to_string() {
                game_event_question_of_interest = game_event.question;
                break;
            }
        }

        // Mark all game events of this question and thereafterward as GameEventCode::DE.
        let mut new_events: Vec<NewGameEvent> = vec![];
        for game_event in self.events.iter() {
            if game_event.question < game_event_question_of_interest {
                new_events.push(game_event.clone());
            }
            else {
                let updated_event = NewGameEvent {
                    event: "DE".to_string(),
                    ..game_event.clone()
                };
                new_events.push(updated_event);
            }
        }

        if errors.len() > 0 {
            return Err(errors);
        }
        Ok(
            Self {
                events: new_events,
                ..self
            }
        )
    }
    pub fn then_add_Aminus(self, name: &str, team: i32) -> Result<Self, Vec<String>> {
        let mut errors: Vec<String> = vec![];
        let prev_event = self.clone().get_last_event().unwrap();
        let prev_question = prev_event.question;
        let prev_eventnum = prev_event.eventnum;
        let quizzer_seat_idx_result = self.teams[team as usize].clone().get_quizzer_seat(name);
        if quizzer_seat_idx_result.is_err() {
            errors.push(quizzer_seat_idx_result.unwrap_err());
            return Err(errors);
        }
        let quizzer_seat_idx = quizzer_seat_idx_result.unwrap();

        // We add a record that the Appeal was attempted but not accepted. That is it.

        let mut new_events = self.events.clone();
        new_events.push(
            GameEventBuilder::new_default(self.gid)
                .set_event(Some(GameEventCode::Cminus))
                .set_question(Some(prev_question))
                .set_eventnum(Some(prev_eventnum + 1))
                .set_name(Some(name.to_string()))
                .set_team(Some(team))
                .set_quizzer(Some(quizzer_seat_idx as i32))
                .build()
                .unwrap(),
        );
        if errors.len() > 0 {
            return Err(errors);
        }
        Ok(
            Self {
                events: new_events,
                ..self
            }
        )
    }
    pub fn then_add_FC(self, is_coach: bool, team: i32) -> Result<Self, Vec<String>> {
        // We add a record that the coach or team received a foul,
        // where quizzer == 5 is a foul on the coach and quizzer == 6 is a foul on the team.

        let errors: Vec<String> = vec![];
        let prev_event = self.clone().get_last_event().unwrap();
        let new_question = prev_event.question + 1;
        let new_eventnum = 0;

        let quizzer_val: i32 = if is_coach { 5 } else { 6 };

        let mut new_events = self.events.clone();
        new_events.push(
            GameEventBuilder::new_default(self.gid)
                .set_event(Some(GameEventCode::FC))
                .set_question(Some(new_question))
                .set_eventnum(Some(new_eventnum))
                .set_name(Some("".to_string()))
                .set_team(Some(team))
                .set_quizzer(Some(quizzer_val))
                .build()
                .unwrap(),
        );
        if errors.len() > 0 {
            return Err(errors);
        }
        Ok(
            Self {
                events: new_events,
                ..self
            }
        )
    }
    pub fn then_add_Fminus(self, name: &str, team: i32) -> Result<Self, Vec<String>> {
        let mut errors: Vec<String> = vec![];
        let prev_event = self.clone().get_last_event().unwrap();
        let prev_question = prev_event.question;
        let prev_eventnum = prev_event.eventnum;
        let quizzer_seat_idx_result = self.teams[team as usize].clone().get_quizzer_seat(name);
        if quizzer_seat_idx_result.is_err() {
            errors.push(quizzer_seat_idx_result.unwrap_err());
            return Err(errors);
        }
        let quizzer_seat_idx = quizzer_seat_idx_result.unwrap();

        let mut new_question = prev_question + 1;
        let mut new_eventnum = 0;
        if prev_question < 2 {  // alternatively could be == 1 instead
            new_question = prev_question;
            new_eventnum = prev_eventnum;
        }

        let mut quizzer_foul_count = 0;
        for event in self.events.iter() {
            if event.event == "Fminus".to_string() && event.name == name {
                quizzer_foul_count += 1;
            }
        }
        if quizzer_foul_count > self.correct_toss_ups_to_quiz_out {
            errors.push(format!["Quizzer '{}' received more fouls than they are elligible to receive. Fouls Maximum: '{}', Fouls Received: '{}'.", name, self.fouls_to_foul_out, quizzer_foul_count]);
        }

        let mut new_events = self.events.clone();
        new_events.push(
            GameEventBuilder::new_default(self.gid)
                .set_event(Some(GameEventCode::Fminus))
                .set_question(Some(new_question))
                .set_eventnum(Some(new_eventnum))
                .set_name(Some(name.to_string()))
                .set_team(Some(team))
                .set_quizzer(Some(quizzer_seat_idx as i32))
                .build()
                .unwrap(),
        );

        if errors.len() > 0 {
            return Err(errors);
        }
        Ok(
            Self {
                events: new_events,
                ..self
            }
        )
    }
    pub fn then_add_SB(self, name: &str, team: i32, new_seat: i32) -> Result<Self, Vec<String>> {
        let errors: Vec<String> = vec![];

        // System Behavior to emulate:
        // 1. Insert "SB" event for *the quizzer being subbed-in* (this is the quizzer reflected in 'name' param)
        // 2. Also add event "QN" for same subbed-in that quizzer to update the quizzer on the seat
        // 3. Lastly, add "QN" event for the quizzer subbed-out (*takes seat 5 (idx = 4) which subbed-in quizzer was in before)

        let prev_event = self.clone().get_last_event().unwrap();
        let prev_question = prev_event.question;
        let prev_eventnum = prev_event.eventnum;

        let mut new_question = prev_question + 1;
        let mut new_eventnum = 0;
        if prev_question < 2 {  // alternatively could be == 1 instead
            new_question = prev_question;
            new_eventnum = prev_eventnum;
        }

        let original_quizzer_name_in_substitution_seat = self.teams[team as usize].clone().quizzers[new_seat as usize].clone();

        let mut new_events = self.events.clone();
        new_events.push(
            GameEventBuilder::new_default(self.gid)
                .set_event(Some(GameEventCode::SB))
                .set_question(Some(new_question))
                .set_eventnum(Some(new_eventnum))
                .set_name(Some(name.to_string()))
                .set_team(Some(team))
                .set_quizzer(Some(new_seat))
                .build()
                .unwrap(),
        );

        new_eventnum += 1;
        
        // For the quizzer subbing-IN:
        new_events.push(
            GameEventBuilder::new_default(self.gid)
            .set_event(Some(GameEventCode::QN))
            .set_question(Some(new_question))
            .set_eventnum(Some(new_eventnum))
            .set_name(Some(name.to_string()))
            .set_team(Some(team))
            .set_quizzer(Some(new_seat))
            .build()
            .unwrap()
        );

        new_eventnum += 1;

        // For the quizzer subbing-OUT:
        let sub_seat_number = 4;
        new_events.push(
            GameEventBuilder::new_default(self.gid)
            .set_event(Some(GameEventCode::QN))
            .set_question(Some(new_question))
            .set_eventnum(Some(new_eventnum))
            .set_name(Some(original_quizzer_name_in_substitution_seat))
            .set_team(Some(team))
            .set_quizzer(Some(sub_seat_number))
            .build()
            .unwrap()
        );

        if errors.len() > 0 {
            return Err(errors);
        }
        Ok(
            Self {
                events: new_events,
                ..self
            }
        )
    }
    pub fn then_add_TO(self, team: i32) -> Result<Self, Vec<String>> {
        // NOTE: QuizMachine, at the time of implementation, does NOT track whether the Captain or the Coach made the T0)
        
        let mut errors: Vec<String> = vec![];
        
        let prev_event = self.clone().get_last_event().unwrap();
        let new_question = prev_event.question + 1;
        let new_eventnum = 0;

        let mut team_count = 0;
        for team in self.teams.iter() {
            if team.name != "".to_string() {
                team_count += 1;
            }
        }
        let mut timeout_counter_for_team = 0;
        for event in self.events.iter() {
            if event.event == "TO".to_string() && event.team == team {
                timeout_counter_for_team += 1;
            }
        }
        let maximum_timeouts_per_team = if team_count > 2 { self.maximum_timeouts_per_team_when_three_teams_playing } else { self.maximum_timeouts_per_team_when_two_teams_playing };
        if timeout_counter_for_team >= maximum_timeouts_per_team {
            errors.push(format!["Maximum Timeouts exceeded. Maximum Timeouts for {} teams: {}, Timeouts granted: {}.", team_count, maximum_timeouts_per_team, timeout_counter_for_team]);
        }

        if errors.len() > 0 {
            return Err(errors);
        }

        let mut new_events = self.events.clone();
        new_events.push(
            GameEventBuilder::new_default(self.gid)
                .set_event(Some(GameEventCode::TO))
                .set_question(Some(new_question))
                .set_eventnum(Some(new_eventnum))
                .set_name(Some("".to_string()))
                .set_team(Some(team))
                .set_quizzer(Some(-1))
                .build()
                .unwrap(),
        );

        Ok(
            Self {
                events: new_events,
                ..self
            }
        )
    }
    pub fn then_remove_questions(self, question_to_return_to: i32) -> Result<Self, Vec<String>> {
        let mut new_events = Vec::<NewGameEvent>::new();
        for new_game_event in self.events.iter() {
            if new_game_event.question >= question_to_return_to {
                new_events.push(
                    NewGameEvent {
                        event: "DE".to_string(),
                        ..new_game_event.clone()
                    }
                );
                continue;
            }
            new_events.push(new_game_event.clone());
        }

        Ok(
            Self {
                events: new_events,
                ..self
            }
        )
    }
    fn get_last_event(self) -> Result<NewGameEvent, String> {
        // * event sort fn should be added here once it exists
        for new_game_event in self.events.iter().rev() {
            if new_game_event.event == "DE".to_string() {
                continue;
            }
            return Ok(new_game_event.clone());
        }
        Err("Could not find valid previous event for getting last question and eventnum.".to_string())
    }
    pub fn to_game_events(self) -> (Vec<GameEvent>, Uuid) {
        let mut mut_game_events: Vec<GameEvent> = vec![];
        for new_game_event in self.events {
            mut_game_events.push(GameEvent::new_from_new_game_event(new_game_event));
        }
        (mut_game_events, self.gid)
    }

    // No stand-alone 'build' method needed. (NewGameEvents already exist in 'events' Vec.)

    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<Vec<GameEvent>> {
        let mut game_events = vec![];
        for new_game_event in self.events.iter() {
            let game_event_result = create(db, new_game_event);
            if game_event_result.is_err() {
                return Err(game_event_result.unwrap_err());
            }
            game_events.push(game_event_result.unwrap());
        }
        Ok(game_events)
    }
}

struct GameEventStreamValidator {
    events: Vec<GameEvent>,
    check_for_everything: bool,  // <- this overrides everything below by checking for everything in the 'validate' method
    check_for_sort_order: bool,
    check_for_has_RM_and_QT: bool,
    check_for_min_one_team_and_min_one_quizzer_per_team: bool,
    check_for_captains_and_cocaptains_are_accurate_based_on_number_of_quizzers_on_team: bool,
    check_for_no_team_name_duplicates: bool,
    check_for_no_quizzer_name_duplicates_within_team: bool,
    // Ideas for Potential Validation Checks:
        // check_for_captain_and_cocaptain_of_each_team_are_specified_before_first_question (TC,TE,NJ)
        // check_for_team_name_redefined_after_round_began
        // check_for_quizzer_name_redefined_after_round_began
        // check_for_QO_and_EO_found_only_on_question_where_correct_amount_is_met: bool,
        // check_for_QO_and_EO_occur_maximum_of_once_per_quizzer: bool,
        // check_for_quizzer_is_not_found_in_events_after_QO_or_EO: bool,
        // check_for_after_TE_each_elligible_quizzer_has_one_bonus_event_before_moving_on: bool,
        // check_for_TO_and_SB_must_occur_between_questions_only: bool,
        // check_for_all_events_have_correct_seat_assignments_for_quizzers_and_teams: bool,
        // check_for_events_exist_only_for_associated_teams_and_quizzersjesse_of_those_teams: bool,
        // check_for_teams_do_not_exceed_maximum_timeouts: bool,
        // check_for_teams_do_not_exceed_maximum_challenges: bool,
        // check_for_each_quizzer_has_only_one_or_less_of_QO_EO_FO: false,
}

impl GameEventStreamValidator {
    pub fn new(events: Vec<GameEvent>) -> Self {
        Self {
            events,
            check_for_everything: false,
            check_for_sort_order: false,
            check_for_has_RM_and_QT: false,
            check_for_min_one_team_and_min_one_quizzer_per_team: false,
            check_for_captains_and_cocaptains_are_accurate_based_on_number_of_quizzers_on_team: false,
            check_for_no_team_name_duplicates: false,
            check_for_no_quizzer_name_duplicates_within_team: false,
        }
    }

    pub fn check_for_everything(self) -> Self {
    // validation rule: Game must have a "RM" and "QT"
        Self {
            check_for_everything: true,
            ..self
        }
    }

    pub fn check_for_sort_order(self) -> Self {
    // validation rule: GameEvents must be in the right order to be interpreted correctly.
        Self {
            check_for_sort_order: true,
            ..self
        }
    }

    pub fn check_for_has_RM_and_QT(self) -> Self {
    // validation rule: Game must have a "RM" and "QT"
        Self {
            check_for_has_RM_and_QT: true,
            ..self
        }
    }
    
    pub fn check_for_min_one_team_and_min_one_quizzer_per_team(self) -> Self {
        Self {
            check_for_min_one_team_and_min_one_quizzer_per_team: true,
            ..self
        }
    }
    
    pub fn check_for_captains_and_cocaptains_are_accurate_based_on_number_of_quizzers_on_team(self) -> Self {
    // validation rule: Count the quizzers of a team: if only 1, must be captain, if 2 or more, captain and CC are needed
        Self {
            check_for_captains_and_cocaptains_are_accurate_based_on_number_of_quizzers_on_team: true,
            ..self
        }
    }
    
    pub fn check_for_no_team_name_duplicates(self) -> Self {
        Self {
            check_for_no_team_name_duplicates: true,
            ..self
        }
    }
    
    pub fn check_for_no_quizzer_name_duplicates_within_team(self) -> Self {
        Self {
            check_for_no_quizzer_name_duplicates_within_team: true,
            ..self
        }
    }

    pub fn validate(mut self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        self.events.sort();

        if self.check_for_everything || self.check_for_sort_order {
            // validation rule: GameEvents must be in the right order to be interpreted correctly.
            let mut question = -1;
            let mut eventnum = -1;
            for game_event in self.events.iter() {
                if question == -1 && eventnum == -1 {
                    question = game_event.question;
                    eventnum = game_event.eventnum;
                    continue;
                }
                if question == game_event.question {
                    if eventnum > game_event.eventnum {
                        errors.push(format!["Non-sequential eventnums: Eventnum {} is less than next expected next eventnum {} for question {}.", game_event.eventnum, (eventnum + 1), game_event.question]);
                    }
                    if eventnum == game_event.eventnum {
                        errors.push(format!["Non-sequential eventnums: Eventnum {} is equal to next expected next eventnum {} for question {}.", game_event.eventnum, (eventnum + 1), game_event.question]);
                    }
                    if eventnum + 1 != game_event.eventnum {
                        errors.push(format!["Non-sequential eventnums: Eventnum {} was skipped/is missing for question {}.", (eventnum + 1), game_event.question]);
                    }
                }
                if question + 1 == game_event.question && game_event.eventnum != 0 {
                    errors.push(format!["Non-sequential questions: Question {} is missing eventnum 0.", (question + 1)]);
                }
                if question > game_event.question {
                    errors.push(format!["Non-sequential questions: Question {} is less than next expected next question {}.", game_event.question, (question + 1)]);
                }
                if game_event.eventnum == 0 && question + 1 != game_event.question {
                    errors.push(format!["Non-sequential questions: Question {} was skipped/is missing.", (question + 1)]);
                }
                question = game_event.question;
                eventnum = game_event.eventnum;
            }
        }

        if self.check_for_everything || self.check_for_has_RM_and_QT {
            let mut has_rm = false;
            let mut has_qt = false;
            for game_event in self.events.iter() {
                let game_event_code = string_to_gameeventcode(game_event.event.as_str());
                match game_event_code {
                    GameEventCode::RM => { has_rm = true; },
                    GameEventCode::QT =>  { has_qt = true; },
                    _ => {}
                }
            }
            if !has_rm {
                errors.push("organization ('RM') not specified and is required".to_string());
            }
            if !has_qt {
                errors.push("quiz_type ('QT') not specified and is required".to_string());
            }
        }

        if self.check_for_everything || self.check_for_min_one_team_and_min_one_quizzer_per_team {
            // Teams and Quizzers:
            let mut team_names: [String; 3] = ["".to_string(), "".to_string(), "".to_string()];
            let mut quizzer_names_per_team: [[String; 6]; 3] = [["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()], ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()], ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()]];

            for game_event in self.events.iter() {
                let game_event_code = string_to_gameeventcode(game_event.event.as_str());
        
                // - Game must have at least one "TN" (team name)
                if game_event_code == GameEventCode::TN {
                    team_names[game_event.team as usize] = game_event.name.clone();
                }
        
                // - Each Team must have at least 1 "QN" (quizzer name)
                if game_event_code == GameEventCode::QN {
                    let team_idx = game_event.team as usize;
                    let quizzer_idx = game_event.quizzer as usize;
                    quizzer_names_per_team[team_idx][quizzer_idx] = game_event.name.clone();
                }
            }
            
            let mut at_least_one_team = false;
            for (team_idx, team_name) in team_names.iter().enumerate() {
                if *team_name != "" {
                    at_least_one_team = true;
                    let mut at_least_one_quizzer = false;
                    for quizzer_name in quizzer_names_per_team[team_idx].iter() {
                        if *quizzer_name != "" {
                            at_least_one_quizzer = true;
                        }
                    }
                    if !at_least_one_quizzer {
                        errors.push("One or more teams have zero quizzers specified. A minimum of one quizzer is required per team.".to_string());
                    }
                }
            }
            if !at_least_one_team {
                errors.push("Zero teams were found for this Game. At least one team is required per Game.".to_string());
            }
        }

        if self.check_for_everything || self.check_for_has_RM_and_QT {
            let mut has_rm = false;
            let mut has_qt = false;
            for game_event in self.events.iter() {
                let game_event_code = string_to_gameeventcode(game_event.event.as_str());
                match game_event_code {
                    GameEventCode::RM => { has_rm = true; },
                    GameEventCode::QT =>  { has_qt = true; },
                    _ => {}
                }
            }
            if !has_rm {
                errors.push("organization ('RM') not specified and is required".to_string());
            }
            if !has_qt {
                errors.push("quiz_type ('QT') not specified and is required".to_string());
            }
        }

        if self.check_for_everything || self.check_for_captains_and_cocaptains_are_accurate_based_on_number_of_quizzers_on_team {
            let mut team_names: [String; 3] = ["".to_string(), "".to_string(), "".to_string()];
            let mut quizzer_names_per_team: [[String; 6]; 3] = [["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()], ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()], ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()]];
            let mut captain_per_team: [String; 3] = ["".to_string(), "".to_string(), "".to_string()];
            let mut cocaptain_per_team: [String; 3] = ["".to_string(), "".to_string(), "".to_string()];

            for game_event in self.events.iter() {
                let game_event_code = string_to_gameeventcode(game_event.event.as_str());
        
                if game_event_code == GameEventCode::TN {
                    team_names[game_event.team as usize] = game_event.name.clone();
                }
        
                if game_event_code == GameEventCode::QN {
                    let team_idx = game_event.team as usize;
                    let quizzer_idx = game_event.quizzer as usize;
                    quizzer_names_per_team[team_idx][quizzer_idx] = game_event.name.clone();
                }

                if game_event_code == GameEventCode::SC {
                    if captain_per_team[game_event.team as usize] != "".to_string() {
                        errors.push(format!["More than one captain specified. Each team can have only one starter captain. Team idx: {}", game_event.team]);
                    }
                    captain_per_team[game_event.team as usize] = game_event.name.clone();
                }

                if game_event_code == GameEventCode::SS {
                    if cocaptain_per_team[game_event.team as usize] != "".to_string() {
                        errors.push(format!["More than one cocaptain specified. Each team can have only one starter cocaptain. Team idx: {}", game_event.team]);
                    }
                    cocaptain_per_team[game_event.team as usize] = game_event.name.clone();
                }
            }
            
            for (team_idx, team_name) in team_names.iter().enumerate() {
                if *team_name != "" {
                    let mut quizzers_count = 0;
                    for quizzer_name in quizzer_names_per_team[team_idx].iter() {
                        if *quizzer_name != "".to_string() {
                            quizzers_count += 1;
                        }
                    }
                    let requires_cocap_also = quizzers_count > 1;
                    let cap = captain_per_team[team_idx].clone();
                    let cocap = cocaptain_per_team[team_idx].clone();
                    let cap_is_in_teams_quizzers = quizzer_names_per_team[team_idx].contains(&cap);
                    let cocap_is_in_teams_quizzers = quizzer_names_per_team[team_idx].contains(&cocap);
                    if cap == "".to_string() {
                        errors.push(format!["Captain is not listed. Each team must have one starter captain. Team: {}", team_name]);
                    }
                    else if requires_cocap_also && cocap == "".to_string() {
                        errors.push(format!["Cocaptain is not listed. Each team that has more than 1 quizzer must have one starter cocaptain. Team: {}", team_name]);
                    }
                    else if requires_cocap_also && cap == cocap {
                        errors.push(format!["Captain is also listed as cocaptain. Quizzer cannot be both captain and cocaptain. Quizzer: {}, Team: {}", cap, team_name]);
                    }
                    if !cap_is_in_teams_quizzers {
                        errors.push(format!["Captain is not listed as a quizzer of the team. Captain: {}, Team: {}", cap, team_name]);
                    }
                    if !cocap_is_in_teams_quizzers {
                        errors.push(format!["Cocaptain is not listed as a quizzer of the team. Cocaptain: {}, Team: {}", cocap, team_name]);
                    }
                }
            }
        }

        if self.check_for_everything || self.check_for_no_team_name_duplicates {
            let mut team_names: [String; 3] = ["".to_string(), "".to_string(), "".to_string()];
            for game_event in self.events.iter() {
                let game_event_code = string_to_gameeventcode(game_event.event.as_str());
        
                if game_event_code == GameEventCode::TN {
                    team_names[game_event.team as usize] = game_event.name.clone();
                }
            }
            if team_names[0] == team_names[1] && team_names[0] != "".to_string() {
                errors.push(format!["Team names are not unique. Left & Center Team name: {}", team_names[0]]);
            }
            if team_names[1] == team_names[2] && team_names[1] != "".to_string() {
                errors.push(format!["Team names are not unique. Center & Right Team name: {}", team_names[1]]);
            }
            if team_names[0] == team_names[2] && team_names[0] != "".to_string() {
                errors.push(format!["Team names are not unique. Left & Right Team name: {}", team_names[0]]);
            }
        }

        if self.check_for_everything || self.check_for_no_quizzer_name_duplicates_within_team {
            let mut team_names: [String; 3] = ["".to_string(), "".to_string(), "".to_string()];
            let mut quizzer_names_per_team: [[String; 6]; 3] = [["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()], ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()], ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()]];
            for game_event in self.events.iter() {
                let game_event_code = string_to_gameeventcode(game_event.event.as_str());
        
                if game_event_code == GameEventCode::TN {
                    team_names[game_event.team as usize] = game_event.name.clone();
                }
        
                if game_event_code == GameEventCode::QN {
                    let team_idx = game_event.team as usize;
                    let quizzer_idx = game_event.quizzer as usize;
                    quizzer_names_per_team[team_idx][quizzer_idx] = game_event.name.clone();
                }
            }

            for (team_idx, team_name) in team_names.iter().enumerate() {
                if *team_name == "" { continue; }
                for (i, quizzer_i) in quizzer_names_per_team[team_idx].iter().enumerate() {
                    if *quizzer_i == "" { continue; }
                    for (j, quizzer_j) in quizzer_names_per_team[team_idx].iter().enumerate() {
                        if i == j || *quizzer_j == "" { continue; }
                        if quizzer_i == quizzer_j {
                            errors.push(format!["Team '{}' has more than 1 quizzer with the name '{}'. Quizzer names must be unique for each team.", team_names[team_idx], quizzer_j]);
                        }
                    }
                }
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }
}


#[derive(PartialEq,Clone)]
pub enum GameEventCode {
    // Configuration events:
    RM,      // Room setup data/Rules/Method ('Tournament' is valid input for 'Name' property, indicating this defines the type of Game)
    QT,      // Quiz Type/Organization (almost always "N" for Nazarene)
    IP,      //* "Is Practice"? Valid values include but are not limited to: 'Name' = 'Practice'
    OP,      //* "Option"? "Other Practice"? Valid values include but are not limited to: 'Name' = 'QuizOut', 'ErrorOut', 'FoulOut', 'QuizzerDeduct', 'TeamDeduct' (*looks like this indicates a change in the rules from the default)
    TN,      // Team Name
    QN,      // Quizzer Name (overwrites Quizzer name for that seat on the team)
    SC,      // Team Captain (C); currently if QuizMachine's captain and cocaptain both become inelligible, then when an appeal by their team is accepted the 'quizzer' of the game event = -1 and QuizMachine asks the quizmaster to specify captain and cocaptain again
    SS,      // Team Co-Captain (CC)
    // In-Game events:
    TC,      // Toss-up, Correct
    TE,      // Toss-up, Error
    NJ,      // "No Jump"; no quizzer jumped after full question was read and 5-second timer expired.
    BC,      // Bonus, Correct
    BE,      // Bonus, Error
    QO,      // Quiz Out
    EO,      // Error Out
    //"C+"   // *there is no 'C+', it is not recorded. Instead, 'DE' is used to 'erase' data when a 'C+' would normally occur.
    Cminus,  // "C-", Challenge, Overruled
    Aplus,   // "A+", Appeal, Accepted. Always comes after 'DE' and has 
    Aminus,  // "A-", Appeal, Overruled
    //FO,    // *no code for "Foul Out"; only 'F-' for each individual foul; QuizMachine tracks Foul Outs in memory only
    FC,      // Foul Coach/Foul Team? Valid 'Name's: '1', {blank}. 'Foul Team' is indicated by 'quizzer' = 6/ 'Foul Coach' is indicated by 'quizzer' = 5.
    Fminus,  // "F-", Foul on quizzer
    //"F+"   // There is no reason for an 'F+' - fouls are always bad :)
    SB,      // Substitution, one quizzer for another quizzer
    TO,      // TimeOut ('Quizzer' = '-1' always; starts a new question with eventnum = '0' (zero); 'name' is always blank)
    DE,      // Data Entry (generic). Could also mean "Delete" based on current usage. Gets new question number with eventnnum set to zero (for A+; comes before A+ to modify previous data based on other matching col values with most previous even with those details). 

    // QuizMachine keeps the events in memory, then when a "Fix" happens each event no longer used is overwritten/mutated to have event = "DE", meaning, keep this record but don't use it for calculating the score.
    // Could improve this by adding a col for "is_del" to indicate "don't count this toward score calculations". Current implementaiton results in data loss of the original event type.
}

impl GameEventCode {
    pub fn to_string(self) -> String {
        match self {
            Self::RM     => "RM".to_string(),
            Self::QT     => "QT".to_string(),
            Self::IP     => "IP".to_string(),
            Self::OP     => "OP".to_string(),
            Self::TN     => "TN".to_string(),
            Self::QN     => "QN".to_string(),
            Self::SS     => "SS".to_string(),
            Self::SC     => "SC".to_string(),
            Self::TC     => "TC".to_string(),
            Self::TE     => "TE".to_string(),
            Self::NJ     => "NJ".to_string(),
            Self::BC     => "BC".to_string(),
            Self::BE     => "BE".to_string(),
            Self::QO     => "QO".to_string(),
            Self::EO     => "EO".to_string(),
            Self::Cminus => "C-".to_string(),
            Self::Aplus  => "A+".to_string(),
            Self::Aminus => "A-".to_string(),
            Self::FC     => "FC".to_string(),
            Self::Fminus => "F-".to_string(),
            Self::SB     => "SB".to_string(),
            Self::TO     => "TO".to_string(),
            Self::DE     => "DE".to_string(),
        }
    }
}

pub fn string_to_gameeventcode(str_code: &str) -> GameEventCode {
    match str_code {
        "RM" => GameEventCode::RM,
        "QT" => GameEventCode::QT,
        "IP" => GameEventCode::IP,
        "OP" => GameEventCode::OP,
        "TN" => GameEventCode::TN,
        "QN" => GameEventCode::QN,
        "SS" => GameEventCode::SS,
        "SC" => GameEventCode::SC,
        "TC" => GameEventCode::TC,
        "TE" => GameEventCode::TE,
        "NJ" => GameEventCode::NJ,
        "BC" => GameEventCode::BC,
        "BE" => GameEventCode::BE,
        "QO" => GameEventCode::QO,
        "EO" => GameEventCode::EO,
        "C-" => GameEventCode::Cminus,
        "A+" => GameEventCode::Aplus,
        "A-" => GameEventCode::Aminus,
        "FC" => GameEventCode::FC,
        "F-" => GameEventCode::Fminus,
        "SB" => GameEventCode::SB,
        "TO" => GameEventCode::TO,
        "DE" => GameEventCode::DE,
        _ => panic!("Unknown game event code: {}", str_code)
    }
}



pub struct GameEventBuilder {
    gid: Uuid,
    question: Option<i32>,
    eventnum: Option<i32>,
    name: Option<String>,
    team: Option<i32>,
    quizzer: Option<i32>,
    event: Option<GameEventCode>,
    parm1: Option<String>,
    parm2: Option<String>,
    clientts: Option<DateTime<Utc>>,
    md5digest: Option<String>,
    source: Option<String>,
}

impl GameEventBuilder {
    pub fn new(game_id: Uuid) -> Self {
        Self {
            gid: game_id,
            question: None,
            eventnum: None,
            name: None,
            team: None,
            quizzer: None,
            event: None,
            parm1: None,
            parm2: None,
            clientts: None,
            md5digest: None,
            source: None,
        }
    }
    pub fn new_default(game_id: Uuid) -> Self {
        Self {
            gid: game_id,
            question: None,
            eventnum: None,
            name: None,
            team: None,
            quizzer: None,
            event: None,
            parm1: Some("".to_string()),
            parm2: Some("".to_string()),
            clientts: Some(Utc::now()),
            md5digest: Some("".to_string()),
            source: None,
        }
    }
    pub fn new_empty(event: String) -> Self {
        Self {
            gid: Uuid::nil(),
            question: Some(-1),
            eventnum: Some(-1),
            name: Some("".to_string()),
            team: Some(-1),
            quizzer: Some(-1),
            event: Some(string_to_gameeventcode(event.as_str())),
            parm1: Some("".to_string()),
            parm2: Some("".to_string()),
            clientts: Some(Utc::now()),
            md5digest: Some("".to_string()),
            source: None,
        }
    }
    pub fn set_gid(mut self, val: Uuid) -> Self {
        self.gid = val;
        self
    }
    pub fn set_question(mut self, val: Option<i32>) -> Self {
        self.question = val;
        self
    }
    pub fn set_eventnum(mut self, val: Option<i32>) -> Self {
        self.eventnum = val;
        self
    }
    pub fn set_name(mut self, val: Option<String>) -> Self {
        self.name = val;
        self
    }
    pub fn set_team(mut self, val: Option<i32>) -> Self {
        self.team = val;
        self
    }
    pub fn set_quizzer(mut self, val: Option<i32>) -> Self {
        self.quizzer = val;
        self
    }
    pub fn set_event(mut self, val: Option<GameEventCode>) -> Self {
        self.event = val;
        self
    }
    pub fn set_event_using_string(mut self, val: String) -> Self {
        self.event = Some(string_to_gameeventcode(val.as_str()));
        self
    }
    pub fn set_parm1(mut self, val: Option<String>) -> Self {
        self.parm1 = val;
        self
    }
    pub fn set_parm2(mut self, val: Option<String>) -> Self {
        self.parm2 = val;
        self
    }
    pub fn set_clientts(mut self, val: Option<DateTime<Utc>>) -> Self {
        self.clientts = val;
        self
    }
    pub fn set_md5digest(mut self, val: Option<String>) -> Self {
        self.md5digest = val;
        self
    }
    pub fn set_source(mut self, val: Option<String>) -> Self {
        self.source = val;
        self
    }
    fn validate_all_are_some(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.question.is_none() {
            errors.push("question is required".to_string());
        }
        if self.eventnum.is_none() {
            errors.push("eventnum is required".to_string());
        }
        if self.name.is_none() {
            errors.push("name is required".to_string());
        }
        if self.team.is_none() {
            errors.push("team is required".to_string());
        }
        if self.quizzer.is_none() {
            errors.push("quizzer is required".to_string());
        }
        if self.event.is_none() {
            errors.push("org is required".to_string());
        }
        if self.clientts.is_none() {
            errors.push("clientts is required".to_string());
        }
        if self.md5digest.is_none() {
            errors.push("md5digest is required".to_string());
        }

        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(())
    }
    pub fn build(self) -> Result<NewGameEvent, Vec<String>> {
        match self.validate_all_are_some() {
            Err(e) => {
                Err(e)
            },
            Ok(_) => {
                Ok(
                    NewGameEvent {
                        gid: self.gid,
                        question: self.question.unwrap(),
                        eventnum: self.eventnum.unwrap(),
                        name: self.name.unwrap(),
                        team: self.team.unwrap(),
                        quizzer: self.quizzer.unwrap(),
                        event: self.event.unwrap().to_string(),
                        parm1: self.parm1.unwrap_or_else(|| "".to_string()),
                        parm2: self.parm2.unwrap_or_else(|| "".to_string()),
                        clientts: self.clientts.unwrap(),
                        serverts: Utc::now(),
                        md5digest: self.md5digest.unwrap(),
                        qm_registration_key: None,
                        source: self.source.clone(),
                    }
                )
            }
        }
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<GameEvent> {
        let new_entity = self.build();
        create(db, &new_entity.unwrap())
    }
}

// Now define the tables that will store each game event
// // #[tsync::tsync]
#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Identifiable,
    AsChangeset,
    ToSchema
)]
#[diesel(table_name = crate::schema::gameevents)]
#[diesel(primary_key(gid,question,eventnum))]
pub struct GameEvent {
    pub gid: Uuid,
    pub question: i32,
    pub eventnum: i32,
    pub name: String,
    pub team: i32,
    pub quizzer: i32,
    pub event: String,
    pub parm1: String,
    pub parm2: String,
    pub clientts: DateTime<Utc>,
    pub serverts: DateTime<Utc>,
    pub md5digest: String,
    pub qm_registration_key: Option<String>,
    pub source: Option<String>,
}
impl GameEvent {
    pub fn new_from_new_game_event(new_game_event: NewGameEvent) -> Self {
        Self {
            gid: new_game_event.gid,
            question: new_game_event.question,
            eventnum: new_game_event.eventnum,
            name: new_game_event.name,
            team: new_game_event.team,
            quizzer: new_game_event.quizzer,
            event: new_game_event.event,
            parm1: new_game_event.parm1,
            parm2: new_game_event.parm2,
            clientts: new_game_event.clientts,
            serverts: new_game_event.serverts,
            md5digest: new_game_event.md5digest,
            qm_registration_key: new_game_event.qm_registration_key.clone(),
            source: new_game_event.source.clone(),
        }
    }
}

trait SortGameEvents {
    fn sort(&mut self);
}
impl SortGameEvents for Vec<GameEvent> {
    fn sort(&mut self) {
        self.sort_by(|a,b| {
            a.question.cmp(&b.question)
                .then_with(|| a.eventnum.cmp(&b.eventnum))
        })
    }
}

#[derive(
    Insertable,
    Serialize,
    Deserialize,
    Debug,
    Clone
)]
#[diesel(table_name = crate::schema::gameevents)]
pub struct NewGameEvent {
    pub gid: Uuid,
    pub question: i32,
    pub eventnum: i32,
    pub name: String,
    pub team: i32,
    pub quizzer: i32,
    pub event: String,
    pub parm1: String,
    pub parm2: String,
    pub clientts: DateTime<Utc>,
    pub serverts: DateTime<Utc>,
    pub md5digest: String,
    pub qm_registration_key: Option<String>,
    pub source: Option<String>,
}

impl NewGameEvent {
    pub fn empty() -> Self {
        // Now populate the quizzes event
        return Self {
            gid: Uuid::nil(),
            question: -1,
            eventnum: -1,
            name: "".to_string(),
            team: -1,
            quizzer: -1,
            event: "".to_string(),
            parm1: "".to_string(),
            parm2: "".to_string(),
            clientts: Utc::now(),
            serverts: Utc::now(),
            md5digest: "".to_string(),
            qm_registration_key: None,
            source: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::gameevents)]
pub struct GameEventChangeset {
    pub name: String,
    pub team: i32,
    pub quizzer: i32,
    pub event: String,
    pub parm1: String,
    pub parm2: String,
    pub clientts: DateTime<Utc>,
    pub serverts: DateTime<Utc>,
    pub md5digest: String,
    pub qm_registration_key: Option<String>,
    pub source: Option<String>,
}

impl GameEventChangeset {
    pub fn from_newgameevent(item: &NewGameEvent) -> Self {
        Self {
            name: item.name.clone(),
            team: item.team,
            quizzer: item.quizzer,
            event: item.event.clone(),
            parm1: item.parm1.clone(),
            parm2: item.parm2.clone(),
            clientts: item.clientts,
            serverts: item.serverts,
            md5digest: item.md5digest.clone(),
            qm_registration_key: item.qm_registration_key.clone(),
            source: item.source.clone(),
        }
    }
}

pub fn create(db: &mut database::Connection, item: &NewGameEvent) -> QueryResult<GameEvent> {
    use crate::schema::gameevents::dsl::*;
    insert_into(gameevents).values(item).get_result::<GameEvent>(db)
}

pub fn exists(db: &mut database::Connection, game_id: Uuid, question_num: i32, event_num: i32) -> bool {
    use crate::schema::gameevents::dsl::*;
    diesel::select(diesel::dsl::exists(
        gameevents
            .filter(gid.eq(game_id))
            .filter(question.eq(question_num))
            .filter(eventnum.eq(event_num))
    ))
    .get_result::<bool>(db)
    .unwrap_or(false)
}

// pub fn read(db: &mut database::Connection, item_id: Uuid) -> QueryResult<GameEvent> {
//     use crate::schema::gameevents::dsl::*;
//     gameevents.filter(gid.eq(item_id)).first::<GameEvent>(db)
// }

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<GameEvent>> {
    use crate::schema::gameevents::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    gameevents
        .order(gid)
        .limit(page_size)
        .offset(offset_val)
        .load::<GameEvent>(db)
}

pub fn count(db: &mut database::Connection) -> QueryResult<i64> {
    use crate::schema::gameevents::dsl::*;
    gameevents.count().get_result(db)
}

pub fn read_all_gameevents_of_game(db: &mut database::Connection, game_id: Uuid, pagination: &PaginationParams) -> QueryResult<Vec<GameEvent>> {
    use crate::schema::gameevents::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    gameevents
        .filter(gid.eq(game_id))
        .order(gid)
        .limit(page_size)
        .offset(offset_val)
        .load::<GameEvent>(db)
}

/// True if a game's events are sequentially incomplete: a question number is missing
/// (they must run 1..=max) or a question is missing a sub-event (eventnum must run 0..=max).
pub fn events_have_gaps(events: &[GameEvent]) -> bool {
    use std::collections::{BTreeMap, HashSet};

    if events.is_empty() {
        return false;
    }

    // Question numbers must be contiguous from 1 through the maximum.
    let mut questions: Vec<i32> = events.iter().map(|e| e.question).collect();
    questions.sort_unstable();
    questions.dedup();
    for (idx, q) in questions.iter().enumerate() {
        if *q != (idx as i32 + 1) {
            return true;
        }
    }

    // Within each question, event numbers must run 0 through the maximum with no gaps.
    let mut by_question: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    for e in events {
        by_question.entry(e.question).or_default().push(e.eventnum);
    }
    for (_question, eventnums) in &by_question {
        let set: HashSet<i32> = eventnums.iter().copied().collect();
        let max = eventnums.iter().copied().max().unwrap_or(0);
        for n in 0..=max {
            if !set.contains(&n) {
                return true;
            }
        }
    }

    false
}

pub fn create_update_game_event(db: &mut database::Connection, item: &NewGameEvent) -> QueryResult<GameEvent> {
    use crate::schema::gameevents::dsl::*;

    if exists(db, item.gid, item.question, item.eventnum) {
        log::debug!("GameEvent (gid={}, question={}, eventnum={}) exists — updating", item.gid, item.question, item.eventnum);
        diesel::update(
            gameevents
                .filter(gid.eq(item.gid))
                .filter(question.eq(item.question))
                .filter(eventnum.eq(item.eventnum))
        )
        .set(&GameEventChangeset::from_newgameevent(item))
        .get_result::<GameEvent>(db)
    } else {
        log::debug!("GameEvent (gid={}, question={}, eventnum={}) not found — inserting", item.gid, item.question, item.eventnum);
        insert_into(gameevents).values(item).get_result::<GameEvent>(db)
    }
}

// Not including a Delete fn until it is apparent that it is needed.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_event_calculation_scenario_one_normal_quiz_works() {

        // Scenario 1: Normal quiz (with as many miscellaneous scenarios included in it as possible)
        
        // ARRANGE:

        let game_id = Uuid::new_v4();

        let seat_one = 0;
        let seat_two = 1;
        let seat_three = 2;

        let left_team = 0;
        let center_team = 1;

        let jacob = ("Jacob", left_team);
        let taran = ("Taran", left_team);
        let lily = ("Lily", left_team);

        let audrey = ("Audrey", center_team);
        let jesse = ("Jesse", center_team);
        let kenzie = ("Kenzie", center_team);

        let (game_events, _) = GameEventStreamBuilder::new(game_id)
            .then_add_RM("Tournament")
            .then_add_QT("N")
            
            .then_add_TN("Red Team", left_team).unwrap()
            .then_add_QN_plus_if_SC_or_SS(jacob.0, jacob.1, seat_one, true, false).unwrap()
            .then_add_QN_plus_if_SC_or_SS(taran.0, taran.1, seat_two, false, true).unwrap()
            .then_add_QN_plus_if_SC_or_SS(lily.0, lily.1, seat_three, false, false).unwrap()
             
            .then_add_TN("Blue Team", center_team).unwrap()
            .then_add_QN_plus_if_SC_or_SS(audrey.0, audrey.1, seat_one, true, false).unwrap()
            .then_add_QN_plus_if_SC_or_SS(jesse.0, jesse.1, seat_two, false, true).unwrap()
            .then_add_QN_plus_if_SC_or_SS(kenzie.0, kenzie.1, seat_three, false, false).unwrap()
            
            .then_add_TC(audrey.0, audrey.1).unwrap()
            // Red Team: 0 { total_fouls: 0, jacob: 0/0, taran: 0/0, lily: 0/0 }, Blue Team: 20 { total_fouls: 0, audrey: 1/0, jesse: 0/0, kenzie: 0/0 }
            .then_add_TC(jacob.0, jacob.1).unwrap()
            // Red Team: 20 { total_fouls: 0, jacob: 1/0, taran: 0/0, lily: 0/0 }, Blue Team: 20 { total_fouls: 0, audrey: 1/0, jesse: 0/0, kenzie: 0/0 }
            .then_add_TC(taran.0, taran.1).unwrap()
            // Red Team: 40 { total_fouls: 0, jacob: 1/0, taran: 1/0, lily: 0/0 }, Blue Team: 20 { total_fouls: 0, audrey: 1/0, jesse: 0/0, kenzie: 0/0 }
            .then_add_TC(jacob.0, jacob.1).unwrap()
            // Red Team: 60 { total_fouls: 0, jacob: 2/0, taran: 1/0, lily: 0/0 }, Blue Team: 20 { total_fouls: 0, audrey: 1/0, jesse: 0/0, kenzie: 0/0 }
            .then_add_TE_and_bonuses(kenzie.0, kenzie.1, true, true).unwrap()
            // Red Team: 70 { total_fouls: 0, jacob: 2/0, taran: 1/0, lily: 0/0 }, Blue Team: 20 { total_fouls: 0, audrey: 1/0, jesse: 0/0, kenzie: 0/1 }
            
            .then_add_TC(audrey.0, audrey.1).unwrap()
            // Red Team: 70 { total_fouls: 0, jacob: 2/0, taran: 1/0, lily: 0/0 }, Blue Team: 40 { total_fouls: 0, audrey: 2/0, jesse: 0/0, kenzie: 0/1 }
            .then_add_TE_and_bonuses(lily.0, lily.1, false, false).unwrap()
            // Red Team: 70 { total_fouls: 0, jacob: 2/0, taran: 1/0, lily: 0/1 }, Blue Team: 40 { total_fouls: 0, audrey: 2/0, jesse: 0/0, kenzie: 0/1 }
            .then_add_TC(taran.0, taran.1).unwrap()
            .then_add_Cminus(audrey.0, audrey.1).unwrap()
            .then_add_FC(true, left_team).unwrap()
            // Red Team: 90 { total_fouls: 1, jacob: 2/0, taran: 2/0, lily: 0/1 }, Blue Team: 40 { total_fouls: 0, audrey: 2/0, jesse: 0/0, kenzie: 0/1 }
            .then_add_TC(jacob.0, jacob.1).unwrap()
            // Red Team: 110 { total_fouls: 1, jacob: 3/0, taran: 2/0, lily: 0/1 }, Blue Team: 40 { total_fouls: 0, audrey: 2/0, jesse: 0/0, kenzie: 0/1 }
            .then_add_TC(audrey.0, audrey.1).unwrap()
            .then_add_TO(center_team).unwrap()
            // Red Team: 110 { total_fouls: 1, jacob: 3/0, taran: 2/0, lily: 0/1 }, Blue Team: 60 { total_fouls: 0, audrey: 3/0, jesse: 0/0, kenzie: 0/1 }
            
            .then_add_NJ().unwrap()
            .then_add_Fminus(jesse.0, jesse.1).unwrap()
            .then_add_TE_and_bonuses(jesse.0, jesse.1, false, false).unwrap()
            // Red Team: 110 { total_fouls: 1, jacob: 3/0, taran: 2/0, lily: 0/1 }, Blue Team: 60 { total_fouls: 1, audrey: 3/0, jesse: 0/1, kenzie: 0/1 }
            .then_add_TE_and_bonuses(kenzie.0, kenzie.1, false, false).unwrap()
            // Red Team: 110 { total_fouls: 1, jacob: 3/0, taran: 2/0, lily: 0/1 }, Blue Team: 60 { total_fouls: 1, audrey: 3/0, jesse: 0/1, kenzie: 0/2 }
            .then_add_TE_and_bonuses(jesse.0, jesse.1, false, false).unwrap()
            .then_add_TO(center_team).unwrap()
            // Red Team: 110 { total_fouls: 1, jacob: 3/0, taran: 2/0, lily: 0/1 }, Blue Team: 60 { total_fouls: 1, audrey: 3/0, jesse: 0/2, kenzie: 0/2 }
            .then_add_TC(lily.0, lily.1).unwrap()
            // Red Team: 140 { total_fouls: 1, jacob: 3/0, taran: 2/0, lily: 1/1 }, Blue Team: 60 { total_fouls: 1, audrey: 3/0, jesse: 0/2, kenzie: 0/2 }

            .then_add_TE_and_bonuses(audrey.0, audrey.1, false, false).unwrap()
            // Red Team: 140 { total_fouls: 1, jacob: 3/0, taran: 2/0, lily: 1/1 }, Blue Team: 50 { total_fouls: 1, audrey: 3/1, jesse: 0/2, kenzie: 0/2 }
            .then_add_TC(jacob.0, jacob.1).unwrap()
            .then_add_TO(left_team).unwrap()
            // Red Team: 170 { total_fouls: 1, jacob: 4/0, taran: 2/0, lily: 1/1 }, Blue Team: 50 { total_fouls: 1, audrey: 3/0, jesse: 0/2, kenzie: 0/2 }
            .then_add_TE_and_bonuses(audrey.0, audrey.1, false, false).unwrap()
            // Red Team: 170 { total_fouls: 1, jacob: 4/0, taran: 2/0, lily: 1/1 }, Blue Team: 40 { total_fouls: 1, audrey: 3/1, jesse: 0/2, kenzie: 0/2 }
            .then_challenge_accepted(audrey.0, audrey.1, false, false).unwrap()
            // Red Team: 170 { total_fouls: 1, jacob: 4/0, taran: 2/0, lily: 1/1 }, Blue Team: 70 { total_fouls: 1, audrey: 4/1, jesse: 0/2, kenzie: 0/2 }
            .then_add_TC(jesse.0, jesse.1).unwrap()
            // Red Team: 170 { total_fouls: 1, jacob: 4/0, taran: 2/0, lily: 1/1 }, Blue Team: 90 { total_fouls: 1, audrey: 4/1, jesse: 1/2, kenzie: 0/2 }
            .then_challenge_accepted(taran.0, taran.1, true, true).unwrap()
            // Red Team: 180 { total_fouls: 1, jacob: 4/0, taran: 2/0, lily: 1/1 }, Blue Team: 60 { total_fouls: 1, audrey: 4/1, jesse: 0/3, kenzie: 0/2 }
            .then_add_FC(false, center_team).unwrap()
            // Red Team: 180 { total_fouls: 1, jacob: 4/0, taran: 2/0, lily: 1/1 }, Blue Team: 50 { total_fouls: 2, audrey: 4/1, jesse: 0/3, kenzie: 0/2 }
            .then_add_TC(kenzie.0, kenzie.1).unwrap()
            // Red Team: 180 { total_fouls: 1, jacob: 4/0, taran: 2/0, lily: 1/1 }, Blue Team: 70 { total_fouls: 1, audrey: 4/1, jesse: 0/3, kenzie: 1/2 }

            .to_game_events();

        let mut counter = 0;
        for event in game_events.iter() {
            println!["{}: {:?}", counter, event];
            counter += 1;
        }



        // ACT:

        let calculated_game_events = GameEventCalculator::new(game_id, game_events)
            .calculate_current_game_scores_and_counts()
            .unwrap();
        
        // ASSERT:

        let check_left_team = calculated_game_events.teams[&(0)].clone();
        let check_center_team = calculated_game_events.teams[&(1)].clone();
        
        // Current Question is accurate:
        assert_eq![calculated_game_events.current_question, 21];

        // Team Names are accurate:
        assert_eq![check_left_team.name, "Red Team"];
        assert_eq![check_center_team.name, "Blue Team"];

        // Team Scores are accurate:
        assert_eq![check_left_team.score, 180];
        assert_eq![check_center_team.score, 70];

        // Team Timeouts are accurate:
        assert_eq![check_left_team.timeouts_taken, vec![18]];
        assert_eq![check_center_team.timeouts_taken, vec![11, 15]];

        // Team Challenges are accurate (what if this was a Vec of objects showing what question and if it was accepted or overruled?):
        assert_eq![check_left_team.overruled_challenges, Vec::<i32>::new()];
        assert_eq![check_center_team.overruled_challenges, vec![8]];  // question #
        // Team Fouls is accurate
        assert_eq![check_left_team.team_and_coach_fouls_received, vec![(9,true)]];  // ((question), (is_coach))
        assert_eq![check_center_team.team_and_coach_fouls_received, vec![(20,false)]];

        // Team Captain and CoCaptains (*for current question!) are correct:
        assert_eq![check_left_team.captain, (0,false)];  // ((seat number), (is currently operational captain))
        assert_eq![check_left_team.cocaptain, (1,true)];
        assert_eq![check_center_team.captain, (0,false)];
        assert_eq![check_center_team.cocaptain, (1,false)];

        // Quizzer Toss-up counts (TCs && TEs), Bonus counts (BCs && BEs), QOs and EOs are accurate and quizzer fouls are accurate:
        // 0 Jacob
        let check_jacob = check_left_team.quizzers[&(0)].clone();
        assert_eq![check_jacob.correct_tossups, vec![2, 4, 9, 17]];
        assert_eq![check_jacob.errors_on_tossups, Vec::<i32>::new()];
        assert_eq![check_jacob.question_quizzed_out_on, 17];
        assert_eq![check_jacob.question_errored_out_on, -1];
        assert_eq![check_jacob.question_fouled_out_on, -1];
        assert_eq![check_jacob.fouls_received, Vec::<i32>::new()];
        assert_eq![check_jacob.correct_bonuses, Vec::<i32>::new()];
        assert_eq![check_jacob.errors_on_bonuses, vec![16]];
        // 1 Taran
        let check_taran = check_left_team.quizzers[&(1)].clone();
        assert_eq![check_taran.correct_tossups, vec![3, 8]];
        assert_eq![check_taran.errors_on_tossups, Vec::<i32>::new()];
        assert_eq![check_taran.question_quizzed_out_on, -1];
        assert_eq![check_taran.question_errored_out_on, -1];
        assert_eq![check_taran.question_fouled_out_on, -1];
        assert_eq![check_taran.fouls_received, Vec::<i32>::new()];
        assert_eq![check_taran.correct_bonuses, vec![19]];
        assert_eq![check_taran.errors_on_bonuses, vec![12, 14]];
        // 2 Lily
        let check_lily = check_left_team.quizzers[&(2)].clone();
        assert_eq![check_lily.correct_tossups, vec![15]];
        assert_eq![check_lily.errors_on_tossups, vec![7]];
        assert_eq![check_lily.question_quizzed_out_on, -1];
        assert_eq![check_lily.question_errored_out_on, -1];
        assert_eq![check_lily.question_fouled_out_on, -1];
        assert_eq![check_lily.fouls_received, Vec::<i32>::new()];
        assert_eq![check_lily.correct_bonuses, vec![5]];
        assert_eq![check_lily.errors_on_bonuses, vec![13]];
        // 0 Audrey
        let check_audrey = check_center_team.quizzers[&(0)].clone();
        assert_eq![check_audrey.correct_tossups, vec![1, 6, 10, 18]];
        assert_eq![check_audrey.errors_on_tossups, vec![16]];
        assert_eq![check_audrey.question_quizzed_out_on, 18];
        assert_eq![check_audrey.question_errored_out_on, -1];
        assert_eq![check_audrey.question_fouled_out_on, -1];
        assert_eq![check_audrey.fouls_received, Vec::<i32>::new()];
        assert_eq![check_audrey.correct_bonuses, Vec::<i32>::new()];
        assert_eq![check_audrey.errors_on_bonuses, Vec::<i32>::new()];
        // 1 Jesse
        let check_jesse = check_center_team.quizzers[&(1)].clone();
        assert_eq![check_jesse.correct_tossups, Vec::<i32>::new()];
        assert_eq![check_jesse.errors_on_tossups, vec![12, 14, 19]];
        assert_eq![check_jesse.question_quizzed_out_on, -1];
        assert_eq![check_jesse.question_errored_out_on, 19];
        assert_eq![check_jesse.question_fouled_out_on, -1];
        assert_eq![check_jesse.fouls_received, vec![12]];
        assert_eq![check_jesse.correct_bonuses, Vec::<i32>::new()];
        assert_eq![check_jesse.errors_on_bonuses, Vec::<i32>::new()];
        // 2 Kenzie
        let check_kenzie = check_center_team.quizzers[&(2)].clone();
        assert_eq![check_kenzie.correct_tossups, vec![20]];
        assert_eq![check_kenzie.errors_on_tossups, vec![5, 13]];
        assert_eq![check_kenzie.question_quizzed_out_on, -1];
        assert_eq![check_kenzie.question_errored_out_on, -1];
        assert_eq![check_kenzie.question_fouled_out_on, -1];
        assert_eq![check_kenzie.fouls_received, Vec::<i32>::new()];
        assert_eq![check_kenzie.correct_bonuses, Vec::<i32>::new()];
        assert_eq![check_kenzie.errors_on_bonuses, vec![7]];
    }

    #[test]
    fn game_event_calculation_scenario_two_tiebreaker_works() {

        // Scenario 2: Tie Breaker (with multiple OT questions)
        // Two variations:
        // (1) TC ends the Game and TC quizzer's team wins
        // (2) TE ends the Game TE quizzer's team loses

        // ARRANGE:

        let game_id = Uuid::new_v4();

        let seat_one = 0;

        let left_team = 0;
        let center_team = 1;

        let jacob = ("Jacob", left_team);

        let audrey = ("Audrey", center_team);

        let base_game_event_stream_builder = GameEventStreamBuilder::new(game_id)
            .then_add_RM("Tournament")
            .then_add_QT("N")
            
            .then_add_TN("Red Team", left_team).unwrap()
            .then_add_QN_plus_if_SC_or_SS(jacob.0, jacob.1, seat_one, true, false).unwrap()
             
            .then_add_TN("Blue Team", center_team).unwrap()
            .then_add_QN_plus_if_SC_or_SS(audrey.0, audrey.1, seat_one, true, false).unwrap()
            
            .then_add_TC(audrey.0, audrey.1).unwrap()
            // Red Team: 0 { jacob: 0/0 }, Blue Team: 20 { audrey: 1/0 }
            .then_add_TC(jacob.0, jacob.1).unwrap()
            // Red Team: 20 { jacob: 1/0 }, Blue Team: 20 { audrey: 1/0 }
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()

            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()

            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()

            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            // Red Team: 20 { jacob: 1/0 }, Blue Team: 20 { audrey: 1/0 }

            // Sudden-death Tie breaker starts here:
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap();

        let (game_events_TC_win, _) = base_game_event_stream_builder.clone()
            .then_add_TC(audrey.0, audrey.1).unwrap()
            // Red Team: 20 { jacob: 1/0 }, Blue Team: 20 { audrey: 2/0 } - scores don't change but winner is determined; last TC doesn't count toward individual score
            .to_game_events();

        let (game_events_TE_loss, _) = base_game_event_stream_builder
            .then_add_TE_and_bonuses(audrey.0, audrey.1, false, false).unwrap()
            // Red Team: 20 { jacob: 1/0 }, Blue Team: 20 { audrey: 1/1 } - scores don't change but winner is determined; last TC doesn't count toward individual score
            .to_game_events();

        let mut counter = 0;
        for event in game_events_TC_win.iter() {
        // for event in game_events_TE_loss.iter() {
            println!["{}: {:?}", counter, event];
            counter += 1;
        }

        // ACT:

        let calculated_game_events_for_TC_win = GameEventCalculator::new(game_id, game_events_TC_win)
            .calculate_current_game_scores_and_counts()
            .unwrap();

        let calculated_game_events_for_TE_loss = GameEventCalculator::new(game_id, game_events_TE_loss)
            .calculate_current_game_scores_and_counts()
            .unwrap();
        
        // ASSERT:

        // Variation 1:

        let check_left_team_for_TC_win = calculated_game_events_for_TC_win.teams[&0].clone();
        let check_center_team_for_TC_win = calculated_game_events_for_TC_win.teams[&1].clone();
        
        // Current Question is accurate:
        assert_eq![calculated_game_events_for_TC_win.current_question, 24];

        // Team Names are accurate (sanity check):
        assert_eq![check_left_team_for_TC_win.name, "Red Team"];
        assert_eq![check_center_team_for_TC_win.name, "Blue Team"];

        // Team Scores are accurate:
        assert_eq![check_left_team_for_TC_win.score, 20];
        assert_eq![check_center_team_for_TC_win.score, 20];

        // Ending positions for each team:
        assert_eq![check_left_team_for_TC_win.rank, 2];
        assert_eq![check_center_team_for_TC_win.rank, 1];

        // Variation 2:

        let check_left_team_for_TE_loss = calculated_game_events_for_TE_loss.teams[&0].clone();
        let check_center_team_for_TE_loss = calculated_game_events_for_TE_loss.teams[&1].clone();
        
        // Current Question is accurate:
        assert_eq![calculated_game_events_for_TE_loss.current_question, 24];

        // Team Names are accurate (sanity check):
        assert_eq![check_left_team_for_TE_loss.name, "Red Team"];
        assert_eq![check_center_team_for_TE_loss.name, "Blue Team"];

        // Team Scores are accurate:
        assert_eq![check_left_team_for_TE_loss.score, 20];
        assert_eq![check_center_team_for_TE_loss.score, 20];

        // Ending positions for each team:
        assert_eq![check_left_team_for_TE_loss.rank, 1];
        assert_eq![check_center_team_for_TE_loss.rank, 2];
    }

    // #[test]
    // fn game_event_calculation_scenario_three_works() {

    //     // Scenario 3: All Quizzers become inelligible and there is a tie
    //     // NOTE: Currently QuizMachine does not handle this situation; it is a manual process to resolve the tie and there are no rules for this from NYI either

    //     // ARRANGE:
    //     // ACT:
    //     // ASSERT:
    // }

    #[test]
    fn game_event_calculation_scenario_four_captain_cocaptain_inelligibility_works() {

        // Scenario 4: The following eight scenarios modify data correctly:
        // A: Captain QO, then Cocaptain QO
        // B: Captain QO, then Cocaptain EO
        // C: Captain EO, then Cocaptain EO
        // D: Captain EO, then Cocaptain QO
        // E: Cocaptain QO, then Captain QO
        // F: Cocaptain QO, then Captain EO
        // G: Cocaptain EO, then Captain EO
        // H: Cocaptain EO, then Captain QO
        // I: Captain QO
        // J: Captain EO
        // K: Cocaptain QO
        // L: Cocaptain EO

        // ARRANGE:

        let game_id = Uuid::new_v4();

        let seat_one = 0;
        let seat_two = 1;

        let left_team = 0;
        let center_team = 1;

        // using these two as cap and cocap:
        let jacob = ("Jacob", left_team);
        let taran = ("Taran", left_team);

        let audrey = ("Audrey", center_team);
        let jesse = ("Jesse", center_team);

        let base_game_event_stream_builder = GameEventStreamBuilder::new(game_id)
        .then_add_RM("Tournament")
        .then_add_QT("N")
            
            .then_add_TN("Red Team", left_team).unwrap()
            .then_add_QN_plus_if_SC_or_SS(jacob.0, jacob.1, seat_one, true, false).unwrap()
            .then_add_QN_plus_if_SC_or_SS(taran.0, taran.1, seat_two, false, true).unwrap()
             
            .then_add_TN("Blue Team", center_team).unwrap()
            .then_add_QN_plus_if_SC_or_SS(audrey.0, audrey.1, seat_one, true, false).unwrap()
            .then_add_QN_plus_if_SC_or_SS(jesse.0, jesse.1, seat_two, false, true).unwrap()
            
            // 12 No Jumps as base (leaving 8 modifiable questions (max = 2 QOs = 8 questions)):
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap();
        
        // A: Captain QO, then Cocaptain QO
        let (game_events_cap_QO_then_cocap_QO, _) = base_game_event_stream_builder.clone()
            .then_add_TC(jacob.0, jacob.1).unwrap()
            .then_add_TC(jacob.0, jacob.1).unwrap()
            .then_add_TC(jacob.0, jacob.1).unwrap()
            .then_add_TC(jacob.0, jacob.1).unwrap()
            .then_add_TC(taran.0, taran.1).unwrap()
            .then_add_TC(taran.0, taran.1).unwrap()
            .then_add_TC(taran.0, taran.1).unwrap()
            .then_add_TC(taran.0, taran.1).unwrap()
            .to_game_events();
        
        // B: Captain QO, then Cocaptain EO
        let (game_events_cap_QO_then_cocap_EO, _) = base_game_event_stream_builder.clone()
            .then_add_TC(jacob.0, jacob.1).unwrap()
            .then_add_TC(jacob.0, jacob.1).unwrap()
            .then_add_TC(jacob.0, jacob.1).unwrap()
            .then_add_TC(jacob.0, jacob.1).unwrap()
            .then_add_TE_and_bonuses(taran.0, taran.1, false, false).unwrap()
            .then_add_TE_and_bonuses(taran.0, taran.1, false, false).unwrap()
            .then_add_TE_and_bonuses(taran.0, taran.1, false, false).unwrap()
            .then_add_NJ().unwrap()
            .to_game_events();
        
        // C: Captain EO, then Cocaptain EO
        let (game_events_cap_EO_then_cocap_EO, _) = base_game_event_stream_builder.clone()
            .then_add_TE_and_bonuses(jacob.0, jacob.1, false, false).unwrap()
            .then_add_TE_and_bonuses(jacob.0, jacob.1, false, false).unwrap()
            .then_add_TE_and_bonuses(jacob.0, jacob.1, false, false).unwrap()
            .then_add_TE_and_bonuses(taran.0, taran.1, false, false).unwrap()
            .then_add_TE_and_bonuses(taran.0, taran.1, false, false).unwrap()
            .then_add_TE_and_bonuses(taran.0, taran.1, false, false).unwrap()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .to_game_events();
        
        // D: Captain EO, then Cocaptain QO
        let (game_events_cap_EO_then_cocap_QO, _) = base_game_event_stream_builder.clone()
            .then_add_TE_and_bonuses(jacob.0, jacob.1, false, false).unwrap()
            .then_add_TE_and_bonuses(jacob.0, jacob.1, false, false).unwrap()
            .then_add_TE_and_bonuses(jacob.0, jacob.1, false, false).unwrap()
            .then_add_TC(taran.0, taran.1).unwrap()
            .then_add_TC(taran.0, taran.1).unwrap()
            .then_add_TC(taran.0, taran.1).unwrap()
            .then_add_TC(taran.0, taran.1).unwrap()
            .then_add_NJ().unwrap()
            .to_game_events();
        
        // E: Cocaptain QO, then Captain QO
        let (game_events_cocap_QO_then_cap_QO, _) = base_game_event_stream_builder.clone()
            .then_add_TC(taran.0, taran.1).unwrap()
            .then_add_TC(taran.0, taran.1).unwrap()
            .then_add_TC(taran.0, taran.1).unwrap()
            .then_add_TC(taran.0, taran.1).unwrap()
            .then_add_TC(jacob.0, jacob.1).unwrap()
            .then_add_TC(jacob.0, jacob.1).unwrap()
            .then_add_TC(jacob.0, jacob.1).unwrap()
            .then_add_TC(jacob.0, jacob.1).unwrap()
            .to_game_events();
        
        // F: Cocaptain QO, then Captain EO
        let (game_events_cocap_QO_then_cap_EO, _) = base_game_event_stream_builder.clone()
            .then_add_TC(taran.0, taran.1).unwrap()
            .then_add_TC(taran.0, taran.1).unwrap()
            .then_add_TC(taran.0, taran.1).unwrap()
            .then_add_TC(taran.0, taran.1).unwrap()
            .then_add_TE_and_bonuses(jacob.0, jacob.1, false, false).unwrap()
            .then_add_TE_and_bonuses(jacob.0, jacob.1, false, false).unwrap()
            .then_add_TE_and_bonuses(jacob.0, jacob.1, false, false).unwrap()
            .then_add_NJ().unwrap()
            .to_game_events();
        
        // G: Cocaptain EO, then Captain EO
        let (game_events_cocap_EO_then_cap_EO, _) = base_game_event_stream_builder.clone()
            .then_add_TE_and_bonuses(taran.0, taran.1, false, false).unwrap()
            .then_add_TE_and_bonuses(taran.0, taran.1, false, false).unwrap()
            .then_add_TE_and_bonuses(taran.0, taran.1, false, false).unwrap()
            .then_add_TE_and_bonuses(jacob.0, jacob.1, false, false).unwrap()
            .then_add_TE_and_bonuses(jacob.0, jacob.1, false, false).unwrap()
            .then_add_TE_and_bonuses(jacob.0, jacob.1, false, false).unwrap()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .to_game_events();
        
        // H: Cocaptain EO, then Captain QO
        let (game_events_cocap_EO_then_cap_QO, _) = base_game_event_stream_builder.clone()
            .then_add_TE_and_bonuses(taran.0, taran.1, false, false).unwrap()
            .then_add_TE_and_bonuses(taran.0, taran.1, false, false).unwrap()
            .then_add_TE_and_bonuses(taran.0, taran.1, false, false).unwrap()
            .then_add_TC(jacob.0, jacob.1).unwrap()
            .then_add_TC(jacob.0, jacob.1).unwrap()
            .then_add_TC(jacob.0, jacob.1).unwrap()
            .then_add_TC(jacob.0, jacob.1).unwrap()
            .then_add_NJ().unwrap()
            .to_game_events();
        
        // I: Captain QO
        let (game_events_cap_QO, _) = base_game_event_stream_builder.clone()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_TC(jacob.0, jacob.1).unwrap()
            .then_add_TC(jacob.0, jacob.1).unwrap()
            .then_add_TC(jacob.0, jacob.1).unwrap()
            .then_add_TC(jacob.0, jacob.1).unwrap()
            .to_game_events();
        
        // J: Captain EO
        let (game_events_cap_EO, _) = base_game_event_stream_builder.clone()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_TE_and_bonuses(jacob.0, jacob.1, false, false).unwrap()
            .then_add_TE_and_bonuses(jacob.0, jacob.1, false, false).unwrap()
            .then_add_TE_and_bonuses(jacob.0, jacob.1, false, false).unwrap()
            .to_game_events();
        
        // K: Cocaptain QO
        let (game_events_cocap_QO, _) = base_game_event_stream_builder.clone()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_TC(taran.0, taran.1).unwrap()
            .then_add_TC(taran.0, taran.1).unwrap()
            .then_add_TC(taran.0, taran.1).unwrap()
            .then_add_TC(taran.0, taran.1).unwrap()
            .to_game_events();
        
        // L: Cocaptain EO
        let (game_events_cocap_EO, _) = base_game_event_stream_builder.clone()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_NJ().unwrap()
            .then_add_TE_and_bonuses(taran.0, taran.1, false, false).unwrap()
            .then_add_TE_and_bonuses(taran.0, taran.1, false, false).unwrap()
            .then_add_TE_and_bonuses(taran.0, taran.1, false, false).unwrap()
            .to_game_events();

        // let mut counter = 0;
        // for event in game_events.iter() {
        //     println!["{}: {:?}", counter, event];
        //     counter += 1;
        // }

        // ACT:

        let calculated_game_events_A = GameEventCalculator::new(game_id, game_events_cap_QO_then_cocap_QO)
            .calculate_current_game_scores_and_counts()
            .unwrap();

        let calculated_game_events_B = GameEventCalculator::new(game_id, game_events_cap_QO_then_cocap_EO)
            .calculate_current_game_scores_and_counts()
            .unwrap();

        let calculated_game_events_C = GameEventCalculator::new(game_id, game_events_cap_EO_then_cocap_EO)
            .calculate_current_game_scores_and_counts()
            .unwrap();

        let calculated_game_events_D = GameEventCalculator::new(game_id, game_events_cap_EO_then_cocap_QO)
            .calculate_current_game_scores_and_counts()
            .unwrap();

        let calculated_game_events_E = GameEventCalculator::new(game_id, game_events_cocap_QO_then_cap_QO)
            .calculate_current_game_scores_and_counts()
            .unwrap();

        let calculated_game_events_F = GameEventCalculator::new(game_id, game_events_cocap_QO_then_cap_EO)
            .calculate_current_game_scores_and_counts()
            .unwrap();

        let calculated_game_events_G = GameEventCalculator::new(game_id, game_events_cocap_EO_then_cap_EO)
            .calculate_current_game_scores_and_counts()
            .unwrap();

        let calculated_game_events_H = GameEventCalculator::new(game_id, game_events_cocap_EO_then_cap_QO)
            .calculate_current_game_scores_and_counts()
            .unwrap();

        let calculated_game_events_I = GameEventCalculator::new(game_id, game_events_cap_QO)
            .calculate_current_game_scores_and_counts()
            .unwrap();

        let calculated_game_events_J = GameEventCalculator::new(game_id, game_events_cap_EO)
            .calculate_current_game_scores_and_counts()
            .unwrap();

        let calculated_game_events_K = GameEventCalculator::new(game_id, game_events_cocap_QO)
            .calculate_current_game_scores_and_counts()
            .unwrap();

        let calculated_game_events_L = GameEventCalculator::new(game_id, game_events_cocap_EO)
            .calculate_current_game_scores_and_counts()
            .unwrap();
        
        // ASSERT:

        // A: Captain QO, then Cocaptain QO
        let check_left_team = calculated_game_events_A.teams[&(0)].clone();
        let check_jacob = check_left_team.quizzers[&(0)].clone();
        let check_taran = check_left_team.quizzers[&(1)].clone();

        assert_eq![check_left_team.captain, (0,false)];  // ((seat number), (is currently operational captain))
        assert_eq![check_left_team.cocaptain, (1,false)];
        assert_eq![check_jacob.question_quizzed_out_on, 16];
        assert_eq![check_jacob.question_errored_out_on, -1];
        assert_eq![check_taran.question_quizzed_out_on, 20];
        assert_eq![check_taran.question_errored_out_on, -1];

        // B: Captain QO, then Cocaptain EO
        let check_left_team = calculated_game_events_B.teams[&(0)].clone();
        let check_jacob = check_left_team.quizzers[&(0)].clone();
        let check_taran = check_left_team.quizzers[&(1)].clone();

        assert_eq![check_left_team.captain, (0,false)];  // ((seat number), (is currently operational captain))
        assert_eq![check_left_team.cocaptain, (1,false)];
        assert_eq![check_jacob.question_quizzed_out_on, 16];
        assert_eq![check_jacob.question_errored_out_on, -1];
        assert_eq![check_taran.question_quizzed_out_on, -1];
        assert_eq![check_taran.question_errored_out_on, 19];

        // C: Captain EO, then Cocaptain EO
        let check_left_team = calculated_game_events_C.teams[&(0)].clone();
        let check_jacob = check_left_team.quizzers[&(0)].clone();
        let check_taran = check_left_team.quizzers[&(1)].clone();

        assert_eq![check_left_team.captain, (0,false)];  // ((seat number), (is currently operational captain))
        assert_eq![check_left_team.cocaptain, (1,false)];
        assert_eq![check_jacob.question_quizzed_out_on, -1];
        assert_eq![check_jacob.question_errored_out_on, 15];
        assert_eq![check_taran.question_quizzed_out_on, -1];
        assert_eq![check_taran.question_errored_out_on, 18];

        // D: Captain EO, then Cocaptain QO
        let check_left_team = calculated_game_events_D.teams[&(0)].clone();
        let check_jacob = check_left_team.quizzers[&(0)].clone();
        let check_taran = check_left_team.quizzers[&(1)].clone();

        assert_eq![check_left_team.captain, (0,false)];  // ((seat number), (is currently operational captain))
        assert_eq![check_left_team.cocaptain, (1,false)];
        assert_eq![check_jacob.question_quizzed_out_on, -1];
        assert_eq![check_jacob.question_errored_out_on, 15];
        assert_eq![check_taran.question_quizzed_out_on, 19];
        assert_eq![check_taran.question_errored_out_on, -1];

        // E: Cocaptain QO, then Captain QO
        let check_left_team = calculated_game_events_E.teams[&(0)].clone();
        let check_jacob = check_left_team.quizzers[&(0)].clone();
        let check_taran = check_left_team.quizzers[&(1)].clone();

        assert_eq![check_left_team.captain, (0,false)];  // ((seat number), (is currently operational captain))
        assert_eq![check_left_team.cocaptain, (1,false)];
        assert_eq![check_jacob.question_quizzed_out_on, 20];
        assert_eq![check_jacob.question_errored_out_on, -1];
        assert_eq![check_taran.question_quizzed_out_on, 16];
        assert_eq![check_taran.question_errored_out_on, -1];

        // F: Cocaptain QO, then Captain EO
        let check_left_team = calculated_game_events_F.teams[&(0)].clone();
        let check_jacob = check_left_team.quizzers[&(0)].clone();
        let check_taran = check_left_team.quizzers[&(1)].clone();

        assert_eq![check_left_team.captain, (0,false)];  // ((seat number), (is currently operational captain))
        assert_eq![check_left_team.cocaptain, (1,false)];
        assert_eq![check_jacob.question_quizzed_out_on, -1];
        assert_eq![check_jacob.question_errored_out_on, 19];
        assert_eq![check_taran.question_quizzed_out_on, 16];
        assert_eq![check_taran.question_errored_out_on, -1];

        // G: Cocaptain EO, then Captain EO
        let check_left_team = calculated_game_events_G.teams[&(0)].clone();
        let check_jacob = check_left_team.quizzers[&(0)].clone();
        let check_taran = check_left_team.quizzers[&(1)].clone();

        assert_eq![check_left_team.captain, (0,false)];  // ((seat number), (is currently operational captain))
        assert_eq![check_left_team.cocaptain, (1,false)];
        assert_eq![check_jacob.question_quizzed_out_on, -1];
        assert_eq![check_jacob.question_errored_out_on, 18];
        assert_eq![check_taran.question_quizzed_out_on, -1];
        assert_eq![check_taran.question_errored_out_on, 15];

        // H: Cocaptain EO, then Captain QO
        let check_left_team = calculated_game_events_H.teams[&(0)].clone();
        let check_jacob = check_left_team.quizzers[&(0)].clone();
        let check_taran = check_left_team.quizzers[&(1)].clone();

        assert_eq![check_left_team.captain, (0,false)];  // ((seat number), (is currently operational captain))
        assert_eq![check_left_team.cocaptain, (1,false)];
        assert_eq![check_jacob.question_quizzed_out_on, 19];
        assert_eq![check_jacob.question_errored_out_on, -1];
        assert_eq![check_taran.question_quizzed_out_on, -1];
        assert_eq![check_taran.question_errored_out_on, 15];

        // I: Captain QO
        let check_left_team = calculated_game_events_I.teams[&(0)].clone();
        let check_jacob = check_left_team.quizzers[&(0)].clone();
        let check_taran = check_left_team.quizzers[&(1)].clone();

        assert_eq![check_left_team.captain, (0,false)];  // ((seat number), (is currently operational captain))
        assert_eq![check_left_team.cocaptain, (1,true)];
        assert_eq![check_jacob.question_quizzed_out_on, 20];
        assert_eq![check_jacob.question_errored_out_on, -1];
        assert_eq![check_taran.question_quizzed_out_on, -1];
        assert_eq![check_taran.question_errored_out_on, -1];

        // J: Captain EO
        let check_left_team = calculated_game_events_J.teams[&(0)].clone();
        let check_jacob = check_left_team.quizzers[&(0)].clone();
        let check_taran = check_left_team.quizzers[&(1)].clone();

        assert_eq![check_left_team.captain, (0,false)];  // ((seat number), (is currently operational captain))
        assert_eq![check_left_team.cocaptain, (1,true)];
        assert_eq![check_jacob.question_quizzed_out_on, -1];
        assert_eq![check_jacob.question_errored_out_on, 20];
        assert_eq![check_taran.question_quizzed_out_on, -1];
        assert_eq![check_taran.question_errored_out_on, -1];

        // K: Cocaptain QO
        let check_left_team = calculated_game_events_K.teams[&(0)].clone();
        let check_jacob = check_left_team.quizzers[&(0)].clone();
        let check_taran = check_left_team.quizzers[&(1)].clone();

        assert_eq![check_left_team.captain, (0,true)];  // ((seat number), (is currently operational captain))
        assert_eq![check_left_team.cocaptain, (1,false)];
        assert_eq![check_jacob.question_quizzed_out_on, -1];
        assert_eq![check_jacob.question_errored_out_on, -1];
        assert_eq![check_taran.question_quizzed_out_on, 20];
        assert_eq![check_taran.question_errored_out_on, -1];

        // L: Cocaptain EO
        let check_left_team = calculated_game_events_L.teams[&(0)].clone();
        let check_jacob = check_left_team.quizzers[&(0)].clone();
        let check_taran = check_left_team.quizzers[&(1)].clone();

        assert_eq![check_left_team.captain, (0,true)];  // ((seat number), (is currently operational captain))
        assert_eq![check_left_team.cocaptain, (1,false)];
        assert_eq![check_jacob.question_quizzed_out_on, -1];
        assert_eq![check_jacob.question_errored_out_on, -1];
        assert_eq![check_taran.question_quizzed_out_on, -1];
        assert_eq![check_taran.question_errored_out_on, 20];
    }
    #[test]
    fn game_event_calculation_scenario_five_removing_questions_works() {

        // Scenario 5: Questions are removed/fixed and events are correct

        // ARRANGE:

        let game_id = Uuid::new_v4();

        let seat_one = 0;
        let seat_two = 1;
        let seat_three = 2;

        let left_team = 0;
        let center_team = 1;

        let jacob = ("Jacob", left_team);
        let taran = ("Taran", left_team);
        let lily = ("Lily", left_team);

        let audrey = ("Audrey", center_team);
        let jesse = ("Jesse", center_team);
        let kenzie = ("Kenzie", center_team);

        let (game_events, _) = GameEventStreamBuilder::new(game_id)
            .then_add_RM("Tournament")
            .then_add_QT("N")
            
            .then_add_TN("Red Team", left_team).unwrap()
            .then_add_QN_plus_if_SC_or_SS(jacob.0, jacob.1, seat_one, true, false).unwrap()
            .then_add_QN_plus_if_SC_or_SS(taran.0, taran.1, seat_two, false, true).unwrap()
            .then_add_QN_plus_if_SC_or_SS(lily.0, lily.1, seat_three, false, false).unwrap()
             
            .then_add_TN("Blue Team", center_team).unwrap()
            .then_add_QN_plus_if_SC_or_SS(audrey.0, audrey.1, seat_one, true, false).unwrap()
            .then_add_QN_plus_if_SC_or_SS(jesse.0, jesse.1, seat_two, false, true).unwrap()
            .then_add_QN_plus_if_SC_or_SS(kenzie.0, kenzie.1, seat_three, false, false).unwrap()
            
            .then_add_TC(audrey.0, audrey.1).unwrap()
            // Red Team: 0 { total_fouls: 0, jacob: 0/0, taran: 0/0, lily: 0/0 }, Blue Team: 20 { total_fouls: 0, audrey: 1/0, jesse: 0/0, kenzie: 0/0 }
            .then_add_TC(jacob.0, jacob.1).unwrap()
            // Red Team: 20 { total_fouls: 0, jacob: 1/0, taran: 0/0, lily: 0/0 }, Blue Team: 20 { total_fouls: 0, audrey: 1/0, jesse: 0/0, kenzie: 0/0 }
            .then_add_TC(taran.0, taran.1).unwrap()
            // Red Team: 40 { total_fouls: 0, jacob: 1/0, taran: 1/0, lily: 0/0 }, Blue Team: 20 { total_fouls: 0, audrey: 1/0, jesse: 0/0, kenzie: 0/0 }
            .then_add_TC(jacob.0, jacob.1).unwrap()
            // Red Team: 60 { total_fouls: 0, jacob: 2/0, taran: 1/0, lily: 0/0 }, Blue Team: 20 { total_fouls: 0, audrey: 1/0, jesse: 0/0, kenzie: 0/0 }
            .then_add_TE_and_bonuses(kenzie.0, kenzie.1, true, true).unwrap()
            // Red Team: 70 { total_fouls: 0, jacob: 2/0, taran: 1/0, lily: 0/0 }, Blue Team: 20 { total_fouls: 0, audrey: 1/0, jesse: 0/0, kenzie: 0/1 }
            
            .then_remove_questions(2).unwrap()
            // Red Team: 0 { total_fouls: 0, jacob: 0/0, taran: 0/0, lily: 0/0 }, Blue Team: 20 { total_fouls: 0, audrey: 1/0, jesse: 0/0, kenzie: 0/0 }
            
            .then_add_TE_and_bonuses(lily.0, lily.1, false, false).unwrap()
            // Red Team: 0 { total_fouls: 0, jacob: 0/0, taran: 0/0, lily: 0/1 }, Blue Team: 20 { total_fouls: 0, audrey: 1/0, jesse: 0/0, kenzie: 0/0 }
            .then_add_TE_and_bonuses(lily.0, lily.1, false, false).unwrap()
            // Red Team: 0 { total_fouls: 0, jacob: 0/0, taran: 0/0, lily: 0/2 }, Blue Team: 20 { total_fouls: 0, audrey: 1/0, jesse: 0/0, kenzie: 0/0 }
            .then_add_TE_and_bonuses(lily.0, lily.1, false, false).unwrap()
            // Red Team: -10 { total_fouls: 0, jacob: 0/0, taran: 0/0, lily: 0/3 }, Blue Team: 20 { total_fouls: 0, audrey: 1/0, jesse: 0/0, kenzie: 0/0 }
            .then_add_TC(jacob.0, jacob.1).unwrap()
            // Red Team: 10 { total_fouls: 0, jacob: 1/0, taran: 0/0, lily: 0/3 }, Blue Team: 20 { total_fouls: 0, audrey: 1/0, jesse: 0/1, kenzie: 0/0 }
            .to_game_events();
            
        let mut counter = 0;
        for event in game_events.iter() {
            println!["{}: {:?}", counter, event];
            counter += 1;
        }

        // ACT:

        let calculated_game_events = GameEventCalculator::new(game_id, game_events)
            .calculate_current_game_scores_and_counts()
            .unwrap();
        
        // ASSERT:

        let check_left_team = calculated_game_events.teams[&(0)].clone();
        let check_center_team = calculated_game_events.teams[&(1)].clone();
        
        // Current Question is accurate:
        assert_eq![calculated_game_events.current_question, 6];

        // Team Scores are accurate:
        assert_eq![check_left_team.score, 10];
        assert_eq![check_center_team.score, 20];

        // Quizzer Toss-up counts (TCs && TEs), Bonus counts (BCs && BEs), QOs and EOs are accurate and quizzer fouls are accurate:
        // 0 Jacob
        let check_jacob = check_left_team.quizzers[&(0)].clone();
        assert_eq![check_jacob.correct_tossups, vec![5]];
        assert_eq![check_jacob.errors_on_tossups, Vec::<i32>::new()];
        assert_eq![check_jacob.question_quizzed_out_on, -1];
        assert_eq![check_jacob.question_errored_out_on, -1];
        assert_eq![check_jacob.question_fouled_out_on, -1];
        assert_eq![check_jacob.fouls_received, Vec::<i32>::new()];
        assert_eq![check_jacob.correct_bonuses, Vec::<i32>::new()];
        assert_eq![check_jacob.errors_on_bonuses, Vec::<i32>::new()];
        // 1 Taran
        let check_taran = check_left_team.quizzers[&(1)].clone();
        assert_eq![check_taran.correct_tossups, Vec::<i32>::new()];
        assert_eq![check_taran.errors_on_tossups, Vec::<i32>::new()];
        assert_eq![check_taran.question_quizzed_out_on, -1];
        assert_eq![check_taran.question_errored_out_on, -1];
        assert_eq![check_taran.question_fouled_out_on, -1];
        assert_eq![check_taran.fouls_received, Vec::<i32>::new()];
        assert_eq![check_taran.correct_bonuses, Vec::<i32>::new()];
        assert_eq![check_taran.errors_on_bonuses, Vec::<i32>::new()];
        // 2 Lily
        let check_lily = check_left_team.quizzers[&(2)].clone();
        assert_eq![check_lily.correct_tossups, Vec::<i32>::new()];
        assert_eq![check_lily.errors_on_tossups, vec![2, 3, 4]];
        assert_eq![check_lily.question_quizzed_out_on, -1];
        assert_eq![check_lily.question_errored_out_on, 4];
        assert_eq![check_lily.question_fouled_out_on, -1];
        assert_eq![check_lily.fouls_received, Vec::<i32>::new()];
        assert_eq![check_lily.correct_bonuses, Vec::<i32>::new()];
        assert_eq![check_lily.errors_on_bonuses, Vec::<i32>::new()];
        // 0 Audrey
        let check_audrey = check_center_team.quizzers[&(0)].clone();
        assert_eq![check_audrey.correct_tossups, vec![1]];
        assert_eq![check_audrey.errors_on_tossups, Vec::<i32>::new()];
        assert_eq![check_audrey.question_quizzed_out_on, -1];
        assert_eq![check_audrey.question_errored_out_on, -1];
        assert_eq![check_audrey.question_fouled_out_on, -1];
        assert_eq![check_audrey.fouls_received, Vec::<i32>::new()];
        assert_eq![check_audrey.correct_bonuses, Vec::<i32>::new()];
        assert_eq![check_audrey.errors_on_bonuses, Vec::<i32>::new()];
        // 1 Jesse
        let check_jesse = check_center_team.quizzers[&(1)].clone();
        assert_eq![check_jesse.correct_tossups, Vec::<i32>::new()];
        assert_eq![check_jesse.errors_on_tossups, Vec::<i32>::new()];
        assert_eq![check_jesse.question_quizzed_out_on, -1];
        assert_eq![check_jesse.question_errored_out_on, -1];
        assert_eq![check_jesse.question_fouled_out_on, -1];
        assert_eq![check_jesse.fouls_received, Vec::<i32>::new()];
        assert_eq![check_jesse.correct_bonuses, Vec::<i32>::new()];
        assert_eq![check_jesse.errors_on_bonuses, Vec::<i32>::new()];
        // 2 Kenzie
        let check_kenzie = check_center_team.quizzers[&(2)].clone();
        assert_eq![check_kenzie.correct_tossups, Vec::<i32>::new()];
        assert_eq![check_kenzie.errors_on_tossups, Vec::<i32>::new()];
        assert_eq![check_kenzie.question_quizzed_out_on, -1];
        assert_eq![check_kenzie.question_errored_out_on, -1];
        assert_eq![check_kenzie.question_fouled_out_on, -1];
        assert_eq![check_kenzie.fouls_received, Vec::<i32>::new()];
        assert_eq![check_kenzie.correct_bonuses, Vec::<i32>::new()];
        assert_eq![check_kenzie.errors_on_bonuses, vec![2, 3, 4]];
    }

    // Situation these tests don't cover:
    // - when both captain and cocaptain become inelligible and new ones need to be specified (needs to be bult into stream builder or else panic if next ruling happens before these are specified)
    //     currently if QuizMachine's captain and cocaptain both become inelligible, then when an appeal by their team is accepted the 'quizzer' of the game event = -1 and QuizMachine asks the quizmaster to specify captain and cocaptain; QuizMachine doesn't record replacement captain and cocaptains other than in-memory, so this cannot currently be checked/validated
    
    // Begin Validations unit tests:
    #[test]
    fn validation_check_RM_and_QT_events_are_present_in_game_event_stream_works() {
        // ARRANGE:

        let game_id = Uuid::new_v4();

        let seat_one = 0;

        let left_team = 0;
        let center_team = 1;

        let jacob = ("Jacob", left_team);

        let audrey = ("Audrey", center_team);

        let base_game_event_stream_builder = GameEventStreamBuilder::new(game_id)
            .then_add_TN("Red Team", left_team).unwrap()
            .then_add_QN_plus_if_SC_or_SS(jacob.0, jacob.1, seat_one, true, false).unwrap()
             
            .then_add_TN("Blue Team", center_team).unwrap()
            .then_add_QN_plus_if_SC_or_SS(audrey.0, audrey.1, seat_one, true, false).unwrap()
            
            .then_add_TC(audrey.0, audrey.1).unwrap()
            .then_add_TC(jacob.0, jacob.1).unwrap();


        let (game_events_with_RM, _) = base_game_event_stream_builder.clone()
            .then_add_RM("Tournament")
            .to_game_events();

        let (game_events_with_QT, _) = base_game_event_stream_builder.clone()
            .then_add_QT("N")
            .to_game_events();

        let (game_events_with_both_RM_and_QT, _) = base_game_event_stream_builder.clone()
            .then_add_RM("Tournament")
            .then_add_QT("N")
            .to_game_events();
        
        // ACT

        let only_RM_specific = GameEventStreamValidator::new(game_events_with_RM.clone())
            .check_for_has_RM_and_QT()
            .validate();
        let only_RM_everything_check = GameEventStreamValidator::new(game_events_with_RM)
            .check_for_everything()
            .validate();

        let only_QT_specific = GameEventStreamValidator::new(game_events_with_QT.clone())
            .check_for_has_RM_and_QT()
            .validate();
        let only_QT_everything_check = GameEventStreamValidator::new(game_events_with_QT)
            .check_for_everything()
            .validate();

        let both_RM_and_QT_specific = GameEventStreamValidator::new(game_events_with_both_RM_and_QT.clone())
            .check_for_has_RM_and_QT()
            .validate();
        let both_RM_and_QT_everything_check = GameEventStreamValidator::new(game_events_with_both_RM_and_QT)
            .check_for_everything()
            .validate();

        // ASSERT

        assert![only_RM_specific.is_err()];
        assert![only_RM_everything_check.is_err()];

        assert![only_QT_specific.is_err()];
        assert![only_QT_everything_check.is_err()];

        assert![both_RM_and_QT_specific.is_ok()];
        assert![both_RM_and_QT_everything_check.is_ok()];
    }

    #[test]
    fn validation_check_sort_order_works() {
        // ARRANGE:

        let game_id = Uuid::new_v4();

        let seat_one = 0;

        let left_team = 0;
        let center_team = 1;

        let jacob = ("Jacob", left_team);

        let audrey = ("Audrey", center_team);

        let (game_events, _) = GameEventStreamBuilder::new(game_id)
            .then_add_RM("Tournament")
            .then_add_QT("N")
            
            .then_add_TN("Red Team", left_team).unwrap()
            .then_add_QN_plus_if_SC_or_SS(jacob.0, jacob.1, seat_one, true, false).unwrap()
             
            .then_add_TN("Blue Team", center_team).unwrap()
            .then_add_QN_plus_if_SC_or_SS(audrey.0, audrey.1, seat_one, true, false).unwrap()
            
            .then_add_TC(audrey.0, audrey.1).unwrap()
            .then_add_TC(jacob.0, jacob.1).unwrap()
            .then_add_TE_and_bonuses(jacob.0, jacob.1, false, false).unwrap()
            .then_add_TE_and_bonuses(jacob.0, jacob.1, false, false).unwrap()
            .then_add_TC(jacob.0, jacob.1).unwrap()

            .then_add_TC(jacob.0, jacob.1).unwrap()
            .then_add_TC(audrey.0, audrey.1).unwrap()
            .then_add_TC(audrey.0, audrey.1).unwrap()
            .then_add_TE_and_bonuses(audrey.0, audrey.1, false, false).unwrap()
            .then_add_TC(audrey.0, audrey.1).unwrap()

            .to_game_events();
        
        let mut game_event_missing_eventnum: Vec<GameEvent> = game_events.clone();
        game_event_missing_eventnum.retain(|e| e.question != 3 || e.eventnum != 0);
        let game_event_missing_eventnum = game_event_missing_eventnum;

        let game_event_missing_question: Vec<GameEvent> = game_events
            .iter()
            .filter(|e| e.question != 2)
            .cloned()
            .collect();
        
        let mut game_event_eventnum_out_of_order = game_events.clone();
        for event in game_event_eventnum_out_of_order.iter_mut() {
            if event.question == 3 && event.eventnum == 0 {
                event.eventnum = 1;
                continue;
            }
            if event.question == 3 && event.eventnum == 1 {
                event.eventnum = 0;
                break;
            }
        }
        let game_event_eventnum_out_of_order = game_event_eventnum_out_of_order;

        let mut game_event_question_out_of_order = game_events.clone();
        for event in game_event_question_out_of_order.iter_mut() {
            if event.question == 6 {
                event.question = 7;
                continue;
            }
            if event.question == 7 {
                event.question = 6;
                break;
            }
        }
        let game_event_question_out_of_order = game_event_question_out_of_order;
        
        let mut game_event_question_eventnum_duplicate = game_events.clone();
        for event in game_event_question_eventnum_duplicate.iter_mut() {
            if event.question == 10 && event.eventnum == 0 {
                event.question = 1;
                break;
            }
        }
        let game_event_question_eventnum_duplicate = game_event_question_eventnum_duplicate;

        // for event in game_event_eventnum_out_of_order.clone() {
        //     println!["{:?}", event];
        // }
        
        // ACT

        // control group:
        let game_events_specific = GameEventStreamValidator::new(game_events.clone())
            .check_for_sort_order()
            .validate();
        let game_events_everything = GameEventStreamValidator::new(game_events.clone())
            .check_for_everything()
            .validate();


        let game_event_missing_eventnum_specific = GameEventStreamValidator::new(game_event_missing_eventnum.clone())
            .check_for_sort_order()
            .validate();
        let game_event_missing_eventnum_everything = GameEventStreamValidator::new(game_event_missing_eventnum.clone())
            .check_for_everything()
            .validate();

        let game_event_missing_question_specific = GameEventStreamValidator::new(game_event_missing_question.clone())
            .check_for_sort_order()
            .validate();
        let game_event_missing_question_everything = GameEventStreamValidator::new(game_event_missing_question.clone())
            .check_for_everything()
            .validate();

        let game_event_eventnum_out_of_order_specific = GameEventStreamValidator::new(game_event_eventnum_out_of_order.clone())
            .check_for_sort_order()
            .validate();
        let game_event_eventnum_out_of_order_everything = GameEventStreamValidator::new(game_event_eventnum_out_of_order.clone())
            .check_for_everything()
            .validate();

        let game_event_question_out_of_order_specific = GameEventStreamValidator::new(game_event_question_out_of_order.clone())
            .check_for_sort_order()
            .validate();
        let game_event_question_out_of_order_everything = GameEventStreamValidator::new(game_event_question_out_of_order.clone())
            .check_for_everything()
            .validate();

        let game_event_question_eventnum_duplicate_specific = GameEventStreamValidator::new(game_event_question_eventnum_duplicate.clone())
            .check_for_sort_order()
            .validate();
        let game_event_question_eventnum_duplicate_everything = GameEventStreamValidator::new(game_event_question_eventnum_duplicate.clone())
            .check_for_everything()
            .validate();

        // ASSERT

        // control:
        assert![game_events_specific.is_ok()];
        assert![game_events_everything.is_ok()];

        assert![game_event_missing_eventnum_specific.is_err()];
        assert![game_event_missing_eventnum_everything.is_err()];

        assert![game_event_missing_question_specific.is_err()];
        assert![game_event_missing_question_everything.is_err()];

        assert![game_event_eventnum_out_of_order_specific.is_ok()];
        assert![game_event_eventnum_out_of_order_everything.is_ok()];

        assert![game_event_question_out_of_order_specific.is_ok()];
        assert![game_event_question_out_of_order_everything.is_ok()];

        assert![game_event_question_eventnum_duplicate_specific.is_err()];
        assert![game_event_question_eventnum_duplicate_everything.is_err()];
    }

    #[test]
    fn validation_check_min_one_team_and_min_one_quizzer_per_team_works() {
        // ARRANGE:

        let game_id = Uuid::new_v4();

        let seat_one = 0;

        let left_team = 0;

        let jacob = ("Jacob", left_team);

        let base_game_events_stream = GameEventStreamBuilder::new(game_id)
            .then_add_RM("Tournament")
            .then_add_QT("N");

        let (game_events_no_team_or_quizzers, _) = base_game_events_stream.clone()
            .to_game_events();
            
        let (game_events_team_no_quizzers, _) = base_game_events_stream.clone()
            .then_add_TN("Red Team", left_team).unwrap()
            .to_game_events();

        let (game_events_team_and_quizzer, _) = base_game_events_stream.clone()
            .then_add_TN("Red Team", left_team).unwrap()
            .then_add_QN_plus_if_SC_or_SS(jacob.0, jacob.1, seat_one, true, false).unwrap()
            .to_game_events();

        for event in game_events_team_and_quizzer.clone() {
            println!["{:?}", event];
        }
        
        // ACT

        // control group:
        let game_events_team_and_quizzer_specific = GameEventStreamValidator::new(game_events_team_and_quizzer.clone())
            .check_for_min_one_team_and_min_one_quizzer_per_team()
            .validate();
        let game_events_team_and_quizzer_everything = GameEventStreamValidator::new(game_events_team_and_quizzer.clone())
            .check_for_everything()
            .validate();

        let game_events_no_team_or_quizzers_specific = GameEventStreamValidator::new(game_events_no_team_or_quizzers.clone())
            .check_for_min_one_team_and_min_one_quizzer_per_team()
            .validate();
        let game_events_no_team_or_quizzers_everything = GameEventStreamValidator::new(game_events_no_team_or_quizzers.clone())
            .check_for_everything()
            .validate();

        let game_events_team_no_quizzers_specific = GameEventStreamValidator::new(game_events_team_no_quizzers.clone())
            .check_for_min_one_team_and_min_one_quizzer_per_team()
            .validate();
        let game_events_team_no_quizzers_everything = GameEventStreamValidator::new(game_events_team_no_quizzers.clone())
            .check_for_everything()
            .validate();

        // ASSERT

        // control:
        assert![game_events_team_and_quizzer_specific.is_ok()];
        assert![game_events_team_and_quizzer_everything.is_ok()];

        assert![game_events_no_team_or_quizzers_specific.is_err()];
        assert![game_events_no_team_or_quizzers_everything.is_err()];

        assert![game_events_team_no_quizzers_specific.is_err()];
        assert![game_events_team_no_quizzers_everything.is_err()];
    }

    #[test]
    fn validation_check_captains_and_cocaptains_are_accurate_based_on_number_of_quizzers_on_team_works() {
        // ARRANGE:

        let game_id = Uuid::new_v4();

        let seat_one = 0;
        let seat_two = 1;
        let seat_three = 2;

        let left_team = 0;

        let jacob = ("Jacob", left_team);
        let taran: (&str, i32) = ("Taran", left_team);
        let lily: (&str, i32) = ("Lily", left_team);

        let base_game_events_stream = GameEventStreamBuilder::new(game_id)
            .then_add_RM("Tournament")
            .then_add_QT("N")
            .then_add_TN("Red Team", left_team).unwrap();

        let (game_events_has_captain_and_cocaptain, _) = base_game_events_stream.clone()
            .then_add_QN_plus_if_SC_or_SS(jacob.0, jacob.1, seat_one, true, false).unwrap()
            .then_add_QN_plus_if_SC_or_SS(taran.0, taran.1, seat_two, false, true).unwrap()
            .to_game_events();

        let (game_events_no_captain, _) = base_game_events_stream.clone()
            .then_add_QN_plus_if_SC_or_SS(jacob.0, jacob.1, seat_one, false, false).unwrap()
            .then_add_QN_plus_if_SC_or_SS(taran.0, taran.1, seat_two, false, true).unwrap()
            .to_game_events();

        let (game_events_no_cocaptain, _) = base_game_events_stream.clone()
            .then_add_QN_plus_if_SC_or_SS(jacob.0, jacob.1, seat_one, true, false).unwrap()
            .then_add_QN_plus_if_SC_or_SS(taran.0, taran.1, seat_two, false, false).unwrap()
            .to_game_events();

        let (game_events_same_captain_as_cocaptain, _) = base_game_events_stream.clone()
            .then_add_QN_plus_if_SC_or_SS(jacob.0, jacob.1, seat_one, true, false).unwrap()
            .then_add_QN_plus_if_SC_or_SS(jacob.0, jacob.1, seat_two, true, false).unwrap()
            .to_game_events();

        let (game_events_multiple_captains, _) = base_game_events_stream.clone()
            .then_add_QN_plus_if_SC_or_SS(jacob.0, jacob.1, seat_one, true, false).unwrap()
            .then_add_QN_plus_if_SC_or_SS(taran.0, taran.1, seat_two, true, false).unwrap()
            .then_add_QN_plus_if_SC_or_SS(lily.0, lily.1, seat_three, false, true).unwrap()
            .to_game_events();

        let (game_events_multiple_cocaptains, _) = base_game_events_stream.clone()
            .then_add_QN_plus_if_SC_or_SS(jacob.0, jacob.1, seat_one, true, false).unwrap()
            .then_add_QN_plus_if_SC_or_SS(taran.0, taran.1, seat_two, false, true).unwrap()
            .then_add_QN_plus_if_SC_or_SS(lily.0, lily.1, seat_three, false, true).unwrap()
            .to_game_events();

        let (mut game_events_captain_not_a_quizzer_on_team, _) = base_game_events_stream.clone()
            .then_add_QN_plus_if_SC_or_SS(jacob.0, jacob.1, seat_one, true, false).unwrap()
            .then_add_QN_plus_if_SC_or_SS(taran.0, taran.1, seat_two, false, true).unwrap()
            .to_game_events();
        game_events_captain_not_a_quizzer_on_team.iter_mut().for_each(|e| {
            if e.event == "SC".to_string() { e.name = "Sandy".to_string(); }
        });
        let game_events_captain_not_a_quizzer_on_team = game_events_captain_not_a_quizzer_on_team;

        let (mut game_events_cocaptain_not_a_quizzer_on_team, _) = base_game_events_stream.clone()
            .then_add_QN_plus_if_SC_or_SS(jacob.0, jacob.1, seat_one, true, false).unwrap()
            .then_add_QN_plus_if_SC_or_SS(taran.0, taran.1, seat_two, false, true).unwrap()
            .to_game_events();
        game_events_cocaptain_not_a_quizzer_on_team.iter_mut().for_each(|e| {
            if e.event == "SS".to_string() { e.name = "Sandy".to_string(); }
        });
        let game_events_cocaptain_not_a_quizzer_on_team = game_events_cocaptain_not_a_quizzer_on_team;

        for event in game_events_has_captain_and_cocaptain.clone() {
            println!["{:?}", event];
        }
        
        // ACT

        // control group:
        let game_events_has_captain_and_cocaptain_specific = GameEventStreamValidator::new(game_events_has_captain_and_cocaptain.clone())
            .check_for_captains_and_cocaptains_are_accurate_based_on_number_of_quizzers_on_team()
            .validate();
        let game_events_has_captain_and_cocaptain_everything = GameEventStreamValidator::new(game_events_has_captain_and_cocaptain.clone())
            .check_for_everything()
            .validate();

        let game_events_no_captain_specific = GameEventStreamValidator::new(game_events_no_captain.clone())
            .check_for_captains_and_cocaptains_are_accurate_based_on_number_of_quizzers_on_team()
            .validate();
        let game_events_no_captain_everything = GameEventStreamValidator::new(game_events_no_captain.clone())
            .check_for_everything()
            .validate();

        let game_events_no_cocaptain_specific = GameEventStreamValidator::new(game_events_no_cocaptain.clone())
            .check_for_captains_and_cocaptains_are_accurate_based_on_number_of_quizzers_on_team()
            .validate();
        let game_events_no_cocaptain_everything = GameEventStreamValidator::new(game_events_no_cocaptain.clone())
            .check_for_everything()
            .validate();

        let game_events_same_captain_as_cocaptain_specific = GameEventStreamValidator::new(game_events_same_captain_as_cocaptain.clone())
            .check_for_captains_and_cocaptains_are_accurate_based_on_number_of_quizzers_on_team()
            .validate();
        let game_events_same_captain_as_cocaptain_everything = GameEventStreamValidator::new(game_events_same_captain_as_cocaptain.clone())
            .check_for_everything()
            .validate();

        let game_events_multiple_captains_specific = GameEventStreamValidator::new(game_events_multiple_captains.clone())
            .check_for_captains_and_cocaptains_are_accurate_based_on_number_of_quizzers_on_team()
            .validate();
        let game_events_multiple_captains_everything = GameEventStreamValidator::new(game_events_multiple_captains.clone())
            .check_for_everything()
            .validate();

        let game_events_multiple_cocaptains_specific = GameEventStreamValidator::new(game_events_multiple_cocaptains.clone())
            .check_for_captains_and_cocaptains_are_accurate_based_on_number_of_quizzers_on_team()
            .validate();
        let game_events_multiple_cocaptains_everything = GameEventStreamValidator::new(game_events_multiple_cocaptains.clone())
            .check_for_everything()
            .validate();

        let game_events_captain_not_a_quizzer_on_team_specific = GameEventStreamValidator::new(game_events_captain_not_a_quizzer_on_team.clone())
            .check_for_captains_and_cocaptains_are_accurate_based_on_number_of_quizzers_on_team()
            .validate();
        let game_events_captain_not_a_quizzer_on_team_everything = GameEventStreamValidator::new(game_events_captain_not_a_quizzer_on_team.clone())
            .check_for_everything()
            .validate();

        let game_events_cocaptain_not_a_quizzer_on_team_specific = GameEventStreamValidator::new(game_events_cocaptain_not_a_quizzer_on_team.clone())
            .check_for_captains_and_cocaptains_are_accurate_based_on_number_of_quizzers_on_team()
            .validate();
        let game_events_cocaptain_not_a_quizzer_on_team_everything = GameEventStreamValidator::new(game_events_cocaptain_not_a_quizzer_on_team.clone())
            .check_for_everything()
            .validate();

        // ASSERT

        // control:
        assert![game_events_has_captain_and_cocaptain_specific.is_ok()];
        assert![game_events_has_captain_and_cocaptain_everything.is_ok()];

        assert![game_events_no_captain_specific.is_err()];
        assert![game_events_no_captain_everything.is_err()];

        assert![game_events_no_cocaptain_specific.is_err()];
        assert![game_events_no_cocaptain_everything.is_err()];

        assert![game_events_same_captain_as_cocaptain_specific.is_err()];
        assert![game_events_same_captain_as_cocaptain_everything.is_err()];

        assert![game_events_multiple_captains_specific.is_err()];
        assert![game_events_multiple_captains_everything.is_err()];

        assert![game_events_multiple_cocaptains_specific.is_err()];
        assert![game_events_multiple_cocaptains_everything.is_err()];

        assert![game_events_captain_not_a_quizzer_on_team_specific.is_err()];
        assert![game_events_captain_not_a_quizzer_on_team_everything.is_err()];

        assert![game_events_cocaptain_not_a_quizzer_on_team_specific.is_err()];
        assert![game_events_cocaptain_not_a_quizzer_on_team_everything.is_err()];
    }

    #[test]
    fn validation_check_no_team_name_duplicates() {
        // ARRANGE:

        let game_id = Uuid::new_v4();

        let seat_one = 0;

        let left_team = 0;
        let center_team = 1;
        let right_team = 2;

        let jacob: (&str, i32) = ("Jacob", left_team);
        let taran: (&str, i32) = ("Taran", center_team);
        let lily: (&str, i32) = ("Lily", right_team);

        let base_game_events_stream = GameEventStreamBuilder::new(game_id)
            .then_add_RM("Tournament")
            .then_add_QT("N");
        
        let (game_events_all_are_unique, _) = base_game_events_stream.clone()
            .then_add_TN("Red Team", left_team).unwrap()
            .then_add_QN_plus_if_SC_or_SS(jacob.0, jacob.1, seat_one, true, false).unwrap()
            .then_add_TN("Blue Team", center_team).unwrap()
            .then_add_QN_plus_if_SC_or_SS(taran.0, taran.1, seat_one, true, false).unwrap()
            .then_add_TN("Green Team", right_team).unwrap()
            .then_add_QN_plus_if_SC_or_SS(lily.0, lily.1, seat_one, true, false).unwrap()
            .to_game_events();
        
        let (game_events_left_and_center_match, _) = base_game_events_stream.clone()
            .then_add_TN("Red Team", left_team).unwrap()
            .then_add_QN_plus_if_SC_or_SS(jacob.0, jacob.1, seat_one, true, false).unwrap()
            .then_add_TN("Red Team", center_team).unwrap()
            .then_add_QN_plus_if_SC_or_SS(taran.0, taran.1, seat_one, true, false).unwrap()
            .then_add_TN("Blue Team", right_team).unwrap()
            .then_add_QN_plus_if_SC_or_SS(lily.0, lily.1, seat_one, true, false).unwrap()
            .to_game_events();
        
        let (game_events_center_and_right_match, _) = base_game_events_stream.clone()
            .then_add_TN("Blue Team", left_team).unwrap()
            .then_add_QN_plus_if_SC_or_SS(jacob.0, jacob.1, seat_one, true, false).unwrap()
            .then_add_TN("Red Team", center_team).unwrap()
            .then_add_QN_plus_if_SC_or_SS(taran.0, taran.1, seat_one, true, false).unwrap()
            .then_add_TN("Red Team", right_team).unwrap()
            .then_add_QN_plus_if_SC_or_SS(lily.0, lily.1, seat_one, true, false).unwrap()
            .to_game_events();
        
        let (game_events_left_and_right_match, _) = base_game_events_stream.clone()
            .then_add_TN("Blue Team", left_team).unwrap()
            .then_add_QN_plus_if_SC_or_SS(jacob.0, jacob.1, seat_one, true, false).unwrap()
            .then_add_TN("Red Team", center_team).unwrap()
            .then_add_QN_plus_if_SC_or_SS(taran.0, taran.1, seat_one, true, false).unwrap()
            .then_add_TN("Blue Team", right_team).unwrap()
            .then_add_QN_plus_if_SC_or_SS(lily.0, lily.1, seat_one, true, false).unwrap()
            .to_game_events();

        // for event in game_events_team_and_quizzer.clone() {
        //     println!["{:?}", event];
        // }
        
        // ACT

        // control group:
        let game_events_all_are_unique_specific = GameEventStreamValidator::new(game_events_all_are_unique.clone())
            .check_for_no_team_name_duplicates()
            .validate();
        let game_events_all_are_unique_everything = GameEventStreamValidator::new(game_events_all_are_unique.clone())
            .check_for_everything()
            .validate();

        let game_events_left_and_center_match_specific = GameEventStreamValidator::new(game_events_left_and_center_match.clone())
            .check_for_no_team_name_duplicates()
            .validate();
        let game_events_left_and_center_match_everything = GameEventStreamValidator::new(game_events_left_and_center_match.clone())
            .check_for_everything()
            .validate();

        let game_events_center_and_right_match_specific = GameEventStreamValidator::new(game_events_center_and_right_match.clone())
            .check_for_no_team_name_duplicates()
            .validate();
        let game_events_center_and_right_match_everything = GameEventStreamValidator::new(game_events_center_and_right_match.clone())
            .check_for_everything()
            .validate();

        let game_events_left_and_right_match_specific = GameEventStreamValidator::new(game_events_left_and_right_match.clone())
            .check_for_no_team_name_duplicates()
            .validate();
        let game_events_left_and_right_match_everything = GameEventStreamValidator::new(game_events_left_and_right_match.clone())
            .check_for_everything()
            .validate();

        // ASSERT

        // control:
        assert![game_events_all_are_unique_specific.is_ok()];
        assert![game_events_all_are_unique_everything.is_ok()];

        assert![game_events_left_and_center_match_specific.is_err()];
        assert![game_events_left_and_center_match_everything.is_err()];

        assert![game_events_center_and_right_match_specific.is_err()];
        assert![game_events_center_and_right_match_everything.is_err()];

        assert![game_events_left_and_right_match_specific.is_err()];
        assert![game_events_left_and_right_match_everything.is_err()];
    }

    #[test]
    fn validation_check_no_quizzer_name_duplicates_within_team() {
        // ARRANGE:

        let game_id = Uuid::new_v4();

        let seat_one = 0;
        let seat_two = 1;
        let seat_three = 2;

        let left_team = 0;

        let jacob: (&str, i32) = ("Jacob", left_team);
        let taran: (&str, i32) = ("Taran", left_team);
        let lily: (&str, i32) = ("Lily", left_team);

        let base_game_events_stream = GameEventStreamBuilder::new(game_id)
            .then_add_RM("Tournament")
            .then_add_QT("N")
            .then_add_TN("Red Team", left_team).unwrap();
        
        let (game_events_all_names_appear_once, _) = base_game_events_stream.clone()
            .then_add_QN_plus_if_SC_or_SS(jacob.0, jacob.1, seat_one, true, false).unwrap()
            .then_add_QN_plus_if_SC_or_SS(taran.0, taran.1, seat_two, false, true).unwrap()
            .then_add_QN_plus_if_SC_or_SS(lily.0, lily.1, seat_three, false, false).unwrap()
            .to_game_events();
        
        let (game_events_two_names_are_same, _) = base_game_events_stream.clone()
            .then_add_QN_plus_if_SC_or_SS(jacob.0, jacob.1, seat_one, true, false).unwrap()
            .then_add_QN_plus_if_SC_or_SS(taran.0, taran.1, seat_two, false, true).unwrap()
            .then_add_QN_plus_if_SC_or_SS(jacob.0, jacob.1, seat_three, false, false).unwrap()
            .to_game_events();

        for event in game_events_two_names_are_same.clone() {
            println!["{:?}", event];
        }
        
        // ACT

        // control group:
        let game_events_all_names_appear_once_specific = GameEventStreamValidator::new(game_events_all_names_appear_once.clone())
            .check_for_no_quizzer_name_duplicates_within_team()
            .validate();
        let game_events_all_names_appear_once_everything = GameEventStreamValidator::new(game_events_all_names_appear_once.clone())
            .check_for_everything()
            .validate();

        let game_events_two_names_are_same_specific = GameEventStreamValidator::new(game_events_two_names_are_same.clone())
            .check_for_no_quizzer_name_duplicates_within_team()
            .validate();
        let game_events_two_names_are_same_everything = GameEventStreamValidator::new(game_events_two_names_are_same.clone())
            .check_for_everything()
            .validate();

        // ASSERT

        // control:
        assert![game_events_all_names_appear_once_specific.is_ok()];
        assert![game_events_all_names_appear_once_everything.is_ok()];

        assert![game_events_two_names_are_same_specific.is_err()];
        assert![game_events_two_names_are_same_everything.is_err()];
    }

    /// The cache contract for `CalculatedGame`: it must survive a JSON round-trip with every
    /// projection byte-identical. If this ever breaks, a cached entry silently serves wrong
    /// stats to a live broadcast -- the worst possible failure mode for this feature.
    ///
    /// Specifically guards:
    ///   - `HashMap<i32, _>` keys (serde_json writes them as strings; they must parse back)
    ///   - `options`, which is per-game derived state (`IP`/`OP` events), not constants
    ///   - the two projections endpoints actually serve
    #[test]
    fn calculated_game_survives_a_json_round_trip_with_identical_projections() {
        // ARRANGE: smallest stream that still produces scores, ranks and quizzer counts.
        let game_id = Uuid::new_v4();
        let left_team = 0;
        let center_team = 1;
        let seat_one = 0;

        let (game_events, _) = GameEventStreamBuilder::new(game_id)
            .then_add_RM("Tournament")
            .then_add_QT("N")
            .then_add_TN("Red Team", left_team).unwrap()
            .then_add_QN_plus_if_SC_or_SS("Jacob", left_team, seat_one, true, false).unwrap()
            .then_add_TN("Blue Team", center_team).unwrap()
            .then_add_QN_plus_if_SC_or_SS("Audrey", center_team, seat_one, true, false).unwrap()
            .then_add_TC("Jacob", left_team).unwrap()
            .then_add_TC("Audrey", center_team).unwrap()
            .then_add_TC("Jacob", left_team).unwrap()
            .to_game_events();

        let fresh = calculate_game(game_id, game_events).expect("stream should score");

        // ACT: the exact trip a cached entry makes.
        let json = serde_json::to_string(&fresh).expect("CalculatedGame must serialize");
        let revived: CalculatedGame =
            serde_json::from_str(&json).expect("CalculatedGame must deserialize");

        // ASSERT: scalar + map-key fidelity.
        assert_eq![fresh.game_id, revived.game_id];
        assert_eq![fresh.current_question, revived.current_question];
        assert_eq![fresh.teams.len(), revived.teams.len()];
        for (team_idx, team) in fresh.teams.iter() {
            let revived_team = revived.teams.get(team_idx)
                .expect("integer team key must survive the round-trip");
            assert_eq![team.name, revived_team.name];
            assert_eq![team.score, revived_team.score];
            assert_eq![team.quizzers.len(), revived_team.quizzers.len()];
            for seat in team.quizzers.keys() {
                assert![revived_team.quizzers.contains_key(seat),
                    "integer quizzer seat key must survive the round-trip"];
            }
        }

        // Per-game options must survive; they drive every point award below.
        assert_eq![
            fresh.options.point_award_for_correct_tossup,
            revived.options.point_award_for_correct_tossup
        ];
        assert_eq![fresh.options.quiz_out, revived.options.quiz_out];
        assert_eq![fresh.options.is_tournament, revived.options.is_tournament];

        // ASSERT: the projections the endpoints serve are unchanged. HashMap iteration order
        // is nondeterministic, so sort before comparing.
        let mut fresh_teams = team_results_from_calculated(&fresh);
        let mut revived_teams = team_results_from_calculated(&revived);
        fresh_teams.sort_by_key(|r| r.team);
        revived_teams.sort_by_key(|r| r.team);
        assert_eq![fresh_teams, revived_teams,
            "team projection changed across a cache round-trip"];

        let sort_key = |r: &QuizzerGameResult| (r.team_name.clone(), r.name.clone());
        let mut fresh_quizzers = quizzer_results_from_calculated(&fresh);
        let mut revived_quizzers = quizzer_results_from_calculated(&revived);
        fresh_quizzers.sort_by_key(sort_key);
        revived_quizzers.sort_by_key(sort_key);
        assert_eq![fresh_quizzers, revived_quizzers,
            "quizzer projection changed across a cache round-trip"];

        // Sanity: the stream really did score, so the comparisons above were not vacuous.
        assert_eq![fresh_teams.len(), 2];
        assert![fresh_teams.iter().any(|r| r.score > 0), "expected a non-zero score"];
        assert![!fresh_quizzers.is_empty(), "expected at least one quizzer row"];

        // The cache key must carry the format version, so a bump orphans stale entries.
        assert![CalculatedGame::cache_key(game_id).contains(&CALC_VERSION.to_string())];
        assert![CalculatedGame::cache_key(game_id).contains(&game_id.to_string())];
    }

    /// Puts the real production payload through a real Valkey. Opt-in; see the matching test
    /// in `crate::cache` for the container one-liner.
    ///
    ///   VALKEY_TEST_URL=redis://127.0.0.1:6399 cargo test --lib -- --ignored --nocapture
    ///
    /// Also prints the payload sizes, which is the measurement behind the decision not to
    /// cache `GameEventCalculator` itself (it carries its own `game_events`).
    #[actix_web::test]
    #[ignore = "needs a running Valkey; set VALKEY_TEST_URL"]
    async fn a_calculated_game_round_trips_through_valkey() {
        let Ok(url) = std::env::var("VALKEY_TEST_URL") else {
            eprintln!("VALKEY_TEST_URL not set; skipping.");
            return;
        };
        let cache = crate::cache::Cache::connect(&url).await;
        assert![cache.is_enabled(), "expected to connect to Valkey at {url}"];

        // A stream with substitutions, fouls, bonuses and a quiz-out, so the payload exercises
        // every Vec and tuple field rather than just scores.
        let game_id = Uuid::new_v4();
        let (game_events, _) = GameEventStreamBuilder::new(game_id)
            .then_add_RM("Tournament")
            .then_add_QT("N")
            .then_add_TN("Red Team", 0).unwrap()
            .then_add_QN_plus_if_SC_or_SS("Jacob", 0, 0, true, false).unwrap()
            .then_add_QN_plus_if_SC_or_SS("Taran", 0, 1, false, true).unwrap()
            .then_add_TN("Blue Team", 1).unwrap()
            .then_add_QN_plus_if_SC_or_SS("Audrey", 1, 0, true, false).unwrap()
            .then_add_QN_plus_if_SC_or_SS("Jesse", 1, 1, false, true).unwrap()
            .then_add_TC("Jacob", 0).unwrap()
            .then_add_TE_and_bonuses("Audrey", 1, true, false).unwrap()
            .then_add_TC("Taran", 0).unwrap()
            .then_add_TO(0).unwrap()
            .then_add_TC("Jesse", 1).unwrap()
            .to_game_events();

        let event_count = game_events.len();
        let events_json = serde_json::to_string(&game_events).unwrap();
        let calculated = calculate_game(game_id, game_events).expect("stream should score");
        let state_json = serde_json::to_string(&calculated).unwrap();

        eprintln!(
            "payload sizes for {event_count} events: raw events {} bytes, calculated state \
             {} bytes ({:.1}x smaller)",
            events_json.len(),
            state_json.len(),
            events_json.len() as f64 / state_json.len() as f64
        );
        assert![
            state_json.len() < events_json.len(),
            "derived state ({} B) should be smaller than the events it came from ({} B)",
            state_json.len(),
            events_json.len()
        ];

        // Miss, then store, then hit.
        let key = CalculatedGame::cache_key(game_id);
        assert![cache.get_json::<CalculatedGame>(&key).await.is_none(), "expected a cold miss"];
        cache.set_json(&key, &calculated, CALCULATED_GAME_TTL).await;

        let revived: CalculatedGame = cache
            .get_json(&key)
            .await
            .expect("the entry just written should read back");

        // The projections the endpoints serve must be identical across the real round trip.
        let mut fresh_teams = team_results_from_calculated(&calculated);
        let mut revived_teams = team_results_from_calculated(&revived);
        fresh_teams.sort_by_key(|r| r.team);
        revived_teams.sort_by_key(|r| r.team);
        assert_eq![fresh_teams, revived_teams];

        let sort_key = |r: &QuizzerGameResult| (r.team_name.clone(), r.name.clone());
        let mut fresh_quizzers = quizzer_results_from_calculated(&calculated);
        let mut revived_quizzers = quizzer_results_from_calculated(&revived);
        fresh_quizzers.sort_by_key(sort_key);
        revived_quizzers.sort_by_key(sort_key);
        assert_eq![fresh_quizzers, revived_quizzers];

        // The detail that only `CalculatedGame` carries has to survive too -- it is the
        // reason for caching the richer object instead of the two projections.
        for (idx, team) in calculated.teams.iter() {
            let revived_team = revived.teams.get(idx).unwrap();
            assert_eq![team.timeouts_taken, revived_team.timeouts_taken];
            assert_eq![team.captain, revived_team.captain];
            assert_eq![team.cocaptain, revived_team.cocaptain];
            for (seat, quizzer) in team.quizzers.iter() {
                let revived_quizzer = revived_team.quizzers.get(seat).unwrap();
                assert_eq![quizzer.correct_tossups, revived_quizzer.correct_tossups];
                assert_eq![quizzer.errors_on_tossups, revived_quizzer.errors_on_tossups];
                assert_eq![quizzer.correct_bonuses, revived_quizzer.correct_bonuses];
            }
        }
        assert![
            calculated.teams.values().any(|t| !t.timeouts_taken.is_empty()),
            "the fixture should have recorded a timeout, or this test is vacuous"
        ];

        // What the write path does after an event lands.
        cache.invalidate(&[key.clone()]).await;
        assert![
            cache.get_json::<CalculatedGame>(&key).await.is_none(),
            "invalidation should have dropped the entry"
        ];
    }
}
