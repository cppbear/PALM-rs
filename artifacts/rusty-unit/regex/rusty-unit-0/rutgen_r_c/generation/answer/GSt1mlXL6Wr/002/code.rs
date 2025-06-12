// Answer 0

#[test]
fn test_set_non_match_no_match() {
    // Given a Result::NoMatch instance
    let non_match_position = 5; // Arbitrary non-match position
    let result = Result::NoMatch(non_match_position);

    // When set_non_match is called with a new position
    let new_result = result.set_non_match(10);

    // Then it should return Result::NoMatch with the new position
    match new_result {
        Result::NoMatch(at) => assert_eq!(at, 10),
        _ => panic!("Expected Result::NoMatch with position 10"),
    }
}

#[test]
fn test_set_non_match_match() {
    // Given a Result::Match instance
    let match_value = 42; // Arbitrary match value
    let result = Result::Match(match_value);

    // When set_non_match is called with a non-match position
    let new_result = result.set_non_match(10);

    // Then it should return the original Result::Match
    match new_result {
        Result::Match(val) => assert_eq!(val, match_value),
        _ => panic!("Expected Result::Match with value 42"),
    }
}

