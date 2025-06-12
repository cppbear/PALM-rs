// Answer 0

#[test]
fn test_follow_epsilons_case1() {
    let mut program = Program {
        insts: vec![Inst::Split(InstSplit { goto1: 1, goto2: 2 }), Inst::Match(0), Inst::Bytes(InstBytes { start: 0, end: 0 })],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0; 256],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 4096,
    };
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![],
        start_states: vec![],
        stack: vec![0],
        flush_count: 0,
        size: 0,
    };
    let mut empty_flags = EmptyFlags::default();
    empty_flags.start = true;

    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let mut q = SparseSet::new(1024);
    let ip: InstPtr = 0;

    fsm.follow_epsilons(ip, &mut q, empty_flags);
}

#[test]
fn test_follow_epsilons_case2() {
    let mut program = Program {
        insts: vec![Inst::Split(InstSplit { goto1: 3, goto2: 4 }), Inst::Match(0), Inst::Bytes(InstBytes { start: 0, end: 0 }), Inst::Split(InstSplit { goto1: 2, goto2: 1 })],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0; 256],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 4096,
    };
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![],
        start_states: vec![],
        stack: vec![1],
        flush_count: 0,
        size: 0,
    };
    let mut empty_flags = EmptyFlags::default();
    empty_flags.end = true;

    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let mut q = SparseSet::new(1024);
    let ip: InstPtr = 0;

    fsm.follow_epsilons(ip, &mut q, empty_flags);
} 

#[test]
fn test_follow_epsilons_case3() {
    let mut program = Program {
        insts: vec![Inst::Split(InstSplit { goto1: 1, goto2: 2 }), Inst::Split(InstSplit { goto1: 3, goto2: 4 }), Inst::Match(0)],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0; 256],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 4096,
    };
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![],
        start_states: vec![],
        stack: vec![1],
        flush_count: 0,
        size: 0,
    };
    let mut empty_flags = EmptyFlags::default();
    empty_flags.word_boundary = true;

    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let mut q = SparseSet::new(1024);
    let ip: InstPtr = 0;

    fsm.follow_epsilons(ip, &mut q, empty_flags);
} 

#[test]
fn test_follow_epsilons_case4() {
    let mut program = Program {
        insts: vec![Inst::Split(InstSplit { goto1: 5, goto2: 6 }), Inst::Match(1), Inst::Bytes(InstBytes { start: 0, end: 0 })],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0; 256],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 4096,
    };
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![],
        start_states: vec![],
        stack: vec![0],
        flush_count: 0,
        size: 0,
    };
    let mut empty_flags = EmptyFlags::default();
    empty_flags.start_line = true;

    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let mut q = SparseSet::new(1024);
    let ip: InstPtr = 0;

    fsm.follow_epsilons(ip, &mut q, empty_flags);
} 

#[test]
fn test_follow_epsilons_case5() {
    let mut program = Program {
        insts: vec![Inst::Split(InstSplit { goto1: 2, goto2: 3 }), Inst::Match(0)],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0; 256],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 4096,
    };
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![],
        start_states: vec![],
        stack: vec![3],
        flush_count: 0,
        size: 0,
    };
    let mut empty_flags = EmptyFlags::default();
    empty_flags.not_word_boundary = true;

    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let mut q = SparseSet::new(1024);
    let ip: InstPtr = 1;

    fsm.follow_epsilons(ip, &mut q, empty_flags);
} 

