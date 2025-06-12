// Answer 0

#[test]
fn test_set_non_match_returns_no_match() {
    struct TestResult;

    impl TestResult {
        fn no_match(at: usize) -> Result<usize> {
            Result::NoMatch(at)
        }
    }

    let input = TestResult::no_match(5);
    let result = input.set_non_match(10);

    match result {
        Result::NoMatch(value) => assert_eq!(value, 10),
        _ => panic!("Expected Result::NoMatch, got a different result"),
    }
}

#[test]
fn test_set_non_match_with_different_value() {
    struct TestResult;

    impl TestResult {
        fn no_match(at: usize) -> Result<usize> {
            Result::NoMatch(at)
        }
    }

    let input = TestResult::no_match(20);
    let result = input.set_non_match(30);

    match result {
        Result::NoMatch(value) => assert_eq!(value, 30),
        _ => panic!("Expected Result::NoMatch, got a different result"),
    }
}

