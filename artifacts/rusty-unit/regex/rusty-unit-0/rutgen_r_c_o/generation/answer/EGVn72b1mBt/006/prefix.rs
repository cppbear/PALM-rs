// Answer 0

#[test]
fn test_step_char_match() {
    let mut slots = vec![None; 2];
    let mut matches = vec![false, false];
    let mut thread_caps = vec![None; 2];
    let prog = Program {
        insts: vec![
            Inst::Char(InstChar { goto: 1, c: 'a' }),
            Inst::Match(0),
        ],
        matches: vec![InstPtr(1)],
        captures: vec![Some(String::from("group1"))],
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
        dfa_size_limit: 100,
    };
    let input = InputAt { pos: 1, c: Char(97), byte: Some(97), len: 1 }; // at: 'a'
    let at_next = InputAt { pos: 2, c: Char(98), byte: Some(98), len: 1 }; // at_next: 'b'
    let mut fsm = Fsm { prog: &prog, stack: &mut vec![], input: () };
    
    fsm.step(&mut Threads { set: SparseSet::new(), caps: vec![], slots_per_thread: 2 }, 
             &mut matches, 
             &mut slots, 
             &mut thread_caps, 
             0, 
             input, 
             at_next);
}

#[test]
fn test_step_char_no_match() {
    let mut slots = vec![None; 2];
    let mut matches = vec![false, false];
    let mut thread_caps = vec![None; 2];
    let prog = Program {
        insts: vec![
            Inst::Char(InstChar { goto: 1, c: 'a' }),
            Inst::Match(0),
        ],
        matches: vec![InstPtr(1)],
        captures: vec![Some(String::from("group1"))],
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
        dfa_size_limit: 100,
    };
    let input = InputAt { pos: 0, c: Char(98), byte: Some(98), len: 1 }; // at: 'b'
    let at_next = InputAt { pos: 1, c: Char(97), byte: Some(97), len: 1 }; // at_next: 'a'
    let mut fsm = Fsm { prog: &prog, stack: &mut vec![], input: () };

    fsm.step(&mut Threads { set: SparseSet::new(), caps: vec![], slots_per_thread: 2 }, 
             &mut matches, 
             &mut slots, 
             &mut thread_caps, 
             0, 
             input, 
             at_next);
}

#[test]
fn test_step_char_edge_case() {
    let mut slots = vec![None; 2];
    let mut matches = vec![false, false];
    let mut thread_caps = vec![None; 2];
    let prog = Program {
        insts: vec![
            Inst::Char(InstChar { goto: 1, c: 'a' }),
            Inst::Match(0),
        ],
        matches: vec![InstPtr(1)],
        captures: vec![Some(String::from("group1"))],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr(0),
        byte_classes: vec![],
        only_utf8: true,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 100,
    };
    let input = InputAt { pos: 0, c: Char(256), byte: None, len: 1 }; // Non-ASCII character
    let at_next = InputAt { pos: 1, c: Char(97), byte: Some(97), len: 1 }; // at_next: 'a'
    let mut fsm = Fsm { prog: &prog, stack: &mut vec![], input: () };

    fsm.step(&mut Threads { set: SparseSet::new(), caps: vec![], slots_per_thread: 2 }, 
             &mut matches, 
             &mut slots, 
             &mut thread_caps, 
             0, 
             input, 
             at_next);
}

#[test]
fn test_step_empty_match_case() {
    let mut slots = vec![None; 2];
    let mut matches = vec![false, false];
    let mut thread_caps = vec![None; 2];
    let prog = Program {
        insts: vec![
            Inst::Char(InstChar { goto: 1, c: 'a' }),
            Inst::Match(0),
        ],
        matches: vec![InstPtr(1)],
        captures: vec![Some(String::from("group1"))],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr(0),
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: true,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 100,
    };
    let input = InputAt { pos: 0, c: Char(97), byte: Some(97), len: 1 }; // at: 'a'
    let at_next = InputAt { pos: 1, c: Char(97), byte: Some(97), len: 1 }; // at_next: 'a'
    let mut fsm = Fsm { prog: &prog, stack: &mut vec![], input: () };

    fsm.step(&mut Threads { set: SparseSet::new(), caps: vec![], slots_per_thread: 2 }, 
             &mut matches, 
             &mut slots, 
             &mut thread_caps, 
             0, 
             input, 
             at_next);
}

