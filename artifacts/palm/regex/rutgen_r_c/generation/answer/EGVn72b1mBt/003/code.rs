// Answer 0

#[test]
fn test_step_with_bytes_not_matching() {
    use prog::Inst;

    let mut threads_nlist = Threads {
        set: SparseSet::new(),
        caps: Vec::new(),
        slots_per_thread: 0,
    };

    let matches: &mut [bool] = &mut [false];
    let slots: &mut [Slot] = &mut [Slot::default()];
    let thread_caps: &mut [Option<usize>] = &mut [None];
    
    let byte_to_match = 100; // Example byte to test
    let inst_ptr = 0; // Pointer to the instruction in program

    let inst_bytes = InstBytes {
        goto: inst_ptr, // Will not be reached
        start: 101, // Start higher than the byte we're testing
        end: 200, // End higher than the start
    };

    let program = Program {
        insts: vec![Inst::Bytes(inst_bytes)],
        matches: vec![InstPtr(0)],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr(0),
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: true,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };

    let input_at = InputAt {
        pos: 0,
        c: Char(byte_to_match as u32),
        byte: Some(byte_to_match),
        len: 1,
    };

    let input_at_next = InputAt {
        pos: 1,
        c: Char(byte_to_match as u32),
        byte: None,
        len: 0,
    };

    let mut fsm = Fsm {
        prog: &program,
        stack: &mut vec![],
        input: (),
    };

    let result = fsm.step(
        &mut threads_nlist,
        matches,
        slots,
        thread_caps,
        inst_ptr,
        input_at,
        input_at_next,
    );

    assert_eq!(result, false);
    assert_eq!(matches[0], false);
}

