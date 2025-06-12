// Answer 0

#[test]
fn test_exec_nfa_auto_to_backtrack() {
    let matches = &mut [false; 10];
    let slots = &mut [Slot::default(); 5];
    let text: &[u8] = &[b'a', b'b', b'c'];
    let start = 0;

    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec!["a".to_string()],
            nfa: Program {
                insts: vec![Inst::Match(0)],
                matches: vec![],
                captures: vec![],
                capture_name_idx: Arc::new(HashMap::new()),
                start: InstPtr::default(),
                byte_classes: vec![],
                only_utf8: true,
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
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    exec_no_sync.exec_nfa(MatchNfaType::Auto, matches, slots, false, text, start);
}

#[test]
fn test_exec_nfa_pikevm() {
    let matches = &mut [false; 10];
    let slots = &mut [Slot::default(); 5];
    let text: &[u8] = &[b'a', b'b', b'c', b'a'];
    let start = 0;

    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec!["a".to_string()],
            nfa: Program {
                insts: vec![Inst::Match(0)],
                matches: vec![],
                captures: vec![],
                capture_name_idx: Arc::new(HashMap::new()),
                start: InstPtr::default(),
                byte_classes: vec![],
                only_utf8: false,
                is_bytes: true,
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
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    exec_no_sync.exec_nfa(MatchNfaType::PikeVM, matches, slots, false, text, start);
}

#[test]
fn test_exec_nfa_pikevm_edge_case() {
    let matches = &mut [false; 1];
    let slots = &mut [Slot::default(); 1];
    let text: &[u8] = &[b'a'];
    let start = 0;

    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec![".*".to_string()],
            nfa: Program {
                insts: vec![Inst::Char(InstChar::default())],
                matches: vec![],
                captures: vec![],
                capture_name_idx: Arc::new(HashMap::new()),
                start: InstPtr::default(),
                byte_classes: vec![],
                only_utf8: true,
                is_bytes: true,
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
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    exec_no_sync.exec_nfa(MatchNfaType::PikeVM, matches, slots, false, text, start);
}

