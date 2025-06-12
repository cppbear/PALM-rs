// Answer 0

#[test]
fn test_map_match() {
    struct TestType;

    let result = Result::Match(TestType);
    let mapped_result: Result<usize> = result.map(|_| 42).unwrap();
    if let Result::Match(value) = mapped_result {
        assert_eq!(value, 42);
    } else {
        panic!("Expected Result::Match with value 42");
    }
}

#[test]
fn test_map_no_match() {
    struct TestType;

    let result = Result::NoMatch("no match");
    let mapped_result: Result<usize> = result.map(|_| 42).unwrap();
    if let Result::NoMatch(message) = mapped_result {
        assert_eq!(message, "no match");
    } else {
        panic!("Expected Result::NoMatch with original message");
    }
}

#[test]
fn test_map_quit() {
    struct TestType;

    let result = Result::Quit;
    let mapped_result: Result<usize> = result.map(|_| 42);
    assert_eq!(mapped_result, Result::Quit);
}

