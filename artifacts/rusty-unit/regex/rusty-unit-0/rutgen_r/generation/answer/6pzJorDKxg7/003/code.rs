// Answer 0

#[derive(Debug)]
enum Result {
    Match(bool),
    NoMatch(bool),
    Quit,
}

#[test]
fn test_is_match_with_match() {
    let result = Result::Match(true);
    assert!(result.is_match());
}

#[test]
fn test_is_match_with_no_match() {
    let result = Result::NoMatch(false);
    assert!(!result.is_match());
}

#[test]
fn test_is_match_with_quit() {
    let result = Result::Quit;
    assert!(!result.is_match());
}

