// Answer 0

fn test_choose_match_type() {
    struct MockNfa {
        insts: Vec<u8>,
        is_anchored_start: bool,
        is_anchored_end: bool,
        prefixes: MockComplete,
    }

    struct MockComplete {
        complete: bool,
    }

    struct MockDfa;

    struct Regex {
        nfa: MockNfa,
        res: Vec<u8>,
        dfa: MockDfa,
        suffixes: MockComplete,
    }

    impl MockComplete {
        fn complete(&self) -> bool {
            self.complete
        }
    }

    impl Regex {
        fn new(nfa: MockNfa, res: Vec<u8>, dfa: MockDfa, suffixes: MockComplete) -> Self {
            Self { nfa, res, dfa, suffixes }
        }

        fn choose_match_type(&self, hint: Option<MatchType>) -> MatchType {
            use MatchType::*;
            if let Some(Nfa(_)) = hint {
                return hint.unwrap();
            }
            if self.nfa.insts.is_empty() {
                return Nothing;
            }
            if self.res.len() == 1 {
                if self.nfa.prefixes.complete() {
                    return if self.nfa.is_anchored_start {
                        Literal(MatchLiteralType::AnchoredStart)
                    } else {
                        Literal(MatchLiteralType::Unanchored)
                    };
                }
                if self.suffixes.complete() {
                    return if self.nfa.is_anchored_end {
                        Literal(MatchLiteralType::AnchoredEnd)
                    } else {
                        Literal(MatchLiteralType::Unanchored)
                    };
                }
            }
            if dfa::can_exec(&self.dfa) {
                if self.res.len() >= 2 {
                    return DfaMany;
                }
                if !self.nfa.is_anchored_start && self.nfa.is_anchored_end {
                    return DfaAnchoredReverse;
                }
                if self.should_suffix_scan() {
                    return DfaSuffix;
                }
                return Dfa;
            }
            Nfa(MatchNfaType::Auto)
        }

        fn should_suffix_scan(&self) -> bool {
            false
        }
    }

    enum MatchType {
        Nfa(MatchNfaType),
        Dfa,
        DfaMany,
        DfaAnchoredReverse,
        DfaSuffix,
        Nothing,
        Literal(MatchLiteralType),
    }

    enum MatchNfaType {
        Auto,
    }

    enum MatchLiteralType {
        AnchoredStart,
        Unanchored,
        AnchoredEnd,
    }

    mod dfa {
        use super::MockDfa;
        pub fn can_exec(_dfa: &MockDfa) -> bool {
            true
        }
    }

    let nfa = MockNfa {
        insts: vec![1, 2, 3],
        is_anchored_start: false,
        is_anchored_end: false,
        prefixes: MockComplete { complete: false },
    };
    let dfa = MockDfa;
    let res = vec![1, 2];
    let suffixes = MockComplete { complete: false };
    
    let regex = Regex::new(nfa, res, dfa, suffixes);
    
    let hint = Some(MatchType::Nfa(MatchNfaType::Auto));
    let result = regex.choose_match_type(hint);
    
    assert_eq!(result, MatchType::Dfa);
}

