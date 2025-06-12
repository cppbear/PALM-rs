// Answer 0

#[test]
fn test_read_captures_at_zero_slots() {
    let text: &[u8] = b"example text";
    let start: usize = 0;
    let locs = &mut Locations(vec![]);
    
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::Nfa(MatchNfaType::Auto),
    });

    let exec = ExecNoSync {
        ro: &ro,
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    assert_eq!(exec.read_captures_at(locs, text, start), None);
}

#[test]
fn test_read_captures_at_two_slots() {
    let text: &[u8] = b"ab";
    let start: usize = 0;
    let locs = &mut Locations(vec![None, None]);
    
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::Dfa,
    });

    let exec = ExecNoSync {
        ro: &ro,
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    // Simulating that find_at returns a match
    // Override find_at logic for testing
    impl ExecNoSync<'_> {
        fn find_at(&self, text: &[u8], start: usize) -> Option<(usize, usize)> {
            if text == b"ab" && start == 0 {
                Some((0, 2))
            } else {
                None
            }
        }
    }
    
    assert_eq!(exec.read_captures_at(locs, text, start), Some((0, 2)));
    assert_eq!(locs.0[0], Some(0));
    assert_eq!(locs.0[1], Some(2));
}

#[test]
fn test_read_captures_at_multiple_slots() {
    let text: &[u8] = b"abcd";
    let start: usize = 0;
    let locs = &mut Locations(vec![None, None, None]);
    
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::Dfa,
    });

    let exec = ExecNoSync {
        ro: &ro,
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    // Simulating is_anchor_end_match returning true
    impl ExecNoSync<'_> {
        fn is_anchor_end_match(&self, text: &[u8]) -> bool {
            text.ends_with(b"d")
        }
    }

    // Simulating find_dfa_forward logic
    impl ExecNoSync<'_> {
        fn find_dfa_forward(&self, text: &[u8], start: usize) -> dfa::Result<(usize, usize)> {
            if text == b"abcd" && start == 0 {
                dfa::Result::Match((0, 4))
            } else {
                dfa::Result::NoMatch(0)
            }
        }
    }

    assert_eq!(exec.read_captures_at(locs, text, start), Some((0, 4)));
}

