// Answer 0

#[test]
fn test_forward_many_case_1() {
    use std::sync::Arc;
    use std::collections::HashMap;

    let mut matches = vec![false];
    let insts = vec![Inst::Match(1), Inst::Match(0)];
    let capture_name_idx = Arc::new(HashMap::new());
    let program = Program {
        insts,
        matches: vec![1],
        captures: vec![],
        capture_name_idx,
        start: 0,
        byte_classes: vec![b'a'],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 100,
    };

    let text = b"abcde";
    let at = 0;

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![State { data: Box::new([0]) }],
        start_states: vec![0],
        stack: vec![0],
        flush_count: 0,
        size: 0,
    };
    let cache = ProgramCacheInner {
        pikevm: pikevm::Cache::new(),
        backtrack: backtrack::Cache::new(),
        dfa: dfa::Cache::new(),
        dfa_reverse: dfa::Cache::new(),
    };
    let result = Fsm::forward_many(&program, &cache, &mut matches, text, at);
}

#[test]
fn test_forward_many_case_2() {
    use std::sync::Arc;
    use std::collections::HashMap;

    let mut matches = vec![false];
    let insts = vec![Inst::Match(1), Inst::Char(InstChar { c: b'a' })];
    let capture_name_idx = Arc::new(HashMap::new());
    let program = Program {
        insts,
        matches: vec![1],
        captures: vec![],
        capture_name_idx,
        start: 0,
        byte_classes: vec![b'a', b'b'],
        only_utf8: true,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 50,
    };

    let text = b"abcde";
    let at = 0;

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![State { data: Box::new([0]) }],
        start_states: vec![0],
        stack: vec![0],
        flush_count: 0,
        size: 0,
    };
    let cache = ProgramCacheInner {
        pikevm: pikevm::Cache::new(),
        backtrack: backtrack::Cache::new(),
        dfa: dfa::Cache::new(),
        dfa_reverse: dfa::Cache::new(),
    };
    let result = Fsm::forward_many(&program, &cache, &mut matches, text, at);
}

#[test]
fn test_forward_many_case_3() {
    use std::sync::Arc;
    use std::collections::HashMap;

    let mut matches = vec![false];
    let insts = vec![Inst::Match(1)];
    let capture_name_idx = Arc::new(HashMap::new());
    let program = Program {
        insts,
        matches: vec![1],
        captures: vec![],
        capture_name_idx,
        start: 0,
        byte_classes: vec![b'a'],
        only_utf8: false,
        is_bytes: true,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: true,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 10,
    };

    let text = b"abcde";
    let at = 0;

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![State { data: Box::new([0]) }],
        start_states: vec![0],
        stack: vec![0],
        flush_count: 0,
        size: 0,
    };
    let cache = ProgramCacheInner {
        pikevm: pikevm::Cache::new(),
        backtrack: backtrack::Cache::new(),
        dfa: dfa::Cache::new(),
        dfa_reverse: dfa::Cache::new(),
    };
    let result = Fsm::forward_many(&program, &cache, &mut matches, text, at);
}

