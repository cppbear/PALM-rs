// Answer 0

fn test_exec_empty_input_at_start() {
    struct TestInput;

    impl Input for TestInput {
        fn at(&self, _: usize) -> InputAt {
            InputAt { pos: 0, c: Char, byte: None, len: 1 }
        }

        fn next_char(&self, _: InputAt) -> Char {
            Char
        }

        fn previous_char(&self, _: InputAt) -> Char {
            Char
        }

        fn is_empty_match(&self, _: InputAt, _: &InstEmptyLook) -> bool {
            false
        }

        fn prefix_at(&self, _: &LiteralSearcher, _: InputAt) -> Option<InputAt> {
            None
        }

        fn len(&self) -> usize {
            0
        }

        fn is_empty(&self) -> bool {
            true
        }

        fn as_bytes(&self) -> &[u8] {
            &[]
        }
    }

    let program = Program {
        insts: vec![], // fill with instruction data
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

    let mut cache = ProgramCache;
    let mut matches = vec![false; 10];
    let mut slots = vec![Slot; 10];
    let quit_after_match = true;
    let input = TestInput;

    let mut fsm = Fsm {
        prog: &program,
        stack: &mut vec![],
        input,
    };

    let clist = Threads::new();
    let nlist = Threads::new();
    let at = InputAt { pos: 0, c: Char, byte: None, len: 1 };

    let result = fsm.exec_(&mut clist, &mut nlist, &mut matches, &mut slots, quit_after_match, at);
    assert!(result);
}

fn test_exec_with_matched() {
    struct TestInput;

    impl Input for TestInput {
        fn at(&self, _: usize) -> InputAt {
            InputAt { pos: 0, c: Char, byte: Some(1), len: 1 }
        }

        fn next_char(&self, _: InputAt) -> Char {
            Char
        }

        fn previous_char(&self, _: InputAt) -> Char {
            Char
        }

        fn is_empty_match(&self, _: InputAt, _: &InstEmptyLook) -> bool {
            false
        }

        fn prefix_at(&self, _: &LiteralSearcher, _: InputAt) -> Option<InputAt> {
            Some(InputAt { pos: 1, c: Char, byte: None, len: 1 })
        }

        fn len(&self) -> usize {
            1
        }

        fn is_empty(&self) -> bool {
            false
        }

        fn as_bytes(&self) -> &[u8] {
            &[1]
        }
    }

    let program = Program {
        insts: vec![], // fill with instruction data that ensures matching
        matches: vec![true],
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

    let mut cache = ProgramCache;
    let mut matches = vec![false; 10];
    let mut slots = vec![Slot; 10];
    let quit_after_match = true;
    let input = TestInput;

    let mut fsm = Fsm {
        prog: &program,
        stack: &mut vec![],
        input,
    };

    let mut clist = Threads::new();
    let mut nlist = Threads::new();
    clist.set.insert(0); // ensure clist is not empty
    let at = InputAt { pos: 0, c: Char, byte: Some(1), len: 1 };

    let result = fsm.exec_(&mut clist, &mut nlist, &mut matches, &mut slots, quit_after_match, at);
    assert!(result);
}

