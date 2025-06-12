// Answer 0

#[test]
fn test_step_empty_look() {
    let program = Program {
        insts: vec![Inst::EmptyLook(InstEmptyLook {}); 5],
        matches: vec![InstPtr::default(); 1],
        captures: vec![None; 0],
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
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let mut stack = vec![];
    let input = Fsm {
        prog: &program,
        stack: &mut stack,
        input: vec![],
    };

    let mut nlist = Threads {
        set: SparseSet::new(),
        caps: vec![],
        slots_per_thread: 0,
    };

    let mut matches = vec![false; 1];
    let mut slots = vec![Slot::default(); 0];
    let mut thread_caps = vec![None; 0];

    let at = InputAt { pos: 0, c: Char(0), byte: None, len: 1 };
    let at_next = InputAt { pos: 1, c: Char(0), byte: None, len: 1 };

    input.step(&mut nlist, &mut matches, &mut slots, &mut thread_caps, 0, at, at_next);
}

