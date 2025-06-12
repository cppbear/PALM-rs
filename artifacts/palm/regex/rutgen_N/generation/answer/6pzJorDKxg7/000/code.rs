// Answer 0

#[derive(Debug)]
enum Result {
    Match(u32),
    NoMatch(String),
    Quit,
}

#[test]
fn test_is_match_with_match() {
    let result = Result::Match(42);
    assert_eq!(result.is_match(), true);
}

#[test]
fn test_is_match_with_no_match() {
    let result = Result::NoMatch(String::from("no match"));
    assert_eq!(result.is_match(), false);
}

#[test]
fn test_is_match_with_quit() {
    let result = Result::Quit;
    assert_eq!(result.is_match(), false);
}

