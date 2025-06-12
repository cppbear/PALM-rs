// Answer 0

#[derive(Debug)]
enum Result<T> {
    NoMatch(usize),
    Match(T),
}

impl<T> Result<T> {
    fn set_non_match(self, at: usize) -> Result<T> {
        match self {
            Result::NoMatch(_) => Result::NoMatch(at),
            r => r,
        }
    }
}

#[test]
fn test_set_non_match_no_match() {
    let result = Result::NoMatch(3);
    let updated_result = result.set_non_match(5);
    match updated_result {
        Result::NoMatch(at) => assert_eq!(at, 5),
        _ => panic!("Expected NoMatch variant"),
    }
}

#[test]
fn test_set_non_match_match() {
    let result = Result::Match("some match");
    let updated_result = result.set_non_match(5);
    match updated_result {
        Result::Match(_) => assert!(true),
        _ => panic!("Expected Match variant"),
    }
}

#[test]
fn test_set_non_match_edge_case() {
    let result = Result::NoMatch(0);
    let updated_result = result.set_non_match(1);
    match updated_result {
        Result::NoMatch(at) => assert_eq!(at, 1),
        _ => panic!("Expected NoMatch variant"),
    }
}

