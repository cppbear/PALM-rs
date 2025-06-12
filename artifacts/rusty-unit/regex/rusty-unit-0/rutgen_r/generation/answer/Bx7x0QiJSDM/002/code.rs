// Answer 0

#[derive(Debug, PartialEq)]
pub enum Result<T> {
    Match(T),
    NoMatch(String),
    Quit,
}

pub fn map<U, F: FnMut(T) -> U>(self, mut f: F) -> Result<U> {
    match self {
        Result::Match(t) => Result::Match(f(t)),
        Result::NoMatch(x) => Result::NoMatch(x),
        Result::Quit => Result::Quit,
    }
}

#[test]
fn test_map_no_match() {
    let input = Result::NoMatch("No Matching Value".to_string());
    let output: Result<u32> = map(input, |x: String| x.parse::<u32>().unwrap());
    assert_eq!(output, Result::NoMatch("No Matching Value".to_string()));
}

#[test]
fn test_map_no_match_empty_string() {
    let input = Result::NoMatch("".to_string());
    let output: Result<u32> = map(input, |x: String| x.parse::<u32>().unwrap());
    assert_eq!(output, Result::NoMatch("".to_string()));
}

#[test]
fn test_map_quit() {
    let input = Result::Quit;
    let output: Result<u32> = map(input, |x: String| x.parse::<u32>().unwrap());
    assert_eq!(output, Result::Quit);
}

