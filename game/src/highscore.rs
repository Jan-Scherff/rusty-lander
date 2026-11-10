//! # Lesson 7 — Hall of Fame
//!
//! High scores survive between games by being saved to a text file:
//!
//! ```text
//! ACE,1250
//! BOB,980
//! ```
//!
//! One `NAME,SCORE` per line. Reading files and parsing text can fail in
//! many ways, so this lesson is about **error handling**: custom error
//! enums, `Result`, the `?` operator and the `From` trait.

use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

/// Only the best five scores are kept.
pub const MAX_ENTRIES: usize = 5;

/// One line of the high-score table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// `String` (not `&str`): the entry *owns* its text.
    pub name: String,
    pub score: u32,
}

/// The table, best score first.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HighScores {
    pub entries: Vec<Entry>,
}

/// Everything that can go wrong when loading high scores.
#[derive(Debug)]
pub enum HighScoreError {
    /// The file system complained (permissions, disk full, ...).
    Io(io::Error),
    /// Line number `line` (counting from 1) is not valid `NAME,SCORE`.
    Parse { line: usize, content: String },
}

impl fmt::Display for HighScoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HighScoreError::Io(err) => write!(f, "could not access high scores: {err}"),
            HighScoreError::Parse { line, content } => {
                write!(f, "bad high score on line {line}: {content:?}")
            }
        }
    }
}

/// Lets `HighScoreError` be used like any other error type
/// (e.g. in `Box<dyn std::error::Error>`).
impl std::error::Error for HighScoreError {}

/// Converting an `io::Error` into our error type. This is what makes `?`
/// work on IO operations inside functions returning `HighScoreError`:
/// `?` calls `From::from` on the error automatically.
impl From<io::Error> for HighScoreError {
    fn from(err: io::Error) -> HighScoreError {
        // ✏️ LESSON 7: wrap it in the right variant
        todo!("Lesson 7")
    }
}

impl HighScores {
    /// Parses the file format described at the top of this module.
    ///
    /// - Skip blank lines (after trimming whitespace).
    /// - Split each line at the comma (`str::split_once`); trim both halves.
    /// - Parse the score with `.parse::<u32>()`.
    /// - Any malformed line → `Err(HighScoreError::Parse { line, content })`
    ///   where `line` counts from 1 and `content` is the trimmed line.
    /// - At the end, call `sort_and_trim` so the table is in order.
    pub fn parse(text: &str) -> Result<HighScores, HighScoreError> {
        // ✏️ LESSON 7: lines().enumerate(), split_once, parse, ok_or / map_err, ?
        todo!("Lesson 7")
    }

    /// The reverse of `parse`: one `NAME,SCORE` line per entry, each ending
    /// in `\n`.
    pub fn to_text(&self) -> String {
        // ✏️ LESSON 7: format! each entry, or push_str onto a String
        todo!("Lesson 7")
    }

    /// Would `score` make it onto the table? It must be greater than 0, and
    /// either the table has free space or `score` beats the lowest entry.
    pub fn qualifies(&self, score: u32) -> bool {
        // ✏️ LESSON 7: `self.entries.last()` is the lowest score
        todo!("Lesson 7")
    }

    /// Adds a score if it `qualifies`, keeping the table sorted and at most
    /// `MAX_ENTRIES` long. Returns the new entry's rank (0 = best), or `None`
    /// if the score didn't qualify.
    ///
    /// `name: &str` is *borrowed*; the entry needs an owned `String`.
    pub fn add(&mut self, name: &str, score: u32) -> Option<usize> {
        // ✏️ LESSON 7: push, sort_and_trim, then find the position
        todo!("Lesson 7")
    }

    /// Loads the table from `path`. A missing file is *not* an error — it
    /// just means nobody has played yet, so return an empty table.
    ///
    /// Hint: `fs::read_to_string`, then `match` on the result. An
    /// `io::Error` has a `.kind()`; compare it with `io::ErrorKind::NotFound`.
    pub fn load(path: &Path) -> Result<HighScores, HighScoreError> {
        // ✏️ LESSON 7: three match arms: Ok, Err(NotFound), other Err
        todo!("Lesson 7")
    }

    /// Saves the table to `path` (overwriting it), using `to_text`.
    pub fn save(&self, path: &Path) -> Result<(), HighScoreError> {
        // ✏️ LESSON 7: `fs::write(...)?;` then `Ok(())`
        todo!("Lesson 7")
    }

    /// Sorts best-first and drops everything beyond `MAX_ENTRIES`.
    /// *(Provided.)* `sort_by` is *stable*: equal scores keep their order,
    /// so an older score stays ahead of a newer one with the same points.
    pub fn sort_and_trim(&mut self) {
        self.entries.sort_by(|a, b| b.score.cmp(&a.score));
        self.entries.truncate(MAX_ENTRIES);
    }
}
