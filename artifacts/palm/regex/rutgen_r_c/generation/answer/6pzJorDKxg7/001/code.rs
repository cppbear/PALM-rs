// Answer 0

#[test]
fn test_is_match_quit() {
    let result: Result<u32> = Result::Quit;
    assert_eq!(result.is_match(), false);
}

#[test]
fn test_is_match_nomatch() {
    let result: Result<u32> = Result::NoMatch(5);
    assert_eq!(result.is_match(), false);
}

#[test]
fn test_is_match_match() {
    let result: Result<u32> = Result::Match(42);
    assert_eq!(result.is_match(), true);
}

