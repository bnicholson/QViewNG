//! Question types of a round's 20-question question set.
//!
//! A round stores its question types in `rounds.question_types` as exactly 20 single-letter codes,
//! one per question in question order, or as an empty string when none are saved. Storage is
//! all-or-nothing: a round never has types for only some of its questions.
//!
//! M (Memory Verse) is not stored. It is inferred from one each of R, V and Q.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// The number of questions in a round's question set.
pub const QUESTIONS_PER_ROUND: usize = 20;

/// Every storable question type code with its name and the number of questions of that type in a
/// standard question set. S and I share one slot: a standard set has exactly one of them.
pub const QUESTION_TYPES: [(char, &str, usize); 8] = [
    ('G', "General", 11),
    ('A', "According-to", 4),
    ('Q', "Quote", 1),
    ('R', "Reference", 1),
    ('V', "Verse", 1),
    ('X', "Context", 1),
    ('S', "Situation", 1),
    ('I', "In what Book and Chapter", 1),
];

pub fn is_storable_type(code: char) -> bool {
    QUESTION_TYPES.iter().any(|(c, _, _)| *c == code)
}

fn type_name(code: char) -> &'static str {
    QUESTION_TYPES.iter().find(|(c, _, _)| *c == code).map(|(_, n, _)| *n).unwrap_or("Unknown")
}

/// Checks a round's question types before they're saved. `types` is either empty (no question
/// types) or 20 storable codes. When `using_standard` is true the count of each type must match a
/// standard question set, which also means it can't hold both S and I questions.
pub fn validate(types: &str, using_standard: bool) -> Result<(), Vec<String>> {
    if types.is_empty() {
        return Ok(());
    }

    let codes: Vec<char> = types.chars().collect();
    let mut errors = Vec::new();

    if codes.len() != QUESTIONS_PER_ROUND {
        errors.push(format!(
            "Every one of the {} questions needs a question type (found {}).",
            QUESTIONS_PER_ROUND,
            codes.len()
        ));
    }
    for (i, code) in codes.iter().enumerate() {
        if *code == 'M' {
            errors.push(format!(
                "Question {}: M (Memory Verse) is inferred from R, V and Q and can't be saved as a type.",
                i + 1
            ));
        } else if !is_storable_type(*code) {
            errors.push(format!("Question {}: '{}' is not a question type.", i + 1, code));
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }

    let count = |code: char| codes.iter().filter(|c| **c == code).count();

    if using_standard {
        if count('S') > 0 && count('I') > 0 {
            errors.push(
                "A standard question set can't contain both S (Situation) and I (In what Book and Chapter) questions."
                    .to_string(),
            );
        }
        for (code, name, expected) in QUESTION_TYPES.iter().filter(|(c, _, _)| *c != 'S' && *c != 'I') {
            let found = count(*code);
            if found != *expected {
                errors.push(format!("Expected {} {} ({}) question(s), found {}.", expected, code, name, found));
            }
        }
        let situation_or_book = count('S') + count('I');
        if situation_or_book != 1 {
            errors.push(format!(
                "Expected 1 S (Situation) or I (In what Book and Chapter) question, found {}.",
                situation_or_book
            ));
        }
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

/// One question set read from an imported file.
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ParsedQuestionSet {
    /// The set's heading in the file, e.g. "Set #1 Experienced QView Questions".
    pub name: String,
    /// The set's 20 question type codes in question order, or empty when `errors` is not.
    pub question_types: String,
    /// Why the set's question types could not be read. Empty when they were.
    pub errors: Vec<String>,
}

/// What an imported question set file contained, in file order.
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ParsedQuestionSetFile {
    pub sets: Vec<ParsedQuestionSet>,
    /// Problems with the file as a whole, e.g. an unsupported format.
    pub errors: Vec<String>,
}

/// One question as read from a file, before it's checked.
struct RawQuestion {
    number: usize,
    code: String,
}

struct RawSet {
    name: String,
    questions: Vec<RawQuestion>,
}

/// Reads question types out of a question set file. Two formats are supported, both as exported
/// with a question set: RTF (each question starts "<type>\tab <number>.") and CSV (with "Set",
/// "Type" and "No." columns). The format comes from the file name's extension, else the content.
pub fn parse_question_set_file(filename: &str, content: &str) -> ParsedQuestionSetFile {
    let lower = filename.to_lowercase();
    let raw_sets = if lower.ends_with(".rtf") || (!lower.ends_with(".csv") && content.trim_start().starts_with("{\\rtf")) {
        parse_rtf(content)
    } else if lower.ends_with(".csv") {
        match parse_csv(content) {
            Ok(sets) => sets,
            Err(e) => return ParsedQuestionSetFile { sets: vec![], errors: vec![e] },
        }
    } else {
        return ParsedQuestionSetFile {
            sets: vec![],
            errors: vec!["Only .rtf and .csv question set files can be imported.".to_string()],
        };
    };

    if raw_sets.is_empty() {
        return ParsedQuestionSetFile {
            sets: vec![],
            errors: vec!["No questions were found in the file.".to_string()],
        };
    }

    ParsedQuestionSetFile { sets: raw_sets.into_iter().map(finish_set).collect(), errors: vec![] }
}

/// Puts a set's questions in question order and checks that it is a complete set of storable types.
fn finish_set(raw: RawSet) -> ParsedQuestionSet {
    let mut errors = Vec::new();
    let mut slots: Vec<Option<char>> = vec![None; QUESTIONS_PER_ROUND];

    for q in &raw.questions {
        let code = match q.code.chars().collect::<Vec<_>>().as_slice() {
            [c] => *c,
            _ => {
                errors.push(format!("Question {}: '{}' is not a question type.", q.number, q.code));
                continue;
            }
        };
        if code == 'M' {
            errors.push(format!("Question {}: M (Memory Verse) can't be imported as a type; use R, V or Q.", q.number));
            continue;
        }
        if !is_storable_type(code) {
            errors.push(format!("Question {}: '{}' is not a question type.", q.number, code));
            continue;
        }
        if q.number == 0 || q.number > QUESTIONS_PER_ROUND {
            errors.push(format!("Question number {} is outside 1–{}.", q.number, QUESTIONS_PER_ROUND));
            continue;
        }
        if slots[q.number - 1].is_some() {
            errors.push(format!("Question {} appears more than once.", q.number));
            continue;
        }
        slots[q.number - 1] = Some(code);
    }

    let missing: Vec<String> = slots
        .iter()
        .enumerate()
        .filter(|(_, s)| s.is_none())
        .map(|(i, _)| (i + 1).to_string())
        .collect();
    if !missing.is_empty() && errors.is_empty() {
        errors.push(format!("Missing question(s) {}.", missing.join(", ")));
    }

    if errors.is_empty() {
        // Counts (including S with I) are checked on save, where the round's standard setting is known.
        let types: String = slots.into_iter().flatten().collect();
        return ParsedQuestionSet { name: raw.name, question_types: types, errors };
    }
    ParsedQuestionSet { name: raw.name, question_types: String::new(), errors }
}

fn default_set_name(index: usize) -> String {
    format!("Set #{}", index + 1)
}

/// Starts a new set when a heading names one, or when numbering restarts at 1 without a heading.
fn push_question(sets: &mut Vec<RawSet>, number: usize, code: String) {
    let needs_new = match sets.last() {
        None => true,
        Some(last) => number == 1 && !last.questions.is_empty(),
    };
    if needs_new {
        let name = default_set_name(sets.len());
        sets.push(RawSet { name, questions: vec![] });
    }
    sets.last_mut().unwrap().questions.push(RawQuestion { number, code });
}

fn parse_rtf(content: &str) -> Vec<RawSet> {
    let mut sets: Vec<RawSet> = Vec::new();

    for line in content.lines() {
        if let Some((code, number)) = rtf_question_start(line) {
            push_question(&mut sets, number, code.to_string());
        } else if let Some(name) = rtf_set_heading(line) {
            sets.push(RawSet { name, questions: vec![] });
        }
    }
    sets.retain(|s| !s.questions.is_empty());
    sets
}

/// A question paragraph in the exported RTF reads `... <type>\tab <number>. <question>`.
fn rtf_question_start(line: &str) -> Option<(char, usize)> {
    let tab = line.find("\\tab")?;
    let before = line[..tab].trim_end();
    let mut rev = before.chars().rev();
    let code = rev.next()?;
    // The type letter stands alone: it follows whitespace, not the tail of a control word.
    if !code.is_ascii_alphabetic() || !rev.next().map_or(true, |c| c.is_whitespace()) {
        return None;
    }
    let after = line[tab + "\\tab".len()..].trim_start();
    let digits: String = after.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() || !after[digits.len()..].starts_with('.') {
        return None;
    }
    Some((code.to_ascii_uppercase(), digits.parse().ok()?))
}

/// A set heading paragraph contains `Set #<n> ...`; the heading runs to the next control word.
fn rtf_set_heading(line: &str) -> Option<String> {
    let start = line.find("Set #")?;
    let rest = &line[start..];
    let end = rest.find('\\').unwrap_or(rest.len());
    let name = rest[..end].trim().trim_end_matches('}').trim();
    if name.is_empty() { None } else { Some(name.to_string()) }
}

fn parse_csv(content: &str) -> Result<Vec<RawSet>, String> {
    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .from_reader(content.trim_start_matches('\u{feff}').as_bytes());

    let headers = reader.headers().map_err(|e| format!("Could not read the CSV header row: {}", e))?.clone();
    let column = |wanted: &[&str]| {
        headers.iter().position(|h| {
            let h = h.trim().to_lowercase();
            wanted.iter().any(|w| h == *w)
        })
    };
    let type_col = column(&["type"]).ok_or("The CSV has no \"Type\" column.")?;
    let number_col = column(&["no.", "no", "number"]).ok_or("The CSV has no \"No.\" column.")?;
    let set_col = column(&["set"]);

    let mut sets: Vec<RawSet> = Vec::new();
    for (row, record) in reader.records().enumerate() {
        let record = record.map_err(|e| format!("Could not read CSV row {}: {}", row + 2, e))?;
        let code = record.get(type_col).unwrap_or("").trim().to_uppercase();
        let number_text = record.get(number_col).unwrap_or("").trim();
        if code.is_empty() && number_text.is_empty() {
            continue;
        }
        let number: usize = number_text
            .trim_matches(|c: char| !c.is_ascii_digit())
            .parse()
            .map_err(|_| format!("CSV row {}: \"{}\" is not a question number.", row + 2, number_text))?;

        match set_col.and_then(|c| record.get(c)).map(str::trim).filter(|s| !s.is_empty()) {
            Some(set_name) => {
                if sets.last().map_or(true, |s| s.name != set_name) {
                    sets.push(RawSet { name: set_name.to_string(), questions: vec![] });
                }
                sets.last_mut().unwrap().questions.push(RawQuestion { number, code });
            }
            None => push_question(&mut sets, number, code),
        }
    }
    Ok(sets)
}

#[cfg(test)]
mod tests {
    use super::*;

    const STANDARD: &str = "GARGGVGAGGQGAGXGSAGG";

    #[test]
    fn validate_accepts_empty_and_standard_sets() {
        assert!(validate("", true).is_ok());
        assert!(validate(STANDARD, true).is_ok());
        assert!(validate(&STANDARD.replace('S', "I"), true).is_ok());
    }

    #[test]
    fn validate_rejects_partial_sets_and_m() {
        assert!(validate("GAR", false).is_err());
        assert!(validate(&STANDARD.replacen('G', "M", 1), false).is_err());
    }

    #[test]
    fn validate_checks_counts_only_when_standard() {
        let all_g = "G".repeat(20);
        assert!(validate(&all_g, true).is_err());
        assert!(validate(&all_g, false).is_ok());
    }

    #[test]
    fn validate_rejects_s_with_i_only_when_standard() {
        let both = STANDARD.replacen('G', "I", 1);
        assert!(validate(&both, true).is_err());
        assert!(validate(&both, false).is_ok());
    }

    #[test]
    fn parses_rtf_question_lines() {
        let rtf = "{\\rtf1\\ansi\n\
            {\\pard\\sa0\\b\\f0\\fs14 Set #1 Experienced QView Questions\\b0\\par}\n\
            {\\pard\\sa0\\f0\\fs14\\li576\\fi-576\\tx576 G\\tab 1. Who?\\par}\n\
            {\\pard\\sa0\\f0\\fs14\\li576 A. Jesus (John 6:11)\\par}\n\
            {\\pard\\sa0\\f0\\fs14\\li576\\fi-576\\tx576 A\\tab 2. According to?\\par}\n}";
        let sets = parse_rtf(rtf);
        assert_eq!(sets.len(), 1);
        assert_eq!(sets[0].name, "Set #1 Experienced QView Questions");
        assert_eq!(sets[0].questions.len(), 2);
        assert_eq!(sets[0].questions[1].number, 2);
        assert_eq!(sets[0].questions[1].code, "A");
    }

    #[test]
    fn parses_csv_sets_in_question_order() {
        let mut csv = String::from("\"Set\",\"Type\",\"No.\",\"Question\"\n");
        for (i, c) in STANDARD.chars().collect::<Vec<_>>().into_iter().enumerate().rev() {
            csv.push_str(&format!("\"Set #1\",\"{}\",\"{}\",\"q\"\n", c, i + 1));
        }
        let parsed = parse_question_set_file("questions.csv", &csv);
        assert!(parsed.errors.is_empty());
        assert_eq!(parsed.sets.len(), 1);
        assert_eq!(parsed.sets[0].question_types, STANDARD);
    }

    #[test]
    fn reports_incomplete_sets() {
        let csv = "Set,Type,No.\nSet #1,G,1\nSet #1,M,2\n";
        let parsed = parse_question_set_file("q.csv", csv);
        assert_eq!(parsed.sets.len(), 1);
        assert!(parsed.sets[0].question_types.is_empty());
        assert!(!parsed.sets[0].errors.is_empty());
    }
}
