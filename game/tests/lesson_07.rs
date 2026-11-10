//! Lesson 7 — Hall of Fame.  Run: cargo test --test lesson_07
use rust_lander::highscore::*;
use std::io;
use std::path::PathBuf;

fn entry(name: &str, score: u32) -> Entry {
    Entry {
        name: name.to_string(),
        score,
    }
}

fn table(entries: &[(&str, u32)]) -> HighScores {
    HighScores {
        entries: entries.iter().map(|&(n, s)| entry(n, s)).collect(),
    }
}

/// A fresh, unique temporary file path for this test.
fn temp_file(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rust_lander_tests_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(name);
    let _ = std::fs::remove_file(&path);
    path
}

#[test]
fn io_errors_convert() {
    let err: HighScoreError = io::Error::new(io::ErrorKind::PermissionDenied, "nope").into();
    assert!(matches!(err, HighScoreError::Io(e) if e.kind() == io::ErrorKind::PermissionDenied));
}

#[test]
fn parse_good_file() {
    let parsed = HighScores::parse("BOB,980\nACE,1250\n").unwrap();
    assert_eq!(
        parsed,
        table(&[("ACE", 1250), ("BOB", 980)]),
        "sorted best first"
    );
}

#[test]
fn parse_is_forgiving_about_whitespace() {
    let parsed = HighScores::parse("\n  ACE , 1250  \n\n   \nBOB,980").unwrap();
    assert_eq!(parsed, table(&[("ACE", 1250), ("BOB", 980)]));
}

#[test]
fn parse_empty() {
    assert_eq!(HighScores::parse("").unwrap(), HighScores::default());
}

#[test]
fn parse_keeps_only_the_best() {
    let text = "A,1\nB,2\nC,3\nD,4\nE,5\nF,6\nG,7\n";
    let parsed = HighScores::parse(text).unwrap();
    assert_eq!(
        parsed,
        table(&[("G", 7), ("F", 6), ("E", 5), ("D", 4), ("C", 3)])
    );
}

#[test]
fn parse_errors_report_the_line() {
    match HighScores::parse("ACE,100\n\nno comma here\n") {
        Err(HighScoreError::Parse { line, content }) => {
            assert_eq!(line, 3, "lines count from 1, blank lines included");
            assert_eq!(content, "no comma here");
        }
        other => panic!("expected a Parse error, got {other:?}"),
    }
    match HighScores::parse("ACE,lots\n") {
        Err(HighScoreError::Parse { line: 1, content }) => assert_eq!(content, "ACE,lots"),
        other => panic!("expected a Parse error on line 1, got {other:?}"),
    }
    assert!(
        HighScores::parse("ACE,-5").is_err(),
        "u32 can't be negative"
    );
}

#[test]
fn to_text_round_trips() {
    let t = table(&[("ACE", 1250), ("BOB", 980)]);
    assert_eq!(t.to_text(), "ACE,1250\nBOB,980\n");
    assert_eq!(HighScores::parse(&t.to_text()).unwrap(), t);
    assert_eq!(HighScores::default().to_text(), "");
}

#[test]
fn qualifies() {
    let empty = HighScores::default();
    assert!(empty.qualifies(1));
    assert!(!empty.qualifies(0), "zero never qualifies");

    let not_full = table(&[("A", 500), ("B", 400)]);
    assert!(not_full.qualifies(10), "free space: anything > 0 qualifies");

    let full = table(&[("A", 500), ("B", 400), ("C", 300), ("D", 200), ("E", 100)]);
    assert!(full.qualifies(101));
    assert!(!full.qualifies(100), "a tie with the lowest isn't enough");
    assert!(!full.qualifies(50));
}

#[test]
fn add_returns_rank() {
    let mut t = table(&[("A", 500), ("B", 300)]);
    assert_eq!(t.add("NEW", 400), Some(1));
    assert_eq!(t, table(&[("A", 500), ("NEW", 400), ("B", 300)]));
    assert_eq!(t.add("TOP", 900), Some(0));
    assert_eq!(t.add("LOW", 1), Some(4));
    assert_eq!(t.entries.len(), MAX_ENTRIES);
    assert_eq!(
        t.add("NOPE", 1),
        None,
        "full table, doesn't beat the lowest"
    );
    assert_eq!(t.add("ZERO", 0), None);
    assert_eq!(t.entries.len(), MAX_ENTRIES);
    assert_eq!(t.add("MID", 350), Some(3));
    assert_eq!(
        t.entries.last().unwrap().name,
        "B",
        "LOW was pushed off the table"
    );
}

#[test]
fn add_ties_go_below_older_scores() {
    let mut t = table(&[("OLD", 300)]);
    assert_eq!(t.add("NEW", 300), Some(1));
}

#[test]
fn load_missing_file_is_empty() {
    let path = temp_file("does_not_exist.txt");
    assert_eq!(HighScores::load(&path).unwrap(), HighScores::default());
}

#[test]
fn save_then_load() {
    let path = temp_file("roundtrip.txt");
    let t = table(&[("ACE", 1250), ("BOB", 980)]);
    t.save(&path).unwrap();
    assert_eq!(
        std::fs::read_to_string(&path).unwrap(),
        "ACE,1250\nBOB,980\n"
    );
    assert_eq!(HighScores::load(&path).unwrap(), t);
}

#[test]
fn load_reports_parse_errors() {
    let path = temp_file("corrupt.txt");
    std::fs::write(&path, "ACE,1250\ngarbage\n").unwrap();
    assert!(matches!(
        HighScores::load(&path),
        Err(HighScoreError::Parse { line: 2, .. })
    ));
}

#[test]
fn load_reports_other_io_errors() {
    // A directory can't be read as a file.
    let dir = std::env::temp_dir();
    assert!(matches!(HighScores::load(&dir), Err(HighScoreError::Io(_))));
}
