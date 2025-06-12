// Answer 0

#[test]
fn test_exec_nfa_backtrack() {
    use std::collections::HashMap;
    use std::sync::Arc;

    let regex = Arc::new(ExecReadOnly {
        res: vec![String::from("test")],
        nfa: Program {
            insts: vec![Inst::Match(0)],
            matches: vec![],
            captures: vec![],
            capture_name_idx: Arc::new(HashMap::new()),
            start: InstPtr(0),
            byte_classes: vec![],
            only_utf8: false,
            is_bytes: false,
            is_dfa: false,
            is_reverse: false,
            is_anchored_start: false,
            is_anchored_end: false,
            has_unicode_word_boundary: false,
            prefixes: LiteralSearcher::default(),
            dfa_size_limit: 0,
        },
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });

    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync {
        ro: &regex,
        cache: &cache,
    };

    let mut matches = [false; 1];
    let mut slots = [Slot::default(); 1];
    let text = b"test text";
    let start = 0;

    let result = exec.exec_nfa(MatchNfaType::Backtrack, &mut matches, &mut slots, false, text, start);
    assert!(result);
}

#[test]
fn test_exec_nfa_pikevm() {
    use std::collections::HashMap;
    use std::sync::Arc;

    let regex = Arc::new(ExecReadOnly {
        res: vec![String::from("example")],
        nfa: Program {
            insts: vec![Inst::Match(0)],
            matches: vec![],
            captures: vec![],
            capture_name_idx: Arc::new(HashMap::new()),
            start: InstPtr(0),
            byte_classes: vec![],
            only_utf8: true,
            is_bytes: false,
            is_dfa: false,
            is_reverse: false,
            is_anchored_start: true,
            is_anchored_end: false,
            has_unicode_word_boundary: false,
            prefixes: LiteralSearcher::default(),
            dfa_size_limit: 0,
        },
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });

    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync {
        ro: &regex,
        cache: &cache,
    };

    let mut matches = [false; 1];
    let mut slots = [Slot::default(); 1];
    let text = b"example text";
    let start = 0;

    let result = exec.exec_nfa(MatchNfaType::PikeVM, &mut matches, &mut slots, false, text, start);
    assert!(result);
}

