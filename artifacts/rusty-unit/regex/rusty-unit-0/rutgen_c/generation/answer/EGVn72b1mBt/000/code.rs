// Answer 0

#[test]
fn test_step_match() {
    let prog = Program { 
        insts: vec![Inst::Match(0)], 
        matches: vec![InstPtr(0)], 
        captures: vec![], 
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
        prefixes: LiteralSearcher::default(), 
        dfa_size_limit: 0 
    };
    
    let mut slots = vec![Slot::default()];
    let mut thread_caps = vec![None];
    let mut nlist = Threads { 
        set: SparseSet::new(), 
        caps: vec![], 
        slots_per_thread: 0 
    };
    
    let at = InputAt { pos: 0, c: Char(0), byte: None, len: 1 };
    let at_next = InputAt { pos: 1, c: Char(0), byte: None, len: 1 };
    
    let mut matches = vec![false];
    
    let mut fsm = Fsm { prog: &prog, stack: &mut vec![], input: () };
    
    assert!(fsm.step(&mut nlist, &mut matches, &mut slots, &mut thread_caps, 0, at, at_next));
    assert!(matches[0]);
}

#[test]
fn test_step_char_match() {
    let prog = Program { 
        insts: vec![Inst::Char(InstChar { goto: InstPtr(1), c: 'a' })], 
        matches: vec![InstPtr(0)], 
        captures: vec![], 
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
        prefixes: LiteralSearcher::default(), 
        dfa_size_limit: 0 
    };
    
    let mut slots = vec![Slot::default()];
    let mut thread_caps = vec![None];
    let mut nlist = Threads { 
        set: SparseSet::new(), 
        caps: vec![], 
        slots_per_thread: 0 
    };
    
    let at = InputAt { pos: 0, c: Char(0), byte: None, len: 1 };
    let at_next = InputAt { pos: 1, c: Char(0), byte: None, len: 1 };
    
    let mut matches = vec![false];
    
    let mut fsm = Fsm { prog: &prog, stack: &mut vec![], input: () };
    
    assert!(!fsm.step(&mut nlist, &mut matches, &mut slots, &mut thread_caps, 0, at, at_next));
    
    let at_char = InputAt { pos: 0, c: Char('a' as u32), byte: None, len: 1 };
    
    assert!(fsm.step(&mut nlist, &mut matches, &mut slots, &mut thread_caps, 0, at_char, at_next));
}

#[test]
fn test_step_ranges_match() {
    let prog = Program { 
        insts: vec![Inst::Ranges(InstRanges { goto: InstPtr(1), ranges: vec![('a', 'z')] })], 
        matches: vec![InstPtr(0)], 
        captures: vec![], 
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
        prefixes: LiteralSearcher::default(), 
        dfa_size_limit: 0 
    };
    
    let mut slots = vec![Slot::default()];
    let mut thread_caps = vec![None];
    let mut nlist = Threads { 
        set: SparseSet::new(), 
        caps: vec![], 
        slots_per_thread: 0 
    };
    
    let at = InputAt { pos: 0, c: Char('a' as u32), byte: None, len: 1 };
    let at_next = InputAt { pos: 1, c: Char('b' as u32), byte: None, len: 1 };
    
    let mut matches = vec![false];
    
    let mut fsm = Fsm { prog: &prog, stack: &mut vec![], input: () };
    
    assert!(fsm.step(&mut nlist, &mut matches, &mut slots, &mut thread_caps, 0, at, at_next));
}

#[test]
fn test_step_bytes_match() {
    let prog = Program { 
        insts: vec![Inst::Bytes(InstBytes { goto: InstPtr(1), start: b'a', end: b'z' })], 
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
        dfa_size_limit: 0 
    };
    
    let mut slots = vec![Slot::default()];
    let mut thread_caps = vec![None];
    let mut nlist = Threads { 
        set: SparseSet::new(), 
        caps: vec![], 
        slots_per_thread: 0 
    };
    
    let at = InputAt { pos: 0, c: Char(0), byte: Some(b'a'), len: 1 };
    let at_next = InputAt { pos: 1, c: Char(0), byte: None, len: 1 };
    
    let mut matches = vec![false];
    
    let mut fsm = Fsm { prog: &prog, stack: &mut vec![], input: () };
    
    assert!(fsm.step(&mut nlist, &mut matches, &mut slots, &mut thread_caps, 0, at, at_next));
}

#[test]
fn test_step_no_match() {
    let prog = Program { 
        insts: vec![Inst::Char(InstChar { goto: InstPtr(1), c: 'a' })], 
        matches: vec![InstPtr(0)], 
        captures: vec![], 
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
        prefixes: LiteralSearcher::default(), 
        dfa_size_limit: 0 
    };
    
    let mut slots = vec![Slot::default()];
    let mut thread_caps = vec![None];
    let mut nlist = Threads { 
        set: SparseSet::new(), 
        caps: vec![], 
        slots_per_thread: 0 
    };
    
    let at = InputAt { pos: 0, c: Char('b' as u32), byte: None, len: 1 };
    let at_next = InputAt { pos: 1, c: Char(0), byte: None, len: 1 };
    
    let mut matches = vec![false];
    
    let mut fsm = Fsm { prog: &prog, stack: &mut vec![], input: () };
    
    assert!(!fsm.step(&mut nlist, &mut matches, &mut slots, &mut thread_caps, 0, at, at_next));
}

