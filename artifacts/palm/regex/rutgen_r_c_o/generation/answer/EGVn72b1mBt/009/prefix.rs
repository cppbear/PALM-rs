// Answer 0

#[test]
fn test_step_with_split_instruction() {
    let program = Program {
        insts: vec![Inst::Split(InstSplit { goto1: 1, goto2: 2 })],
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
        dfa_size_limit: 0,
    };

    let mut stack = vec![];
    let input_at = InputAt { pos: 0, c: Char(97), byte: Some(97), len: 1 };
    let input_at_next = InputAt { pos: 1, c: Char(98), byte: Some(98), len: 1 };

    let mut threads = Threads {
        set: SparseSet::new(),
        caps: vec![],
        slots_per_thread: 0,
    };
    
    let mut matches = vec![false; 0];
    let mut slots = vec![];
    let mut thread_caps = vec![None; 0];

    let mut fsm = Fsm {
        prog: &program,
        stack: &mut stack,
        input: (),
    };

    fsm.step(&mut threads, &mut matches, &mut slots, &mut thread_caps, 0, input_at, input_at_next);
}

#[test]
fn test_step_with_empty_lookup_instruction() {
    let program = Program {
        insts: vec![Inst::EmptyLook(InstEmptyLook {})],
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
        dfa_size_limit: 0,
    };

    let mut stack = vec![];
    let input_at = InputAt { pos: 0, c: Char(97), byte: Some(97), len: 1 };
    let input_at_next = InputAt { pos: 1, c: Char(98), byte: Some(98), len: 1 };

    let mut threads = Threads {
        set: SparseSet::new(),
        caps: vec![],
        slots_per_thread: 0,
    };
    
    let mut matches = vec![false; 0];
    let mut slots = vec![];
    let mut thread_caps = vec![None; 0];

    let mut fsm = Fsm {
        prog: &program,
        stack: &mut stack,
        input: (),
    };

    fsm.step(&mut threads, &mut matches, &mut slots, &mut thread_caps, 0, input_at, input_at_next);
}

#[test]
fn test_step_with_save_instruction() {
    let program = Program {
        insts: vec![Inst::Save(InstSave {})],
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
        dfa_size_limit: 0,
    };

    let mut stack = vec![];
    let input_at = InputAt { pos: 0, c: Char(97), byte: Some(97), len: 1 };
    let input_at_next = InputAt { pos: 1, c: Char(98), byte: Some(98), len: 1 };

    let mut threads = Threads {
        set: SparseSet::new(),
        caps: vec![],
        slots_per_thread: 0,
    };
    
    let mut matches = vec![false; 0];
    let mut slots = vec![];
    let mut thread_caps = vec![None; 0];

    let mut fsm = Fsm {
        prog: &program,
        stack: &mut stack,
        input: (),
    };

    fsm.step(&mut threads, &mut matches, &mut slots, &mut thread_caps, 0, input_at, input_at_next);
}

#[test]
fn test_step_with_ranges_instruction() {
    let program = Program {
        insts: vec![Inst::Ranges(InstRanges { ranges: vec![(Char(97), Char(122))] })],
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
        dfa_size_limit: 0,
    };

    let mut stack = vec![];
    let input_at = InputAt { pos: 0, c: Char(100), byte: Some(100), len: 1 };
    let input_at_next = InputAt { pos: 1, c: Char(101), byte: Some(101), len: 1 };

    let mut threads = Threads {
        set: SparseSet::new(),
        caps: vec![],
        slots_per_thread: 0,
    };
    
    let mut matches = vec![false; 0];
    let mut slots = vec![];
    let mut thread_caps = vec![None; 0];

    let mut fsm = Fsm {
        prog: &program,
        stack: &mut stack,
        input: (),
    };

    fsm.step(&mut threads, &mut matches, &mut slots, &mut thread_caps, 0, input_at, input_at_next);
}

#[test]
fn test_step_with_bytes_instruction() {
    let program = Program {
        insts: vec![Inst::Bytes(InstBytes { start: 97, end: 100, goto: 1 })],
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
        dfa_size_limit: 0,
    };

    let mut stack = vec![];
    let input_at = InputAt { pos: 0, c: Char(99), byte: Some(99), len: 1 };
    let input_at_next = InputAt { pos: 1, c: Char(100), byte: Some(100), len: 1 };

    let mut threads = Threads {
        set: SparseSet::new(),
        caps: vec![],
        slots_per_thread: 0,
    };
    
    let mut matches = vec![false; 0];
    let mut slots = vec![];
    let mut thread_caps = vec![None; 0];

    let mut fsm = Fsm {
        prog: &program,
        stack: &mut stack,
        input: (),
    };

    fsm.step(&mut threads, &mut matches, &mut slots, &mut thread_caps, 0, input_at, input_at_next);
}

