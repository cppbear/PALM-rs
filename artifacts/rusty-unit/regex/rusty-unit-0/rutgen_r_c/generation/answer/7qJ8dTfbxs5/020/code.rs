// Answer 0

#[test]
fn test_follow_epsilons_with_start_line_flag() {
    use std::collections::HashMap;
    use std::sync::Arc;

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let program = Program {
        insts: vec![
            Inst::EmptyLook(InstEmptyLook {
                goto: 1,
                look: EmptyLook::StartLine,
            }),
            Inst::Match(0),
        ],
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
        dfa_size_limit: 1024,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    let mut q = SparseSet::new(10);
    let flags = EmptyFlags {
        start: false,
        end: false,
        start_line: true,
        end_line: false,
        word_boundary: false,
        not_word_boundary: false,
    };

    // Push initial instruction pointer to stack
    fsm.cache.stack.push(0);
    fsm.follow_epsilons(0, &mut q, flags);

    assert!(q.contains(0));
}

#[test]
fn test_follow_epsilons_with_empty_stack() {
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let program = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1024,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    let mut q = SparseSet::new(10);
    let flags = EmptyFlags::default();

    // Empty stack should not panic and q should remain empty
    fsm.follow_epsilons(0, &mut q, flags);

    assert!(q.is_empty());
}

