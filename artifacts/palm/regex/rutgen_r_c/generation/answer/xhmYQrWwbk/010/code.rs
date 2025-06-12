// Answer 0

#[test]
fn test_add_step_with_split_instruction() {
    struct MockInput;
    impl Input for MockInput {
        fn at(&self, _i: usize) -> InputAt { InputAt { pos: 0, c: Char::from(0), byte: None, len: 0 } }
        fn next_char(&self, _at: InputAt) -> Char { Char::from(0) }
        fn previous_char(&self, _at: InputAt) -> Char { Char::from(0) }
        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool { false }
        fn prefix_at(&self, _prefixes: &LiteralSearcher, _at: InputAt) -> Option<InputAt> { None }
        fn len(&self) -> usize { 1 }
        fn is_empty(&self) -> bool { false }
        fn as_bytes(&self) -> &[u8] { &[b'a'] }
    }

    let mut program = Program {
        insts: vec![
            Inst::Split(InstSplit { goto1: 2, goto2: 1 }),
            Inst::Match(0),
            Inst::Save(InstSave { goto: 1, slot: 0 }),
        ],
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

    let mut stack = Vec::new();
    let mut nlist = Threads::new();
    nlist.resize(3, 1); // Resize to accommodate instructions and captures
    let mut thread_caps = vec![None; 1];

    let mut fsm = Fsm {
        prog: &program,
        stack: &mut stack,
        input: MockInput,
    };

    fsm.add_step(&mut nlist, &mut thread_caps, 0, InputAt { pos: 0, c: Char::from(0), byte: None, len: 0 });

    assert!(nlist.set.contains(0)); // Ensure that instruction pointer 0 is tracked
    assert!(nlist.set.contains(1)); // Ensure that instruction pointer 1 is reached via Split
}

#[test]
#[should_panic]
fn test_add_step_panic_on_visited_state() {
    struct MockInput;
    impl Input for MockInput {
        fn at(&self, _i: usize) -> InputAt { InputAt { pos: 0, c: Char::from(0), byte: None, len: 0 } }
        fn next_char(&self, _at: InputAt) -> Char { Char::from(0) }
        fn previous_char(&self, _at: InputAt) -> Char { Char::from(0) }
        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool { false }
        fn prefix_at(&self, _prefixes: &LiteralSearcher, _at: InputAt) -> Option<InputAt> { None }
        fn len(&self) -> usize { 1 }
        fn is_empty(&self) -> bool { false }
        fn as_bytes(&self) -> &[u8] { &[b'a'] }
    }

    let mut program = Program {
        insts: vec![
            Inst::Split(InstSplit { goto1: 2, goto2: 1 }),
            Inst::Match(0),
            Inst::Save(InstSave { goto: 1, slot: 0 }),
        ],
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

    let mut stack = Vec::new();
    let mut nlist = Threads::new();
    nlist.resize(3, 1); 
    nlist.set.insert(0); // Manually mark state 0 as visited.
    let mut thread_caps = vec![None; 1];

    let mut fsm = Fsm {
        prog: &program,
        stack: &mut stack,
        input: MockInput,
    };

    fsm.add_step(&mut nlist, &mut thread_caps, 0, InputAt { pos: 0, c: Char::from(0), byte: None, len: 0 });
}

