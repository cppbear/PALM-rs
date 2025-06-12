// Answer 0

#[test]
fn test_read_captures_at_empty_slots() {
    let text = b"";
    let start = 0;
    let locs = Locations(vec![]);
    let exec = ExecNoSync { ro: &Arc::new(ExecReadOnly { res: Vec::new(), nfa: Program::default(), dfa: Program::default(), dfa_reverse: Program::default(), suffixes: LiteralSearcher::default(), match_type: MatchType::Nothing }), cache: &RefCell::new(ProgramCacheInner::default()) };
    exec.read_captures_at(&mut locs, text, start);
}

#[test]
fn test_read_captures_at_two_slots() {
    let text = b"hello world";
    let start = 0;
    let mut locs = Locations(vec![None, None]);
    let exec = ExecNoSync { ro: &Arc::new(ExecReadOnly { res: vec!["hello".to_string()], nfa: Program::default(), dfa: Program::default(), dfa_reverse: Program::default(), suffixes: LiteralSearcher::default(), match_type: MatchType::Literal(MatchLiteralType::Unanchored) }), cache: &RefCell::new(ProgramCacheInner::default()) };
    exec.read_captures_at(&mut locs, text, start);
}

#[test]
fn test_read_captures_at_three_slots() {
    let text = b"hello world";
    let start = 0;
    let mut locs = Locations(vec![None, None, None]);
    let exec = ExecNoSync { ro: &Arc::new(ExecReadOnly { res: vec!["hello".to_string()], nfa: Program::default(), dfa: Program::default(), dfa_reverse: Program::default(), suffixes: LiteralSearcher::default(), match_type: MatchType::Literal(MatchLiteralType::Unanchored) }), cache: &RefCell::new(ProgramCacheInner::default()) };
    exec.read_captures_at(&mut locs, text, start);
}

#[test]
fn test_read_captures_at_five_slots() {
    let text = b"hello world";
    let start = 0;
    let mut locs = Locations(vec![None, None, None, None, None]);
    let exec = ExecNoSync { ro: &Arc::new(ExecReadOnly { res: vec!["hello world".to_string()], nfa: Program::default(), dfa: Program::default(), dfa_reverse: Program::default(), suffixes: LiteralSearcher::default(), match_type: MatchType::Dfa }), cache: &RefCell::new(ProgramCacheInner::default()) };
    exec.read_captures_at(&mut locs, text, start);
}

#[test]
fn test_read_captures_at_no_match() {
    let text = b"goodbye";
    let start = 0;
    let mut locs = Locations(vec![None]);
    let exec = ExecNoSync { ro: &Arc::new(ExecReadOnly { res: vec!["hello".to_string()], nfa: Program::default(), dfa: Program::default(), dfa_reverse: Program::default(), suffixes: LiteralSearcher::default(), match_type: MatchType::Nothing }), cache: &RefCell::new(ProgramCacheInner::default()) };
    exec.read_captures_at(&mut locs, text, start);
}

