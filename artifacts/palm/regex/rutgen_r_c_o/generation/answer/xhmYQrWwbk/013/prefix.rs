// Answer 0

#[test]
fn test_add_step_with_match() {
    let prog = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![InstPtr(0)],
        captures: vec![None],
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
        dfa_size_limit: 0,
    };
    
    let mut stack = vec![];
    let input = vec![];
    let fsm = Fsm { prog: &prog, stack: &mut stack, input };
    
    let mut nlist = Threads::new();
    nlist.resize(1, 1);
    
    let thread_caps = &mut vec![None];
    let at = InputAt { pos: 0, c: Char::from('a'), byte: Some(97), len: 1 };
    let ip = 0; // valid index for the prog

    fsm.add_step(&mut nlist, thread_caps, ip, at);
}

#[test]
fn test_add_step_with_empty_match() {
    let prog = Program {
        insts: vec![
            Inst::EmptyLook(InstEmptyLook { goto: InstPtr(1), look: EmptyLook::new() }),
            Inst::Match(0),
        ],
        matches: vec![InstPtr(0)],
        captures: vec![None],
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
        dfa_size_limit: 0,
    };
    
    let mut stack = vec![];
    let input = vec![];
    let fsm = Fsm { prog: &prog, stack: &mut stack, input };
    
    let mut nlist = Threads::new();
    nlist.resize(1, 1);
    
    let thread_caps = &mut vec![None];
    let at = InputAt { pos: 0, c: Char::from('a'), byte: Some(97), len: 1 };
    let ip = 0; // valid index for the prog

    fsm.add_step(&mut nlist, thread_caps, ip, at);
}

#[test]
#[should_panic]
fn test_add_step_with_visited_state() {
    let prog = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![InstPtr(0)],
        captures: vec![None],
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
        dfa_size_limit: 0,
    };
    
    let mut stack = vec![];
    let input = vec![];
    let mut fsm = Fsm { prog: &prog, stack: &mut stack, input };
    
    let mut nlist = Threads::new();
    nlist.resize(1, 1);
    nlist.set.insert(0); // Setting the state as visited
    
    let thread_caps = &mut vec![None];
    let at = InputAt { pos: 0, c: Char::from('a'), byte: Some(97), len: 1 };
    let ip = 0; // valid index for the prog

    fsm.add_step(&mut nlist, thread_caps, ip, at);
}

#[test]
fn test_add_step_with_no_capture() {
    let prog = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![InstPtr(0)],
        captures: vec![None, None],
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
        dfa_size_limit: 0,
    };
    
    let mut stack = vec![];
    let input = vec![];
    let fsm = Fsm { prog: &prog, stack: &mut stack, input };
    
    let mut nlist = Threads::new();
    nlist.resize(1, 2); // two capture slots
    
    let thread_caps = &mut vec![None, None];
    let at = InputAt { pos: 0, c: Char::from('a'), byte: Some(97), len: 1 };
    let ip = 0; // valid index for the prog

    fsm.add_step(&mut nlist, thread_caps, ip, at);
}

