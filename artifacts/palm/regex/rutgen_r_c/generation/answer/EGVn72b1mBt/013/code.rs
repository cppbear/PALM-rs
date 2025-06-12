// Answer 0

#[test]
fn test_step_match_slot_valid() {
    let program = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![InstPtr::default()],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr::default(),
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1,
    };
    
    let mut matches = vec![false; 1];
    let mut slots = vec![];
    let thread_caps: Vec<Option<usize>> = vec![];
    
    let input_at = InputAt {
        pos: 0,
        c: Char(0),
        byte: None,
        len: 1,
    };
    
    let input_at_next = InputAt {
        pos: 1,
        c: Char(1),
        byte: None,
        len: 1,
    };
    
    let mut threads = Threads {
        set: SparseSet::default(),
        caps: vec![],
        slots_per_thread: 0,
    };
    
    let mut fsm = Fsm {
        prog: &program,
        stack: &mut vec![],
        input: (),
    };

    assert_eq!(fsm.step(&mut threads, &mut matches, &mut slots, &mut thread_caps[0..], 0, input_at, input_at_next), true);
}

#[test]
#[should_panic]
fn test_step_match_slot_out_of_bounds() {
    let program = Program {
        insts: vec![Inst::Match(1)], // match_slot is 1, which exceeds matches.len()
        matches: vec![InstPtr::default()],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr::default(),
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1,
    };
    
    let mut matches = vec![false; 1];
    let mut slots = vec![];
    let thread_caps: Vec<Option<usize>> = vec![];
    
    let input_at = InputAt {
        pos: 0,
        c: Char(0),
        byte: None,
        len: 1,
    };
    
    let input_at_next = InputAt {
        pos: 1,
        c: Char(1),
        byte: None,
        len: 1,
    };
    
    let mut threads = Threads {
        set: SparseSet::default(),
        caps: vec![],
        slots_per_thread: 0,
    };
    
    let mut fsm = Fsm {
        prog: &program,
        stack: &mut vec![],
        input: (),
    };

    fsm.step(&mut threads, &mut matches, &mut slots, &mut thread_caps[0..], 0, input_at, input_at_next);
}

#[test]
fn test_step_capture_slot_valid() {
    let program = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![InstPtr::default()],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr::default(),
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1,
    };

    let mut matches = vec![false; 1];
    let mut slots = vec![];
    let thread_caps = vec![Some(0)];
    
    let input_at = InputAt {
        pos: 0,
        c: Char(0),
        byte: None,
        len: 1,
    };
    
    let input_at_next = InputAt {
        pos: 1,
        c: Char(1),
        byte: None,
        len: 1,
    };
    
    let mut threads = Threads {
        set: SparseSet::default(),
        caps: vec![],
        slots_per_thread: 1,
    };
    
    let mut fsm = Fsm {
        prog: &program,
        stack: &mut vec![],
        input: (),
    };

    fsm.step(&mut threads, &mut matches, &mut slots, &mut thread_caps, 0, input_at, input_at_next);
}

#[test]
fn test_step_capture_slot_invalid() {
    let program = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![InstPtr::default()],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr::default(),
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1,
    };
    
    let mut matches = vec![false; 1];
    let mut slots = vec![];
    let thread_caps = vec![None];

    let input_at = InputAt {
        pos: 0,
        c: Char(0),
        byte: None,
        len: 1,
    };
    
    let input_at_next = InputAt {
        pos: 1,
        c: Char(1),
        byte: None,
        len: 1,
    };

    let mut threads = Threads {
        set: SparseSet::default(),
        caps: vec![],
        slots_per_thread: 1,
    };

    let mut fsm = Fsm {
        prog: &program,
        stack: &mut vec![],
        input: (),
    };

    assert_eq!(fsm.step(&mut threads, &mut matches, &mut slots, &mut thread_caps, 0, input_at, input_at_next), false);
}

