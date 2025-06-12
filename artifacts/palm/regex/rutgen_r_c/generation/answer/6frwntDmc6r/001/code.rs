// Answer 0

#[test]
fn test_shortest_nfa_type_auto_match() {
    struct TestRegex;
    impl RegularExpression for TestRegex {
        type Text = [u8];
        fn slots_len(&self) -> usize { 2 }
        fn locations(&self) -> Locations { Locations::default() }
        fn next_after_empty(&self, _: &Self::Text, _: usize) -> usize { 0 }
        fn shortest_match_at(&self, _: &Self::Text, _: usize) -> Option<usize> { Some(1) }
        fn is_match_at(&self, _: &Self::Text, _: usize) -> bool { true }
        fn find_at(&self, _: &Self::Text, _: usize) -> Option<(usize, usize)> { Some((0, 1)) }
        fn read_captures_at(&self, _: &mut Locations, _: &Self::Text, _: usize) -> Option<(usize, usize)> { Some((0, 1)) }
    }

    let text: &[u8] = b"test";
    let start = 0;
    let ty = MatchNfaType::Auto;
    let exec_no_sync = ExecNoSync { ro: &Arc::new(ExecReadOnly { res: vec![], nfa: Program::default(), dfa: Program::default(), dfa_reverse: Program::default(), suffixes: LiteralSearcher::default(), match_type: MatchType::default() }), cache: &RefCell::new(ProgramCacheInner::default()) };

    let result = exec_no_sync.shortest_nfa_type(ty, text, start);
    assert_eq!(result, Some(1));
}

#[test]
fn test_shortest_nfa_type_backtrack_match() {
    struct TestRegex;
    impl RegularExpression for TestRegex {
        type Text = [u8];
        fn slots_len(&self) -> usize { 2 }
        fn locations(&self) -> Locations { Locations::default() }
        fn next_after_empty(&self, _: &Self::Text, _: usize) -> usize { 0 }
        fn shortest_match_at(&self, _: &Self::Text, _: usize) -> Option<usize> { Some(1) }
        fn is_match_at(&self, _: &Self::Text, _: usize) -> bool { true }
        fn find_at(&self, _: &Self::Text, _: usize) -> Option<(usize, usize)> { Some((0, 1)) }
        fn read_captures_at(&self, _: &mut Locations, _: &Self::Text, _: usize) -> Option<(usize, usize)> { Some((0, 1)) }
    }

    let text: &[u8] = b"backtrack";
    let start = 0;
    let ty = MatchNfaType::Backtrack;
    let exec_no_sync = ExecNoSync { ro: &Arc::new(ExecReadOnly { res: vec![], nfa: Program::default(), dfa: Program::default(), dfa_reverse: Program::default(), suffixes: LiteralSearcher::default(), match_type: MatchType::default() }), cache: &RefCell::new(ProgramCacheInner::default()) };

    let result = exec_no_sync.shortest_nfa_type(ty, text, start);
    assert_eq!(result, Some(1));
}

#[test]
fn test_shortest_nfa_type_pikevm_match() {
    struct TestRegex;
    impl RegularExpression for TestRegex {
        type Text = [u8];
        fn slots_len(&self) -> usize { 2 }
        fn locations(&self) -> Locations { Locations::default() }
        fn next_after_empty(&self, _: &Self::Text, _: usize) -> usize { 0 }
        fn shortest_match_at(&self, _: &Self::Text, _: usize) -> Option<usize> { Some(1) }
        fn is_match_at(&self, _: &Self::Text, _: usize) -> bool { true }
        fn find_at(&self, _: &Self::Text, _: usize) -> Option<(usize, usize)> { Some((0, 1)) }
        fn read_captures_at(&self, _: &mut Locations, _: &Self::Text, _: usize) -> Option<(usize, usize)> { Some((0, 1)) }
    }

    let text: &[u8] = b"pikevm";
    let start = 0;
    let ty = MatchNfaType::PikeVM;
    let exec_no_sync = ExecNoSync { ro: &Arc::new(ExecReadOnly { res: vec![], nfa: Program::default(), dfa: Program::default(), dfa_reverse: Program::default(), suffixes: LiteralSearcher::default(), match_type: MatchType::default() }), cache: &RefCell::new(ProgramCacheInner::default()) };

    let result = exec_no_sync.shortest_nfa_type(ty, text, start);
    assert_eq!(result, Some(1));
}

