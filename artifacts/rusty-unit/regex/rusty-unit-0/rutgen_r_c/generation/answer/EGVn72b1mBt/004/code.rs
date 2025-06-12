// Answer 0

#[test]
fn test_step_with_ranges_success() {
    use sparse::SparseSet;
    
    // Create a character range for testing
    let ranges = vec![(Char(97), Char(122))]; // e.g., a-z
    let inst_ranges = InstRanges {
        goto: 1,
        ranges,
    };
    
    let program = Program {
        insts: vec![Inst::Ranges(inst_ranges), Inst::Match(0)],
        matches: vec![0],
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
        prefixes: Default::default(),
        dfa_size_limit: 10,
    };

    let input_at = InputAt {
        pos: 0,
        c: Char(100), // 'd'
        byte: Some(100),
        len: 1,
    };
    
    let input_at_next = InputAt {
        pos: 1,
        c: Char(0), // End of input
        byte: None,
        len: 0,
    };

    let mut nlist = Threads {
        set: SparseSet::new(),
        caps: vec![],
        slots_per_thread: 0,
    };
    
    let mut matches = vec![false];
    let mut slots = vec![];
    let mut thread_caps = vec![None];

    let mut fsm = Fsm {
        prog: &program,
        stack: &mut vec![],
        input: (),
    };

    let result = fsm.step(&mut nlist, &mut matches, &mut slots, &mut thread_caps, 0, input_at, input_at_next);
    
    assert_eq!(result, false);
    assert!(matches[0]);
}

