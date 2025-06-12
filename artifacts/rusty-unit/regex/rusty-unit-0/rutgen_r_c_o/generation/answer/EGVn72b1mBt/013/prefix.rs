// Answer 0

#[test]
fn test_step_match_success() {
    let prog = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![InstPtr(0)],
        captures: vec![Some("group".to_string())],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr(0),
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 10,
    };
    let mut slots = vec![Slot::default(), Slot::default()];
    let mut matches = vec![false; 1];
    let nlist = Threads {
        set: SparseSet::default(),
        caps: vec![Slot::default()],
        slots_per_thread: 2,
    };
    let at = InputAt { pos: 0, c: Char(97), byte: Some(97), len: 1 };
    let at_next = InputAt { pos: 1, c: Char(98), byte: Some(98), len: 1 };
    
    let mut fsm = Fsm { prog: &prog, stack: &mut vec![], input: vec![] };
    fsm.step(&mut nlist, &mut matches, &mut slots, &mut vec![None; 2], 0, at, at_next);
}

#[test]
fn test_step_match_no_success() {
    let prog = Program {
        insts: vec![Inst::Match(3)],
        matches: vec![InstPtr(0)],
        captures: vec![Some("group".to_string())],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr(0),
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 10,
    };
    let mut slots = vec![Slot::default(), Slot::default()];
    let mut matches = vec![false; 3];
    let nlist = Threads {
        set: SparseSet::default(),
        caps: vec![Slot::default()],
        slots_per_thread: 2,
    };
    let at = InputAt { pos: 0, c: Char(97), byte: Some(97), len: 1 };
    let at_next = InputAt { pos: 1, c: Char(98), byte: Some(98), len: 1 };

    let mut fsm = Fsm { prog: &prog, stack: &mut vec![], input: vec![] };
    fsm.step(&mut nlist, &mut matches, &mut slots, &mut vec![None; 2], 0, at, at_next);
}

#[test]
#[should_panic]
fn test_step_match_slot_out_of_bounds() {
    let prog = Program {
        insts: vec![Inst::Match(3)],
        matches: vec![InstPtr(0)],
        captures: vec![Some("group".to_string())],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr(0),
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 10,
    };
    let mut slots = vec![Slot::default(), Slot::default()];
    let mut matches = vec![false; 3];
    let nlist = Threads {
        set: SparseSet::default(),
        caps: vec![Slot::default()],
        slots_per_thread: 2,
    };
    let at = InputAt { pos: 0, c: Char(97), byte: Some(97), len: 1 };
    let at_next = InputAt { pos: 1, c: Char(98), byte: Some(98), len: 1 };

    let mut fsm = Fsm { prog: &prog, stack: &mut vec![], input: vec![] };
    fsm.step(&mut nlist, &mut matches, &mut slots, &mut vec![None; 2], 0, at, at_next);
}

#[test]
fn test_step_with_slot_and_val() {
    let prog = Program {
        insts: vec![Inst::Match(1), Inst::Match(0)],
        matches: vec![InstPtr(0), InstPtr(1)],
        captures: vec![Some("group1".to_string()), Some("group2".to_string())],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr(0),
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 10,
    };
    let mut slots = vec![Slot::default(), Slot::default()];
    let mut matches = vec![false; 2];
    let nlist = Threads {
        set: SparseSet::default(),
        caps: vec![Slot::default(), Slot::default()],
        slots_per_thread: 2,
    };
    let at = InputAt { pos: 0, c: Char(97), byte: Some(97), len: 1 };
    let at_next = InputAt { pos: 1, c: Char(98), byte: Some(98), len: 1 };

    let mut fsm = Fsm { prog: &prog, stack: &mut vec![], input: vec![] };
    fsm.step(&mut nlist, &mut matches, &mut slots, &mut vec![Some(0), Some(1)], 1, at, at_next);
}

