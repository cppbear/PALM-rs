// Answer 0

#[test]
fn test_is_match_no_match() {
    struct Result {
        value: String,
    }

    enum MatchResult {
        Match(Result),
        NoMatch(Result),
        Quit,
    }

    impl MatchResult {
        pub fn is_match(&self) -> bool {
            match *self {
                MatchResult::Match(_) => true,
                MatchResult::NoMatch(_) | MatchResult::Quit => false,
            }
        }
    }

    let no_match_result = MatchResult::NoMatch(Result { value: String::from("No match found") });
    assert_eq!(no_match_result.is_match(), false);
}

#[test]
fn test_is_match_quit() {
    struct Result {
        value: String,
    }

    enum MatchResult {
        Match(Result),
        NoMatch(Result),
        Quit,
    }

    impl MatchResult {
        pub fn is_match(&self) -> bool {
            match *self {
                MatchResult::Match(_) => true,
                MatchResult::NoMatch(_) | MatchResult::Quit => false,
            }
        }
    }

    let quit_result = MatchResult::Quit;
    assert_eq!(quit_result.is_match(), false);
}

