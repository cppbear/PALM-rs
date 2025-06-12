// Answer 0

#[test]
fn test_add_step_when_ip_points_to_save_with_no_exceeding_slot() {
    let program = Program {
        insts: vec![Inst::Save(InstSave { goto: 1, slot: 0 })],
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
        dfa_size_limit: 10,
    };

    let mut stack = Vec::new();
    let input = MockInput {};
    let mut nlist = Threads::new();
    nlist.resize(1, 1);
    let mut thread_caps = vec![None];

    let mut fsm = Fsm {
        prog: &program,
        stack: &mut stack,
        input,
    };

    fsm.add_step(&mut nlist, &mut thread_caps, 0, InputAt { pos: 0, c: Char::from('a'), byte: None, len: 1 });
}

#[test]
fn test_add_step_when_ip_is_already_in_nlist() {
    let program = Program {
        insts: vec![Inst::Save(InstSave { goto: 1, slot: 0 })],
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
        dfa_size_limit: 10,
    };

    let mut stack = Vec::new();
    let input = MockInput {};
    let mut nlist = Threads::new();
    nlist.resize(1, 1);
    nlist.set.insert(0); // Already contains the instruction pointer
    let mut thread_caps = vec![Some(0)];

    let mut fsm = Fsm {
        prog: &program,
        stack: &mut stack,
        input,
    };

    fsm.add_step(&mut nlist, &mut thread_caps, 0, InputAt { pos: 0, c: Char::from('a'), byte: None, len: 1 });
}

#[test]
fn test_add_step_when_slot_exceeds_thread_caps() {
    let program = Program {
        insts: vec![Inst::Save(InstSave { goto: 1, slot: 1 })],
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
        dfa_size_limit: 10,
    };

    let mut stack = Vec::new();
    let input = MockInput {};
    let mut nlist = Threads::new();
    nlist.resize(1, 1);
    let mut thread_caps: Vec<Option<usize>> = vec![None]; // Ensure slot is out of bounds

    let mut fsm = Fsm {
        prog: &program,
        stack: &mut stack,
        input,
    };

    fsm.add_step(&mut nlist, &mut thread_caps, 0, InputAt { pos: 0, c: Char::from('a'), byte: None, len: 1 });
}

