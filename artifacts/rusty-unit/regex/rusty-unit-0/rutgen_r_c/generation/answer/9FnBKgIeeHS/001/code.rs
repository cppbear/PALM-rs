// Answer 0

#[test]
fn test_exec_pikevm_bytes_case() {
    struct TestRegularExpression {
        uses_bytes: bool,
    }

    impl RegularExpression for TestRegularExpression {
        type Text = [u8];

        fn slots_len(&self) -> usize {
            0
        }

        fn locations(&self) -> Locations {
            Locations::default()
        }

        fn next_after_empty(&self, _text: &[u8], _i: usize) -> usize {
            0
        }

        fn shortest_match_at(&self, _text: &[u8], _start: usize) -> Option<usize> {
            Some(0)
        }

        fn is_match_at(&self, _text: &[u8], _start: usize) -> bool {
            true
        }

        fn find_at(&self, _text: &[u8], _start: usize) -> Option<(usize, usize)> {
            Some((0, 1))
        }

        fn read_captures_at(&self, _locs: &mut Locations, _text: &[u8], _start: usize) -> Option<(usize, usize)> {
            Some((0, 1))
        }
    }

    let nfa = Program {
        is_bytes: true,
        ..Program::new()
    };

    let ro = ExecReadOnly {
        res: vec!["test".to_string()],
        nfa,
        dfa: Program::new(),
        dfa_reverse: Program::new(),
        suffixes: LiteralSearcher::empty(),
        match_type: MatchType::default(),
    };

    let exec = ExecNoSync {
        ro: &Arc::new(ro),
        cache: &RefCell::new(ProgramCacheInner {
            pikevm: pikevm::Cache::default(),
            backtrack: backtrack::Cache::default(),
            dfa: dfa::Cache::default(),
            dfa_reverse: dfa::Cache::default(),
        }),
    };

    let mut matches = vec![false; 1];
    let mut slots: Vec<Slot> = vec![Slot::default(); 1];
    let text = b"test";
    let start = 0;

    let result = exec.exec_pikevm(&mut matches, &mut slots, false, text, start);
    assert!(result);
    assert!(matches[0]);
}

#[test]
#[should_panic]
fn test_exec_pikevm_with_panic() {
    struct TestRegularExpression {
        uses_bytes: bool,
    }

    impl RegularExpression for TestRegularExpression {
        type Text = [u8];

        fn slots_len(&self) -> usize {
            0
        }

        fn locations(&self) -> Locations {
            Locations::default()
        }

        fn next_after_empty(&self, _text: &[u8], _i: usize) -> usize {
            0
        }

        fn shortest_match_at(&self, _text: &[u8], _start: usize) -> Option<usize> {
            Some(0)
        }

        fn is_match_at(&self, _text: &[u8], _start: usize) -> bool {
            true
        }

        fn find_at(&self, _text: &[u8], _start: usize) -> Option<(usize, usize)> {
            Some((0, 1))
        }

        fn read_captures_at(&self, _locs: &mut Locations, _text: &[u8], _start: usize) -> Option<(usize, usize)> {
            Some((0, 1))
        }
    }

    let nfa = Program {
        is_bytes: true,
        ..Program::new()
    };

    let ro = ExecReadOnly {
        res: vec!["test".to_string()],
        nfa,
        dfa: Program::new(),
        dfa_reverse: Program::new(),
        suffixes: LiteralSearcher::empty(),
        match_type: MatchType::default(),
    };

    let exec = ExecNoSync {
        ro: &Arc::new(ro),
        cache: &RefCell::new(ProgramCacheInner {
            pikevm: pikevm::Cache::default(),
            backtrack: backtrack::Cache::default(),
            dfa: dfa::Cache::default(),
            dfa_reverse: dfa::Cache::default(),
        }),
    };

    let mut matches = vec![false; 1];
    let mut slots: Vec<Slot> = vec![Slot::default(); 1];
    let text = b""; // Empty input to trigger panic
    let start = 0;

    exec.exec_pikevm(&mut matches, &mut slots, false, text, start);
}

