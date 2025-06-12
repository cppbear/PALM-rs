// Answer 0

#[test]
fn test_is_match_true() {
    let res = Result::Match(42);
    assert!(res.is_match());
}

#[test]
fn test_is_match_no_match() {
    let res = Result::NoMatch(1);
    assert!(!res.is_match());
}

#[test]
fn test_is_match_quit() {
    let res = Result::Quit;
    assert!(!res.is_match());
}

