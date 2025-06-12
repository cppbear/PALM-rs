// Answer 0

#[test]
fn test_is_match_with_match_variant() {
    let result = Result::Match("test");
    assert!(result.is_match());
}

#[test]
fn test_is_match_with_no_match_variant() {
    let result = Result::NoMatch(0);
    assert!(!result.is_match());
}

#[test]
fn test_is_match_with_quit_variant() {
    let result = Result::Quit;
    assert!(!result.is_match());
}

