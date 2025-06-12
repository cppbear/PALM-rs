// Answer 0

#[test]
fn test_is_match_at_not_anchor_end_match() {
    let regex = {
        struct TestReadOnly {
            match_type: MatchType,
            nfa: Program,
            dfa: Program,
            dfa_reverse: Program,
            suffixes: LiteralSearcher,
        }
        
        struct TestExecReadOnly {
            ro: Arc<TestReadOnly>
        }

        struct TestExecNoSync<'c> {
            ro: &'c Arc<TestReadOnly>,
            cache: ProgramCache,
        }

        let nfa_program = Program {
            insts: vec![],
            matches: vec![],
            captures: vec![],
            capture_name_idx: Arc::new(HashMap::new()),
            start: 0,
            byte_classes: vec![],
            only_utf8: false,
            is_bytes: false,
            is_dfa: false,
            is_reverse: false,
            is_anchored_start: false,
            is_anchored_end: false,
            has_unicode_word_boundary: false,
            prefixes: LiteralSearcher::default(),
            dfa_size_limit: 100,
        };

        let dfa_program = nfa_program.clone();
        let dfa_reverse_program = nfa_program.clone();

        let ro = Arc::new(TestReadOnly {
            match_type: MatchType::Nothing,
            nfa: nfa_program,
            dfa: dfa_program,
            dfa_reverse: dfa_reverse_program,
            suffixes: LiteralSearcher::default(),
        });

        TestExecNoSync {
            ro: &ro,
            cache: RefCell::new(ProgramCacheInner {
                pikevm: pikevm::Cache::default(),
                backtrack: backtrack::Cache::default(),
                dfa: dfa::Cache::default(),
                dfa_reverse: dfa::Cache::default(),
            }),
        }
    };

    let text = b"example text";
    let start = 0;

    assert_eq!(regex.is_match_at(text, start), false);
}

