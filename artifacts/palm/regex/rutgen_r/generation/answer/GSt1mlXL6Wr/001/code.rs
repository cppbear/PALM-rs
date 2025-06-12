// Answer 0

#[test]
fn test_set_non_match_not_no_match() {
    // Given a Result that is not NoMatch
    let result = Result::Match("some match data");
    
    // When calling set_non_match
    let new_result = result.set_non_match(5);
    
    // Then it should return the original Result without any changes
    match new_result {
        Result::Match(data) => assert_eq!(data, "some match data"),
        _ => panic!("Expected Result::Match but got a different variant."),
    }
}

#[test]
fn test_set_non_match_with_non_no_match() {
    // Given a Result::Error variant
    let result = Result::Error("an error occurred");
    
    // When calling set_non_match
    let new_result = result.set_non_match(10);
    
    // Then it should return the original Result, unchanged
    match new_result {
        Result::Error(err) => assert_eq!(err, "an error occurred"),
        _ => panic!("Expected Result::Error but got a different variant."),
    }
}

