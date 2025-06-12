// Answer 0

#[test]
fn test_set_non_match_no_match() {
    let result: Result<i32> = Result::Match(42);
    let output = result.set_non_match(5);
    match output {
        Result::Match(value) => assert_eq!(value, 42),
        _ => panic!("Expected a match result"),
    }
}

#[test]
fn test_set_non_match_no_match() {
    let result: Result<i32> = Result::NoMatch(3);
    let output = result.set_non_match(5);
    match output {
        Result::NoMatch(pos) => assert_eq!(pos, 5),
        _ => panic!("Expected a no-match result"),
    }
}

