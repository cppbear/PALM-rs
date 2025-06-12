// Answer 0

#[test]
fn test_follow_epsilons_with_start_text_false() {
    let inst_empty_look = prog::Inst::EmptyLook(prog::InstEmptyLook {
        goto: 1,
        look: prog::EmptyLook::StartText,
    });
    
    let program = regex::Program {
        insts: vec![inst_empty_look, inst_empty_look],
        matches: vec![],
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
        prefixes: regex::LiteralSearcher::default(),
        dfa_size_limit: 1024,
    };

    let mut cache_inner = regex::CacheInner {
        compiled: HashMap::new(),
        trans: regex::Transitions::new(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut fsm = regex::Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: regex::STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    let mut sparse_set = regex::SparseSet::new(32);
    let flags = regex::EmptyFlags {
        start: false,
        end: false,
        start_line: false,
        end_line: false,
        word_boundary: false,
        not_word_boundary: false,
    };

    fsm.follow_epsilons(0, &mut sparse_set, flags);
}

#[test]
fn test_follow_epsilons_with_multiple_empty_look() {
    let inst_empty_look_start = prog::Inst::EmptyLook(prog::InstEmptyLook {
        goto: 2,
        look: prog::EmptyLook::StartText,
    });

    let inst_empty_look_bypass = prog::Inst::EmptyLook(prog::InstEmptyLook {
        goto: 1,
        look: prog::EmptyLook::EndLine,
    });

    let program = regex::Program {
        insts: vec![inst_empty_look_start, inst_empty_look_bypass],
        matches: vec![],
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
        prefixes: regex::LiteralSearcher::default(),
        dfa_size_limit: 1024,
    };

    let mut cache_inner = regex::CacheInner {
        compiled: HashMap::new(),
        trans: regex::Transitions::new(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut fsm = regex::Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: regex::STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    let mut sparse_set = regex::SparseSet::new(32);
    let flags = regex::EmptyFlags {
        start: false,
        end: false,
        start_line: false,
        end_line: false,
        word_boundary: false,
        not_word_boundary: false,
    };

    fsm.follow_epsilons(0, &mut sparse_set, flags);
}

#[test]
fn test_follow_epsilons_with_flags_zero() {
    let inst_empty_look = prog::Inst::EmptyLook(prog::InstEmptyLook {
        goto: 3,
        look: prog::EmptyLook::StartText,
    });

    let program = regex::Program {
        insts: vec![inst_empty_look, inst_empty_look, inst_empty_look],
        matches: vec![],
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
        prefixes: regex::LiteralSearcher::default(),
        dfa_size_limit: 1024,
    };

    let mut cache_inner = regex::CacheInner {
        compiled: HashMap::new(),
        trans: regex::Transitions::new(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut fsm = regex::Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: regex::STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    let mut sparse_set = regex::SparseSet::new(32);
    let flags = regex::EmptyFlags {
        start: false,
        end: false,
        start_line: false,
        end_line: false,
        word_boundary: false,
        not_word_boundary: false,
    };

    fsm.follow_epsilons(1, &mut sparse_set, flags);
}

