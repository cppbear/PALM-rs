// Answer 0

#[test]
fn test_result_no_match() {
    let no_match_result = Result::NoMatch(0);
    assert_eq!(no_match_result.is_match(), false);
}

#[test]
fn test_result_no_match_large_index() {
    let no_match_result = Result::NoMatch(usize::MAX);
    assert_eq!(no_match_result.is_match(), false);
}

#[test]
fn test_result_quit() {
    let quit_result = Result::Quit;
    assert_eq!(quit_result.is_match(), false);
}

