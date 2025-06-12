// Answer 0

#[test]
fn test_follow_epsilons_with_split_instruction() {
    use std::sync::Arc;

    let inst_split1 = prog::Inst::Split(prog::InstSplit {
        goto1: 1,
        goto2: 2,
    });
    let inst_split2 = prog::Inst::Match(0);
    
    let instructions = vec![inst_split1, inst_split2];
    let matches = vec![0]; // Assuming some matches exist
    let captures = vec![None; 0]; // No captures for this test
    let capture_name_idx = Arc::new(HashMap::new());
    
    let program = Program {
        insts: instructions,
        matches,
        captures,
        capture_name_idx,
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
        dfa_size_limit: 1024,
    };

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
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

    let mut sparse_set = SparseSet::new(10);
    let flags = EmptyFlags {
        start_line: true,
        end_line: false,
        start: false,
        end: false,
        word_boundary: false,
        not_word_boundary: false,
    };

    fsm.follow_epsilons(0, &mut sparse_set, flags);

    assert!(sparse_set.contains(0)); // Ensure the first state (split instruction) was processed
    assert!(sparse_set.contains(1)); // Ensure the goto1 was followed
    assert!(!sparse_set.contains(2)); // Ensure the goto2 was not followed as we didn't trigger it
}

#[test]
fn test_follow_epsilons_with_flags() {
    use std::sync::Arc;

    let inst_split = prog::Inst::Split(prog::InstSplit {
        goto1: 1,
        goto2: 2,
    });
    let inst_empty_look = prog::Inst::EmptyLook(prog::InstEmptyLook {
        goto: 1,
        look: EmptyLook::StartLine,
    });
    
    let instructions = vec![inst_split, inst_empty_look];
    let matches = vec![0]; // Assuming some matches exist
    let captures = vec![None; 0]; // No captures for this test
    let capture_name_idx = Arc::new(HashMap::new());
    
    let program = Program {
        insts: instructions,
        matches,
        captures,
        capture_name_idx,
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
        dfa_size_limit: 1024,
    };

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
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

    let mut sparse_set = SparseSet::new(10);
    let flags = EmptyFlags {
        start_line: true,
        end_line: false,
        start: false,
        end: false,
        word_boundary: false,
        not_word_boundary: false,
    };

    fsm.follow_epsilons(0, &mut sparse_set, flags);

    assert!(sparse_set.contains(0)); // Ensure the split instruction was processed
    assert!(sparse_set.contains(1)); // Ensure the EmptyLook instruction was followed due to flags
}

