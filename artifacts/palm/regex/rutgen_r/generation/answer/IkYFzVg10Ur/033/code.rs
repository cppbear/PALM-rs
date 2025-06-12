// Answer 0

fn test_choose_match_type() {
    #[derive(Default)]
    struct Nfa {
        insts: Vec<usize>,
        is_anchored_start: bool,
        is_anchored_end: bool,
        prefixes: Prefixes,
    }

    #[derive(Default)]
    struct Prefixes {
        complete: bool,
    }

    #[derive(Default)]
    struct Dfa;

    #[derive(Default)]
    struct Regex {
        nfa: Nfa,
        res: Vec<usize>,
        dfa: Dfa,
    }

    impl Regex {
        fn choose_match_type(&self, hint: Option<MatchType>) -> MatchType {
            use self::MatchType::*;
            if let Some(Nfa(_)) = hint {
                return hint.unwrap();
            }
            if self.nfa.insts.is_empty() {
                return Nothing;
            }
            if self.res.len() == 1 {
                if self.nfa.prefixes.complete {
                    return if self.nfa.is_anchored_start {
                        Literal(MatchLiteralType::AnchoredStart)
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
            true
        }
    }

    #[derive(Debug)]
    enum MatchType {
        Nfa(MatchNfaType),
        DfaSuffix,
        Dfa,
        DfaMany,
        DfaAnchoredReverse,
        Literal(MatchLiteralType),
        Nothing,
    }

    #[derive(Debug)]
    enum MatchNfaType {
        Auto,
    }

    #[derive(Debug)]
    enum MatchLiteralType {
        AnchoredStart,
        Unanchored,
        AnchoredEnd,
    }

    mod dfa {
        use super::*;
        pub fn can_exec(_: &Dfa) -> bool {
            true
        }
    }

    let regex = Regex {
        nfa: Nfa {
            insts: vec![1],
            is_anchored_start: true,
            is_anchored_end: false,
            prefixes: Prefixes {
                complete: false,
            },
        },
        res: vec![1],
        dfa: Dfa::default(),
    };

    let hint = Some(MatchType::Nfa(MatchNfaType::Auto));
    let result = regex.choose_match_type(hint);
    assert_eq!(result, MatchType::DfaSuffix);
}

