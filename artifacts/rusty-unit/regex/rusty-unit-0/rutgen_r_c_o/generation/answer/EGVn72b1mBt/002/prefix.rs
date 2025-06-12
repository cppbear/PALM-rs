// Answer 0

#[test]
fn test_step_with_bytes_inst_and_matching_byte() {
    let mut matches = vec![false; 2];
    let mut slots = vec![None; 2];
    let mut thread_caps = vec![None; 2];
    
    let prog = Program {
        insts: vec![Inst::Bytes(InstBytes { goto: 1, start: 100, end: 200 })],
        matches: vec![0],
        captures: vec![None; 2],
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
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 10,
    };
    
    let input_at = InputAt { pos: 0, c: Char(100), byte: Some(150), len: 1 };
    let next_input_at = InputAt { pos: 1, c: Char(101), byte: Some(151), len: 1 };
    
    let mut fsm = Fsm { prog: &prog, stack: &mut vec![], input: () };
    fsm.step(&mut Threads { set: SparseSet::default(), caps: vec![], slots_per_thread: 1 }, 
              &mut matches, &mut slots, &mut thread_caps, 0, 
              input_at, next_input_at);
}

#[test]
fn test_step_with_bytes_inst_and_non_matching_byte() {
    let mut matches = vec![false; 2];
    let mut slots = vec![None; 2];
    let mut thread_caps = vec![None; 2];
    
    let prog = Program {
        insts: vec![Inst::Bytes(InstBytes { goto: 1, start: 100, end: 200 })],
        matches: vec![0],
        captures: vec![None; 2],
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
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 10,
    };
    
    let input_at = InputAt { pos: 0, c: Char(100), byte: Some(250), len: 1 };
    let next_input_at = InputAt { pos: 1, c: Char(101), byte: Some(251), len: 1 };
    
    let mut fsm = Fsm { prog: &prog, stack: &mut vec![], input: () };
    fsm.step(&mut Threads { set: SparseSet::default(), caps: vec![], slots_per_thread: 1 }, 
              &mut matches, &mut slots, &mut thread_caps, 0, 
              input_at, next_input_at);
}

#[test]
fn test_step_with_bytes_inst_and_no_byte() {
    let mut matches = vec![false; 2];
    let mut slots = vec![None; 2];
    let mut thread_caps = vec![None; 2];
    
    let prog = Program {
        insts: vec![Inst::Bytes(InstBytes { goto: 1, start: 100, end: 200 })],
        matches: vec![0],
        captures: vec![None; 2],
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
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 10,
    };
    
    let input_at = InputAt { pos: 0, c: Char(100), byte: None, len: 1 };
    let next_input_at = InputAt { pos: 1, c: Char(101), byte: None, len: 1 };
    
    let mut fsm = Fsm { prog: &prog, stack: &mut vec![], input: () };
    fsm.step(&mut Threads { set: SparseSet::default(), caps: vec![], slots_per_thread: 1 }, 
              &mut matches, &mut slots, &mut thread_caps, 0, 
              input_at, next_input_at);
}

