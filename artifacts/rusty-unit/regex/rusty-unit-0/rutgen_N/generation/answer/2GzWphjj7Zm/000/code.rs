// Answer 0

#[test]
fn test_bounded_backtracking_sets_match_type_correctly() {
    struct Regex {
        match_type: Option<MatchType>,
    }

    enum MatchType {
        Nfa(MatchNfaType),
    }

    enum MatchNfaType {
        Backtrack,
    }

    impl Regex {
        pub fn new() -> Self {
            Regex { match_type: None }
        }

        pub fn bounded_backtracking(mut self) -> Self {
            self.match_type = Some(MatchType::Nfa(MatchNfaType::Backtrack));
            self
        }
    }

    let regex = Regex::new();
    let modified_regex = regex.bounded_backtracking();

    assert!(matches!(modified_regex.match_type, Some(MatchType::Nfa(MatchNfaType::Backtrack))));
}

#[test]
fn test_bounded_backtracking_overrides_previous_setting() {
    struct Regex {
        match_type: Option<MatchType>,
    }

    enum MatchType {
        Nfa(MatchNfaType),
        Automatic,
    }

    enum MatchNfaType {
        Backtrack,
    }

    impl Regex {
        pub fn new() -> Self {
            Regex { match_type: None }
        }

        pub fn automatic(mut self) -> Self {
            self.match_type = Some(MatchType::Automatic);
            self
        }

        pub fn bounded_backtracking(mut self) -> Self {
            self.match_type = Some(MatchType::Nfa(MatchNfaType::Backtrack));
            self
        }
    }

    let regex = Regex::new().automatic();
    let modified_regex = regex.bounded_backtracking();

    assert!(matches!(modified_regex.match_type, Some(MatchType::Nfa(MatchNfaType::Backtrack))));
}

