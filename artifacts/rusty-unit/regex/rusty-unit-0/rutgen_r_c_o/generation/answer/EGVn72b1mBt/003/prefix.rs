// Answer 0

#[test]
fn test_step_with_bytes_match_false() {
    let program = Program {
        insts: vec![
            Inst::Bytes(InstBytes {
                goto: 1,
                start: 2,
                end: 5,
            }),
            Inst::Match(0),
        ],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: true,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let mut stack = vec![];
    let input_at = InputAt {
        pos: 0,
        c: Char(0),
        byte: Some(1),
        len: 1,
    };
    let input_at_next = InputAt {
        pos: 1,
        c: Char(1),
        byte: Some(2),
        len: 1,
    };

    let mut nlist = Threads {
        set: SparseSet::new(),
        caps: vec![],
        slots_per_thread: 0,
    };
    
    let mut matches = vec![false];
    let mut slots: Vec<Option<usize>> = vec![None; 2];
    let mut thread_caps: Vec<Option<usize>> = vec![None; 2];

    let mut fsm = Fsm {
        prog: &program,
        stack: &mut stack,
        input: (),
    };

    let result = fsm.step(&mut nlist, &mut matches, &mut slots, &mut thread_caps, 0, input_at, input_at_next);
}

