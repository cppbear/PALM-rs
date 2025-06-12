// Answer 0

#[test]
fn test_set_non_match_no_match() {
    let result = Result::Match(42);
    let at = 0;
    let _ = result.set_non_match(at);
}

#[test]
fn test_set_non_match_match() {
    let result = Result::Match("test");
    let at = 5;
    let _ = result.set_non_match(at);
}

#[test]
fn test_set_non_match_quit() {
    let result = Result::Quit;
    let at = 10;
    let _ = result.set_non_match(at);
}

#[test]
fn test_set_non_match_edge_case() {
    let result = Result::Match(vec![1, 2, 3]);
    let at = STATE_MAX;
    let _ = result.set_non_match(at);
}

