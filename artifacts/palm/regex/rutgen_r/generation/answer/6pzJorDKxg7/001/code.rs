// Answer 0

#[derive(Debug)]
enum Result {
    Match(String),
    NoMatch(String),
    Quit,
}

impl Result {
    pub fn is_match(&self) -> bool {
        match *self {
            Result::Match(_) => true,
            Result::NoMatch(_) | Result::Quit => false,
        }
    }
}

#[test]
fn test_is_match_quit() {
    let result = Result::Quit;
    assert_eq!(result.is_match(), false);
}

#[test]
fn test_is_match_no_match() {
    let result = Result::NoMatch(String::from("example"));
    assert_eq!(result.is_match(), false);
}

