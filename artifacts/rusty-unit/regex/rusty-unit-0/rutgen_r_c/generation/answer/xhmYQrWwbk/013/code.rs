// Answer 0

#[test]
fn test_add_step_match_case() {
    struct MockInput;
    
    impl Input for MockInput {
        fn at(&self, _i: usize) -> InputAt { InputAt { pos: 0, c: 'a', byte: None, len: 1 } }
        fn next_char(&self, _at: InputAt) -> Char { 'a' }
        fn previous_char(&self, _at: InputAt) -> Char { 'a' }
        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool { false }
        fn prefix_at(&self, _prefixes: &LiteralSearcher, _at: InputAt) -> Option<InputAt> { None }
        fn len(&self) -> usize { 1 }
        fn is_empty(&self) -> bool { false }
        fn as_bytes(&self) -> &[u8] { &[] }
    }

    let program = Program {
        insts: vec![Inst::Match(0)],
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
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };
    
    let mut stack = vec![];
    let mut nlist = Threads::new();
    let mut thread_caps = vec![None; 1];
    
    let ip = 0; // Inst::Match(0)
    let at = InputAt { pos: 0, c: 'a', byte: None, len: 1 };

    let mut fsm = Fsm { prog: &program, stack: &mut stack, input: MockInput };

    fsm.add_step(&mut nlist, &mut thread_caps, ip, at);
    
    assert!(nlist.set.contains(0));
}

#[test]
fn test_add_step_no_slot_update() {
    struct MockInput;

    impl Input for MockInput {
        fn at(&self, _i: usize) -> InputAt { InputAt { pos: 0, c: 'a', byte: None, len: 1 } }
        fn next_char(&self, _at: InputAt) -> Char { 'a' }
        fn previous_char(&self, _at: InputAt) -> Char { 'a' }
        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool { false }
        fn prefix_at(&self, _prefixes: &LiteralSearcher, _at: InputAt) -> Option<InputAt> { None }
        fn len(&self) -> usize { 1 }
        fn is_empty(&self) -> bool { false }
        fn as_bytes(&self) -> &[u8] { &[] }
    }
    
    let program = Program {
        insts: vec![Inst::Split(InstSplit { goto1: 1, goto2: 2 }), Inst::Match(0)],
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
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };
    
    let mut stack = vec![];
    let mut nlist = Threads::new();
    let mut thread_caps = vec![Some(0); 1];
    
    let ip = 0; // Inst::Split
    let at = InputAt { pos: 0, c: 'a', byte: None, len: 1 };

    let mut fsm = Fsm { prog: &program, stack: &mut stack, input: MockInput };

    fsm.add_step(&mut nlist, &mut thread_caps, ip, at);
    
    assert!(nlist.set.contains(0));
    assert_eq!(thread_caps[0], Some(0));
}

