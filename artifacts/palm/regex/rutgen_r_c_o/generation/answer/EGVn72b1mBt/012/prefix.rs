// Answer 0

#[test]
fn test_step_with_valid_match_on_first_instruction() {
    let mut matches = vec![false; 1];
    let mut slots = vec![Slot::default(); 1];
    let mut thread_caps = vec![Some(0)];

    let inst = prog::Inst::Match(0);
    let prog = Program {
        insts: vec![inst],
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
        dfa_size_limit: 10,
    };

    let input_at = InputAt { pos: 0, c: Char(97), byte: Some(97), len: 1 }; // 'a'
    let input_at_next = InputAt { pos: 1, c: Char(98), byte: Some(98), len: 1 }; // 'b'

    let mut nlist = Threads { set: SparseSet::default(), caps: vec![], slots_per_thread: 1 };

    let mut fsm = Fsm { prog: &prog, stack: &mut vec![], input: () };

    let result = fsm.step(&mut nlist, &mut matches, &mut slots, &mut thread_caps, 0, input_at, input_at_next);
}

#[test]
fn test_step_with_multiple_matches() {
    let mut matches = vec![false; 3];
    let mut slots = vec![Slot::default(); 3];
    let mut thread_caps = vec![Some(1), Some(2)];

    let inst1 = prog::Inst::Match(0);
    let inst2 = prog::Inst::Match(1);
    let inst3 = prog::Inst::Match(2);
    
    let prog = Program {
        insts: vec![inst1, inst2, inst3],
        matches: vec![InstPtr::default(); 3],
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
        dfa_size_limit: 10,
    };

    let input_at = InputAt { pos: 0, c: Char(97), byte: Some(97), len: 1 }; // 'a'
    let input_at_next = InputAt { pos: 1, c: Char(98), byte: Some(98), len: 1 }; // 'b'

    let mut nlist = Threads { set: SparseSet::default(), caps: vec![], slots_per_thread: 2 };

    let mut fsm = Fsm { prog: &prog, stack: &mut vec![], input: () };

    let result_first = fsm.step(&mut nlist, &mut matches, &mut slots, &mut thread_caps, 0, input_at, input_at_next);
    let result_second = fsm.step(&mut nlist, &mut matches, &mut slots, &mut thread_caps, 1, input_at, input_at_next);
    let result_third = fsm.step(&mut nlist, &mut matches, &mut slots, &mut thread_caps, 2, input_at, input_at_next);
}

#[test]
fn test_step_with_non_matching_slot() {
    let mut matches = vec![false; 5];
    let mut slots = vec![Slot::default(); 5];
    let mut thread_caps = vec![None, None, None];

    let inst = prog::Inst::Match(0);
    
    let prog = Program {
        insts: vec![inst],
        matches: vec![InstPtr::default(); 5],
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
        dfa_size_limit: 10,
    };

    let input_at = InputAt { pos: 0, c: Char(97), byte: Some(97), len: 1 }; // 'a'
    let input_at_next = InputAt { pos: 1, c: Char(98), byte: Some(98), len: 1 }; // 'b'

    let mut nlist = Threads { set: SparseSet::default(), caps: vec![], slots_per_thread: 1 };

    let mut fsm = Fsm { prog: &prog, stack: &mut vec![], input: () };

    let result = fsm.step(&mut nlist, &mut matches, &mut slots, &mut thread_caps, 0, input_at, input_at_next);
}

