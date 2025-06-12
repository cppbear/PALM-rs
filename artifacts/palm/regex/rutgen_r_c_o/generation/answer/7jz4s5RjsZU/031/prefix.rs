// Answer 0

#[test]
fn test_read_captures_at_empty_text() {
    let text: &[u8] = &[];
    let locs = Locations(Vec::new());
    let exec = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec![],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::Literal(MatchLiteralType::Unanchored),
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    exec.read_captures_at(&mut locs, text, 0);
}

#[test]
fn test_read_captures_at_two_slots() {
    let text: &[u8] = &[b'a', b'b'];
    let mut locs = Locations(vec![None, None]);
    let exec = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec![String::from("ab")],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::Literal(MatchLiteralType::Unanchored),
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    exec.read_captures_at(&mut locs, text, 0);
}

#[test]
fn test_read_captures_at_non_matching_length() {
    let text: &[u8] = &[b'a', b'b'];
    let mut locs = Locations(vec![None, None, None]);
    let exec = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec![String::from("ab")],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::Literal(MatchLiteralType::Unanchored),
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    exec.read_captures_at(&mut locs, text, 0);
}

#[test]
fn test_read_captures_at_anchor_end() {
    let text: &[u8] = &[b'a', b'b'];
    let mut locs = Locations(vec![None, None]);
    let exec = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec![String::from("ab")],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::Literal(MatchLiteralType::AnchoredEnd),
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    exec.read_captures_at(&mut locs, text, 0);
}

#[test]
fn test_read_captures_at_anchored_start() {
    let text: &[u8] = &[b'a', b'b'];
    let mut locs = Locations(vec![None, None]);
    let exec = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec![String::from("ab")],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::Literal(MatchLiteralType::AnchoredStart),
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    exec.read_captures_at(&mut locs, text, 0);
}

