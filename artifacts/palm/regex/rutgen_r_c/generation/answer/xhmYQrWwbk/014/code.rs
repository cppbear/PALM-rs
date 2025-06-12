// Answer 0

#[test]
fn test_add_step_with_match_instruction() {
    use prog::{Inst, InstSave, InstSplit, InstEmptyLook, InstEmptyLook};
    use std::ptr;

    struct TestInput {}

    impl Input for TestInput {
        fn at(&self, _: usize) -> InputAt {
            InputAt { pos: 0, c: Char::from('a'), byte: None, len: 1 }
        }
        fn next_char(&self, _: InputAt) -> Char { Char::from('a') }
        fn previous_char(&self, _: InputAt) -> Char { Char::from('a') }
        fn is_empty_match(&self, _: InputAt, _: &InstEmptyLook) -> bool { true }
        fn prefix_at(&self, _: &LiteralSearcher, _: InputAt) -> Option<InputAt> { None }
        fn len(&self) -> usize { 1 }
        fn is_empty(&self) -> bool { false }
        fn as_bytes(&self) -> &[u8] { b"a" }
    }

    let program = Program {
        insts: vec![
            Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::Match }),
            Inst::Match(0),
        ],
        matches: vec![InstPtr::new(0)],
        captures: vec![None],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr::new(0),
        byte_classes: vec![],
        only_utf8: true,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1024,
    };

    let mut stack = vec![];
    let mut nlist = Threads::new();
    let mut thread_caps = vec![None; 1];
    let mut fsm = Fsm {
        prog: &program,
        stack: &mut stack,
        input: TestInput {},
    };

    let ip = 1;

    fsm.add_step(&mut nlist, &mut thread_caps, ip, InputAt::new(0, Char::from('a'), None, 1));
    assert!(nlist.set.contains(ip));
}

#[test]
#[should_panic]
fn test_add_step_panic_on_already_visited() {
    use prog::{Inst, InstSave, InstSplit, InstEmptyLook, InstEmptyLook};
    
    struct TestInput {}

    impl Input for TestInput {
        fn at(&self, _:usize) -> InputAt { InputAt { pos: 0, c: Char::from('a'), byte: None, len: 1 } }
        fn next_char(&self, _: InputAt) -> Char { Char::from('a') }
        fn previous_char(&self, _: InputAt) -> Char { Char::from('a') }
        fn is_empty_match(&self, _: InputAt, _: &InstEmptyLook) -> bool { true }
        fn prefix_at(&self, _: &LiteralSearcher, _: InputAt) -> Option<InputAt> { None }
        fn len(&self) -> usize { 1 }
        fn is_empty(&self) -> bool { false }
        fn as_bytes(&self) -> &[u8] { b"a" }
    }

    let program = Program {
        insts: vec![
            Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::Match }),
            Inst::Match(0),
        ],
        matches: vec![InstPtr::new(0)],
        captures: vec![None],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr::new(0),
        byte_classes: vec![],
        only_utf8: true,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1024,
    };

    let mut stack = vec![];
    let mut nlist = Threads::new();
    let mut thread_caps = vec![None; 1];
    let mut fsm = Fsm {
        prog: &program,
        stack: &mut stack,
        input: TestInput {},
    };

    let ip = 0;

    nlist.set.insert(ip);
    
    fsm.add_step(&mut nlist, &mut thread_caps, ip, InputAt::new(0, Char::from('a'), None, 1));
}

#[test]
fn test_add_step_with_empty_match() {
    use prog::{Inst, InstSave, InstSplit, InstEmptyLook, InstEmptyLook};

    struct TestInput {}

    impl Input for TestInput {
        fn at(&self, _: usize) -> InputAt {
            InputAt { pos: 0, c: Char::from('a'), byte: None, len: 1 }
        }
        fn next_char(&self, _: InputAt) -> Char { Char::from('a') }
        fn previous_char(&self, _: InputAt) -> Char { Char::from('a') }
        fn is_empty_match(&self, _: InputAt, _: &InstEmptyLook) -> bool { true }
        fn prefix_at(&self, _: &LiteralSearcher, _: InputAt) -> Option<InputAt> { None }
        fn len(&self) -> usize { 1 }
        fn is_empty(&self) -> bool { false }
        fn as_bytes(&self) -> &[u8] { b"a" }
    }

    let program = Program {
        insts: vec![
            Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::Match }),
            Inst::Match(0),
        ],
        matches: vec![InstPtr::new(0)],
        captures: vec![None],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr::new(0),
        byte_classes: vec![],
        only_utf8: true,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1024,
    };

    let mut stack = vec![];
    let mut nlist = Threads::new();
    let mut thread_caps = vec![None; 1];
    let mut fsm = Fsm {
        prog: &program,
        stack: &mut stack,
        input: TestInput {},
    };

    let ip = 0;

    fsm.add_step(&mut nlist, &mut thread_caps, ip, InputAt::new(0, Char::from('a'), None, 1));
    assert!(nlist.set.contains(ip));
}

