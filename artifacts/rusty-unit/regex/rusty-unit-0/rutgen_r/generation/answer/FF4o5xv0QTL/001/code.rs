// Answer 0

#[test]
fn test_nfa_sets_match_type_to_nfa() {
    struct RegexEngine {
        match_type: Option<MatchType>,
    }

    enum MatchType {
        Nfa(MatchNfaType),
    }

    enum MatchNfaType {
        PikeVM,
    }

    impl RegexEngine {
        pub fn nfa(mut self) -> Self {
            self.match_type = Some(MatchType::Nfa(MatchNfaType::PikeVM));
            self
        }
    }

    let engine = RegexEngine { match_type: None };
    let updated_engine = engine.nfa();
    
    assert!(updated_engine.match_type.is_some());
    match updated_engine.match_type {
        Some(MatchType::Nfa(MatchNfaType::PikeVM)) => assert!(true),
        _ => assert!(false, "Match type should be set to Nfa with PikeVM variant"),
    }
}

#[test]
fn test_nfa_overrides_previous_setting() {
    struct RegexEngine {
        match_type: Option<MatchType>,
    }

    enum MatchType {
        Nfa(MatchNfaType),
        Other,
    }

    enum MatchNfaType {
        PikeVM,
    }

    impl RegexEngine {
        pub fn nfa(mut self) -> Self {
            self.match_type = Some(MatchType::Nfa(MatchNfaType::PikeVM));
            self
        }
    }

    let engine = RegexEngine { match_type: Some(MatchType::Other) };
    let updated_engine = engine.nfa();

    assert!(updated_engine.match_type.is_some());
    match updated_engine.match_type {
        Some(MatchType::Nfa(MatchNfaType::PikeVM)) => assert!(true),
        _ => assert!(false, "Match type should be set to Nfa with PikeVM variant"),
    }
}

