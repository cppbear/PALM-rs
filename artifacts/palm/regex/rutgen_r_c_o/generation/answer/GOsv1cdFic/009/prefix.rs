// Answer 0

#[test]
fn test_forward_many_no_match_empty_text() {
    let prog = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let mut cache = ProgramCacheInner {
        pikevm: pikevm::Cache::new(),
        backtrack: backtrack::Cache::new(),
        dfa: dfa::Cache::new(),
        dfa_reverse: dfa::Cache::new(),
    };

    let mut matches = vec![false];
    let text = b"";
    let at = 0;

    let result = Fsm::forward_many(&prog, &mut cache, &mut matches, text, at);
}

#[test]
fn test_forward_many_no_match_single_character_text() {
    let prog = Program {
        insts: vec![Inst::Match(1)],
        matches: vec![0, 1],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let mut cache = ProgramCacheInner {
        pikevm: pikevm::Cache::new(),
        backtrack: backtrack::Cache::new(),
        dfa: dfa::Cache::new(),
        dfa_reverse: dfa::Cache::new(),
    };

    let mut matches = vec![false, false];
    let text = b"a";
    let at = 0;

    let result = Fsm::forward_many(&prog, &mut cache, &mut matches, text, at);
}

#[test]
fn test_forward_many_no_match_two_characters_text() {
    let prog = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let mut cache = ProgramCacheInner {
        pikevm: pikevm::Cache::new(),
        backtrack: backtrack::Cache::new(),
        dfa: dfa::Cache::new(),
        dfa_reverse: dfa::Cache::new(),
    };

    let mut matches = vec![false, false];
    let text = b"ab";
    let at = 1;

    let result = Fsm::forward_many(&prog, &mut cache, &mut matches, text, at);
}

#[test]
fn test_forward_many_no_match_two_matches_single_character() {
    let prog = Program {
        insts: vec![Inst::Match(0), Inst::Match(1)],
        matches: vec![0, 1],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let mut cache = ProgramCacheInner {
        pikevm: pikevm::Cache::new(),
        backtrack: backtrack::Cache::new(),
        dfa: dfa::Cache::new(),
        dfa_reverse: dfa::Cache::new(),
    };

    let mut matches = vec![false, false];
    let text = b"b";
    let at = 0;

    let result = Fsm::forward_many(&prog, &mut cache, &mut matches, text, at);
}

#[test]
fn test_forward_many_no_match_two_matches_two_characters() {
    let prog = Program {
        insts: vec![Inst::Match(0), Inst::Match(1)],
        matches: vec![0, 1],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let mut cache = ProgramCacheInner {
        pikevm: pikevm::Cache::new(),
        backtrack: backtrack::Cache::new(),
        dfa: dfa::Cache::new(),
        dfa_reverse: dfa::Cache::new(),
    };

    let mut matches = vec![false, false];
    let text = b"cd";
    let at = 0;

    let result = Fsm::forward_many(&prog, &mut cache, &mut matches, text, at);
}

