// Answer 0

#[test]
fn test_add_step_with_empty_nlist() {
    use prog::{Inst, InstBytes, InputAt};

    struct MockInput;
    impl Input for MockInput {
        fn at(&self, _i: usize) -> InputAt {
            InputAt { pos: 0, c: 'a', byte: None, len: 1 }
        }

        fn next_char(&self, _at: InputAt) -> char { 'b' }
        fn previous_char(&self, _at: InputAt) -> char { 'a' }
        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool { false }
        fn prefix_at(&self, _prefixes: &LiteralSearcher, _at: InputAt) -> Option<InputAt> { None }
        fn len(&self) -> usize { 1 }
        fn is_empty(&self) -> bool { false }
        fn as_bytes(&self) -> &[u8] { b"ab" }
    }

    let program = Program {
        insts: vec![Inst::Bytes(InstBytes)],
        matches: vec![],
        captures: vec![],
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
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let mut stack: Vec<FollowEpsilon> = vec![];
    let mut nlist = Threads::new();
    let mut thread_caps = vec![None; 0]; // No slots
    let mut fsm = Fsm {
        prog: &program,
        stack: &mut stack,
        input: MockInput,
    };

    let input_at = InputAt { pos: 0, c: 'a', byte: None, len: 1 };
    fsm.add_step(&mut nlist, &mut thread_caps, 0, input_at);

    assert_eq!(nlist.set.len(), 1); // nlist should contain the ip
}

#[test]
#[should_panic]
fn test_add_step_with_existing_ip() {
    use prog::{Inst, InstBytes, InputAt};

    struct MockInput;
    impl Input for MockInput {
        fn at(&self, _i: usize) -> InputAt {
            InputAt { pos: 0, c: 'a', byte: None, len: 1 }
        }

        fn next_char(&self, _at: InputAt) -> char { 'b' }
        fn previous_char(&self, _at: InputAt) -> char { 'a' }
        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool { false }
        fn prefix_at(&self, _prefixes: &LiteralSearcher, _at: InputAt) -> Option<InputAt> { None }
        fn len(&self) -> usize { 1 }
        fn is_empty(&self) -> bool { false }
        fn as_bytes(&self) -> &[u8] { b"ab" }
    }

    let program = Program {
        insts: vec![Inst::Bytes(InstBytes), Inst::Bytes(InstBytes)],
        matches: vec![],
        captures: vec![],
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
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let mut stack: Vec<FollowEpsilon> = vec![];
    let mut nlist = Threads::new();
    let mut thread_caps = vec![None; 1]; // One slot for capturing
    let mut fsm = Fsm {
        prog: &program,
        stack: &mut stack,
        input: MockInput,
    };

    let input_at = InputAt { pos: 0, c: 'a', byte: None, len: 1 };
    nlist.set.insert(0); // Simulate existing ip
    fsm.add_step(&mut nlist, &mut thread_caps, 0, input_at); 
}

#[test]
fn test_add_step_with_no_captures() {
    use prog::{Inst, InstBytes, InputAt};

    struct MockInput;
    impl Input for MockInput {
        fn at(&self, _i: usize) -> InputAt {
            InputAt { pos: 0, c: 'a', byte: None, len: 1 }
        }

        fn next_char(&self, _at: InputAt) -> char { 'b' }
        fn previous_char(&self, _at: InputAt) -> char { 'a' }
        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool { false }
        fn prefix_at(&self, _prefixes: &LiteralSearcher, _at: InputAt) -> Option<InputAt> { None }
        fn len(&self) -> usize { 1 }
        fn is_empty(&self) -> bool { false }
        fn as_bytes(&self) -> &[u8] { b"ab" }
    }

    let program = Program {
        insts: vec![Inst::Bytes(InstBytes)],
        matches: vec![],
        captures: vec![],
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
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let mut stack: Vec<FollowEpsilon> = vec![];
    let mut nlist = Threads::new();
    let mut thread_caps = vec![None; 1]; // One slot available
    let mut fsm = Fsm {
        prog: &program,
        stack: &mut stack,
        input: MockInput,
    };

    let input_at = InputAt { pos: 0, c: 'a', byte: None, len: 1 };
    fsm.add_step(&mut nlist, &mut thread_caps, 0, input_at);
    assert!(thread_caps.iter().all(|&slot| slot.is_none())); // No captures should be updated
}

