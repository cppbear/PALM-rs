// Answer 0

#[test]
fn test_add_step_match_instruction() {
    let program = Program {
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
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let mut threads = Threads::new();
    threads.resize(1, 0);

    let input_at = InputAt { pos: 0, c: Char::from(0), byte: None, len: 1 };

    let mut nlist = Threads::new();
    let mut thread_caps = vec![None];

    let fsm = Fsm {
        prog: &program,
        stack: &mut vec![],
        input: DummyInput {},
    };

    fsm.add_step(&mut nlist, &mut thread_caps, 0, input_at);
    assert!(nlist.set.contains(0));
}

#[test]
fn test_add_step_save_instruction() {
    let program = Program {
        insts: vec![Inst::Save(InstSave { goto: 1, slot: 0 }), Inst::Match(0)],
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
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let mut threads = Threads::new();
    threads.resize(2, 1);

    let input_at = InputAt { pos: 0, c: Char::from(0), byte: None, len: 1 };

    let mut nlist = Threads::new();
    let mut thread_caps = vec![None];

    let fsm = Fsm {
        prog: &program,
        stack: &mut vec![],
        input: DummyInput {},
    };

    fsm.add_step(&mut nlist, &mut thread_caps, 0, input_at);
    assert!(nlist.set.contains(0));
    assert_eq!(thread_caps[0], Some(0));
}

#[test]
fn test_add_step_split_instruction() {
    let program = Program {
        insts: vec![
            Inst::Split(InstSplit { goto1: 2, goto2: 3 }),
            Inst::Match(1),
            Inst::Match(0),
        ],
        matches: vec![0, 1],
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

    let mut threads = Threads::new();
    threads.resize(3, 0);

    let input_at = InputAt { pos: 0, c: Char::from(0), byte: None, len: 1 };

    let mut nlist = Threads::new();
    let mut thread_caps = vec![None];

    let fsm = Fsm {
        prog: &program,
        stack: &mut vec![],
        input: DummyInput {},
    };

    fsm.add_step(&mut nlist, &mut thread_caps, 0, input_at);
    assert!(nlist.set.contains(0));
}

#[test]
fn test_add_step_emptylook_instruction() {
    let program = Program {
        insts: vec![
            Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::Start }),
            Inst::Match(0),
        ],
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
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let mut threads = Threads::new();
    threads.resize(1, 0);

    let input_at = InputAt { pos: 0, c: Char::from(0), byte: None, len: 1 };

    let mut nlist = Threads::new();
    let mut thread_caps = vec![None];

    let fsm = Fsm {
        prog: &program,
        stack: &mut vec![],
        input: DummyInput {},
    };

    fsm.add_step(&mut nlist, &mut thread_caps, 0, input_at);
    assert!(nlist.set.contains(0));
}

// DummyInput struct to satisfy trait bound for Fsm's input
struct DummyInput {}

impl Input for DummyInput {
    fn at(&self, _: usize) -> InputAt {
        InputAt { pos: 0, c: Char::from(0), byte: None, len: 1 }
    }
    fn next_char(&self, _: InputAt) -> Char { Char::from(0) }
    fn previous_char(&self, _: InputAt) -> Char { Char::from(0) }
    fn is_empty_match(&self, _: InputAt, _: &InstEmptyLook) -> bool { true }
    fn prefix_at(&self, _: &LiteralSearcher, _: InputAt) -> Option<InputAt> { None }
    fn len(&self) -> usize { 0 }
    fn is_empty(&self) -> bool { true }
    fn as_bytes(&self) -> &[u8] { &[] }
}

