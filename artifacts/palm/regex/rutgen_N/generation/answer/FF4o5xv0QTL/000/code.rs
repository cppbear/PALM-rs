// Answer 0

#[derive(Debug)]
struct MatchType {
    nfa: Option<MatchNfaType>,
}

#[derive(Debug)]
enum MatchNfaType {
    PikeVM,
}

struct Regex {
    match_type: Option<MatchType>,
}

impl Regex {
    pub fn new() -> Self {
        Regex {
            match_type: None,
        }
    }

    pub fn nfa(mut self) -> Self {
        self.match_type = Some(MatchType { nfa: Some(MatchNfaType::PikeVM) });
        self
    }
}

#[test]
fn test_nfa_sets_match_type_to_nfa() {
    let regex = Regex::new();
    let updated_regex = regex.nfa();

    assert!(updated_regex.match_type.is_some());
    if let Some(match_type) = updated_regex.match_type {
        assert!(match_type.nfa.is_some());
        assert_eq!(match_type.nfa, Some(MatchNfaType::PikeVM));
    } else {
        panic!("Match type should not be None");
    }
}

#[test]
fn test_nfa_overrides_previous_match_type() {
    let regex = Regex::new().nfa();
    let updated_regex = regex.nfa();

    assert!(updated_regex.match_type.is_some());
    if let Some(match_type) = updated_regex.match_type {
        assert!(match_type.nfa.is_some());
        assert_eq!(match_type.nfa, Some(MatchNfaType::PikeVM));
    } else {
        panic!("Match type should not be None");
    }
}

