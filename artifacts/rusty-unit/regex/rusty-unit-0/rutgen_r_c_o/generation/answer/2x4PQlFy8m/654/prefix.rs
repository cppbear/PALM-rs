// Answer 0

#[test]
fn test_exec_empty_clist_set() {
    let prog = Program {
        insts: vec![],
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
        prefixes: LiteralSearcher::empty(),
        dfa_size_limit: 0,
    };
    
    let mut clist = Threads::new();
    let mut nlist = Threads::new();
    let mut matches = vec![false];
    let mut slots = vec![];
    
    let at = InputAt { pos: 0, c: Char::new(), byte: None, len: 1 };

    let fsm = Fsm { prog: &prog, stack: &mut vec![], input: TestInput {} };
    fsm.exec_(&mut clist, &mut nlist, &mut matches, &mut slots, false, at);
}

#[test]
fn test_exec_non_empty_clist_set() {
    let prog = Program {
        insts: vec![Inst::Match(0)],
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
        prefixes: LiteralSearcher::empty(),
        dfa_size_limit: 0,
    };
    
    let mut clist = Threads::new();
    clist.set.insert(0);
    let mut nlist = Threads::new();
    let mut matches = vec![false];
    let mut slots = vec![0; 1];
    
    let at = InputAt { pos: 0, c: Char::new(), byte: None, len: 1 };

    let fsm = Fsm { prog: &prog, stack: &mut vec![], input: TestInput {} };
    fsm.exec_(&mut clist, &mut nlist, &mut matches, &mut slots, false, at);
}

#[test]
fn test_exec_at_end() {
    let prog = Program {
        insts: vec![Inst::Match(0)],
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
        prefixes: LiteralSearcher::empty(),
        dfa_size_limit: 0,
    };
    
    let mut clist = Threads::new();
    clist.set.insert(0);
    let mut nlist = Threads::new();
    let mut matches = vec![false];
    let mut slots = vec![0; 1];
    
    let at = InputAt { pos: 1, c: Char::new(), byte: None, len: 1 };

    let fsm = Fsm { prog: &prog, stack: &mut vec![], input: TestInput {} };
    fsm.exec_(&mut clist, &mut nlist, &mut matches, &mut slots, false, at);
}

#[test]
#[should_panic]
fn test_exec_panic_on_out_of_bounds_index() {
    let prog = Program {
        insts: vec![Inst::Match(0)],
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
        prefixes: LiteralSearcher::empty(),
        dfa_size_limit: 0,
    };
    
    let mut clist = Threads::new();
    clist.set.insert(0); // clist has 1 element
    let mut nlist = Threads::new();
    let mut matches = vec![false];
    let mut slots = vec![0; 1];
    
    let at = InputAt { pos: 0, c: Char::new(), byte: None, len: 1 };

    let fsm = Fsm { prog: &prog, stack: &mut vec![], input: TestInput {} };
    // This should panic as we will be trying to access an index that doesn't exist.
    fsm.exec_(&mut clist, &mut nlist, &mut matches, &mut slots, false, at);
}

