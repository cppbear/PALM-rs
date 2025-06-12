// Answer 0

#[test]
fn test_follow_epsilons_with_word_boundary() {
    use std::collections::HashMap;
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: vec![0; 256], 
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };
    
    let prog = Program {
        insts: vec![
            Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::WordBoundaryAscii }),
            Inst::EmptyLook(InstEmptyLook { goto: 2, look: EmptyLook::NotWordBoundary }),
            Inst::Match(0),
        ],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: HashMap::new(),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let mut q = SparseSet::new(10);
    let flags = EmptyFlags {
        start: false,
        end: false,
        start_line: false,
        end_line: false,
        word_boundary: true,
        not_word_boundary: false,
    };

    // Initial state to follow
    let ip: InstPtr = 0;
    fsm.cache.stack.push(ip);
    fsm.follow_epsilons(ip, &mut q, flags);

    assert!(q.contains(0)); // State at ip should have been added to q
}

#[test]
fn test_follow_epsilons_no_word_boundary() {
    use std::collections::HashMap;
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: vec![0; 256], 
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };
    
    let prog = Program {
        insts: vec![
            Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::WordBoundaryAscii }),
            Inst::EmptyLook(InstEmptyLook { goto: 2, look: EmptyLook::NotWordBoundary }),
            Inst::Match(0),
        ],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: HashMap::new(),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let mut q = SparseSet::new(10);
    let flags = EmptyFlags {
        start: false,
        end: false,
        start_line: false,
        end_line: false,
        word_boundary: false,
        not_word_boundary: false,
    };

    // Initial state to follow
    let ip: InstPtr = 0;
    fsm.cache.stack.push(ip);
    fsm.follow_epsilons(ip, &mut q, flags);

    assert!(!q.contains(0)); // State at ip should NOT have been added to q
}

#[test]
#[should_panic]
fn test_follow_epsilons_out_of_bounds() {
    use std::collections::HashMap;
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: vec![0; 256],
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };
    
    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: HashMap::new(),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let mut q = SparseSet::new(10);
    let flags = EmptyFlags::default();

    // Trying to follow a non-existent instruction
    let ip: InstPtr = 0; // Index out of bounds since insts is empty
    fsm.cache.stack.push(ip);
    fsm.follow_epsilons(ip, &mut q, flags);
}

