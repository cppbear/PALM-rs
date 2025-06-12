// Answer 0

#[test]
fn test_add_step_new_thread() {
    struct TestInput;
    
    impl Input for TestInput {
        fn at(&self, _i: usize) -> InputAt {
            InputAt { pos: 0, c: 'a', byte: None, len: 1 }
        }
        fn next_char(&self, _at: InputAt) -> char {
            'a'
        }
        fn previous_char(&self, _at: InputAt) -> char {
            'a'
        }
        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool {
            true
        }
        fn prefix_at(&self, _prefixes: &LiteralSearcher, _at: InputAt) -> Option<InputAt> {
            Some(InputAt { pos: 0, c: 'a', byte: None, len: 1 })
        }
        fn len(&self) -> usize {
            1
        }
        fn is_empty(&self) -> bool {
            false
        }
        fn as_bytes(&self) -> &[u8] {
            b"a"
        }
    }

    let prog = Program {
        insts: vec![
            Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::ZeroWidth }), // 0
            Inst::Match(0)  // 1
        ],
        matches: vec![0],
        captures: vec![None],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: true,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: true,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let mut stack = vec![];

    let input = TestInput;
    let mut nlist = Threads::new();
    let mut thread_caps = vec![None; 1];

    let mut fsm = Fsm { prog: &prog, stack: &mut stack, input };

    fsm.add_step(&mut nlist, &mut thread_caps, 0, InputAt { pos: 0, c: 'a', byte: None, len: 1 });

    assert!(nlist.set.contains(0));
}

#[test]
#[should_panic]
fn test_add_step_panic_on_out_of_bounds() {
    struct TestInput;
    
    impl Input for TestInput {
        fn at(&self, _i: usize) -> InputAt {
            InputAt { pos: 0, c: 'a', byte: None, len: 1 }
        }
        fn next_char(&self, _at: InputAt) -> char {
            'a'
        }
        fn previous_char(&self, _at: InputAt) -> char {
            'a'
        }
        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool {
            true
        }
        fn prefix_at(&self, _prefixes: &LiteralSearcher, _at: InputAt) -> Option<InputAt> {
            Some(InputAt { pos: 0, c: 'a', byte: None, len: 1 })
        }
        fn len(&self) -> usize {
            1
        }
        fn is_empty(&self) -> bool {
            false
        }
        fn as_bytes(&self) -> &[u8] {
            b"a"
        }
    }

    let prog = Program {
        insts: vec![
            Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::ZeroWidth }), // 0
            Inst::Match(0)  // 1
        ],
        matches: vec![0],
        captures: vec![None],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: true,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let mut stack = vec![];

    let input = TestInput;
    let mut nlist = Threads::new();
    nlist.resize(2, 1); // resize with two instructions but no valid capture slots
    let thread_caps = vec![None; 1];

    let mut fsm = Fsm { prog: &prog, stack: &mut stack, input };

    fsm.add_step(&mut nlist, &thread_caps, 10, InputAt { pos: 10, c: 'a', byte: None, len: 1 });
}

#[test]
fn test_add_step_with_a_split() {
    struct TestInput;

    impl Input for TestInput {
        fn at(&self, _i: usize) -> InputAt {
            InputAt { pos: 0, c: 'a', byte: None, len: 1 }
        }
        fn next_char(&self, _at: InputAt) -> char {
            'a'
        }
        fn previous_char(&self, _at: InputAt) -> char {
            'a'
        }
        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool {
            true
        }
        fn prefix_at(&self, _prefixes: &LiteralSearcher, _at: InputAt) -> Option<InputAt> {
            Some(InputAt { pos: 0, c: 'a', byte: None, len: 1 })
        }
        fn len(&self) -> usize {
            1
        }
        fn is_empty(&self) -> bool {
            false
        }
        fn as_bytes(&self) -> &[u8] {
            b"a"
        }
    }

    let prog = Program {
        insts: vec![
            Inst::Split(InstSplit { goto1: 1, goto2: 2 }), // 0
            Inst::Match(0),  // 1
            Inst::Match(1),  // 2
        ],
        matches: vec![0, 1],
        captures: vec![None, None],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: true,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: true,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let mut stack = vec![];

    let input = TestInput;
    let mut nlist = Threads::new();
    let mut thread_caps = vec![None; 1];

    let mut fsm = Fsm { prog: &prog, stack: &mut stack, input };

    fsm.add_step(&mut nlist, &mut thread_caps, 0, InputAt { pos: 0, c: 'a', byte: None, len: 1 });

    assert!(nlist.set.contains(0));
    assert!(nlist.set.contains(1)); // Ensure that we encountered both goto1 and goto2
}

