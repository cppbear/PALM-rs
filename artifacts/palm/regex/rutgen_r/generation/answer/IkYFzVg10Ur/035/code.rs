// Answer 0

fn choose_match_type_test() {
    struct TestContext {
        nfa: NfaContext,
        res: Vec<Regex>,
        dfa: DfaContext,
    }

    struct NfaContext {
        insts: Vec<Inst>,
        is_anchored_start: bool,
        is_anchored_end: bool,
        prefixes: Prefixes,
    }

    struct DfaContext {}

    struct Prefixes {
        complete: bool,
    }

    struct Inst {}

    struct Regex {}

    impl DfaContext {
        fn can_exec(&self) -> bool {
            true
        }
    }

    let context = TestContext {
        nfa: NfaContext {
            insts: vec![Inst {}, Inst {}], // Not empty
            is_anchored_start: false,
            is_anchored_end: true,
            prefixes: Prefixes {
                complete: true,
            },
        },
        res: vec![Regex {}], // Length is 1
        dfa: DfaContext {},
    };

    let hint = Some(MatchType::Nfa(MatchNfaType::Auto));
    let result = choose_match_type(&context, hint);
    assert_eq!(result, MatchType::DfaAnchoredReverse);
}

