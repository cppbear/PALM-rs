// Answer 0

#[test]
fn test_is_anchor_end_match_below_threshold() {
    struct TestRegularExpression {
        ro: Arc<ExecReadOnly>,
    }

    impl RegularExpression for TestRegularExpression {
        type Text = [u8];

        fn slots_len(&self) -> usize { 0 }
        fn locations(&self) -> Locations { Locations::default() }
        fn next_after_empty(&self, _text: &[u8], _i: usize) -> usize { 0 }
        fn shortest_match_at(&self, _text: &[u8], _start: usize) -> Option<usize> { Some(0) }
        fn is_match_at(&self, _text: &[u8], _start: usize) -> bool { true }
        fn find_at(&self, _text: &[u8], _start: usize) -> Option<(usize, usize)> { Some((0, 0)) }
        fn read_captures_at(&self, _locs: &mut Locations, _text: &[u8], _start: usize) -> Option<(usize, usize)> { None }
    }

    let suffixes = LiteralSearcher::empty();
    let nfa = Program::default();
    let ro = ExecReadOnly {
        res: vec!["test".to_string()],
        nfa,
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes,
        match_type: MatchType::default(),
    };

    let text: [u8; 1 << 20] = [0; 1 << 20];
    let regex = TestRegularExpression { ro: Arc::new(ro) };
    assert_eq!(regex.is_anchor_end_match(&text), true);
}

