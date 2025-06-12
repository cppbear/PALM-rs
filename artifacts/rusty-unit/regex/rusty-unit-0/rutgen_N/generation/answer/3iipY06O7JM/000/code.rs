// Answer 0

#[test]
fn test_automatic_sets_match_type_to_none() {
    struct Regex {
        match_type: Option<String>,
    }

    impl Regex {
        pub fn new() -> Self {
            Regex { match_type: Some("nfa".to_string()) } // Initial state simulating a prior setting
        }

        pub fn automatic(mut self) -> Self {
            self.match_type = None;
            self
        }
    }

    let regex = Regex::new();
    let updated_regex = regex.automatic();
    assert_eq!(updated_regex.match_type, None);
}

#[test]
fn test_automatic_stays_none_when_called_multiple_times() {
    struct Regex {
        match_type: Option<String>,
    }

    impl Regex {
        pub fn new() -> Self {
            Regex { match_type: Some("nfa".to_string()) }
        }

        pub fn automatic(mut self) -> Self {
            self.match_type = None;
            self
        }
    }

    let regex = Regex::new();
    let updated_regex = regex.automatic().automatic();
    assert_eq!(updated_regex.match_type, None);
}

