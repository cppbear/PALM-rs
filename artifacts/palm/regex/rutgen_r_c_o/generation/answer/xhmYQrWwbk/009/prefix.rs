// Answer 0

#[test]
fn test_add_step_case_1() {
    let mut prog = Program {
        insts: vec![Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::new() })],
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
    let input_at = InputAt { pos: 0, c: Char::new(), byte: None, len: 0 };
    let mut nlist = Threads::new();
    nlist.set.insert(0);
    let mut thread_caps = vec![None, None];
    
    let mut fsm = Fsm {
        prog: &prog,
        stack: &mut stack,
        input: input,
    };
    
    fsm.add_step(&mut nlist, &mut thread_caps, 0, input_at);
}

#[test]
fn test_add_step_case_2() {
    let mut prog = Program {
        insts: vec![
            Inst::EmptyLook(InstEmptyLook { goto: 2, look: EmptyLook::new() }),
            Inst::Save(InstSave { goto: 0, slot: 0 }),
            Inst::Match(0),
        ],
        matches: vec![],
        captures: vec![Some("capture1".to_string())],
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
    let input_at = InputAt { pos: 0, c: Char::new(), byte: None, len: 0 };
    let mut nlist = Threads::new();
    nlist.set.insert(1);
    let mut thread_caps = vec![None, None];
    
    let mut fsm = Fsm {
        prog: &prog,
        stack: &mut stack,
        input: input,
    };
    
    fsm.add_step(&mut nlist, &mut thread_caps, 1, input_at);
}

#[test]
fn test_add_step_case_3() {
    let mut prog = Program {
        insts: vec![
            Inst::Split(InstSplit { goto1: 1, goto2: 2 }),
            Inst::Bytes(InstBytes::new()),
            Inst::Match(3),
        ],
        matches: vec![],
        captures: vec![Some("capture2".to_string())],
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
    let input_at = InputAt { pos: 1, c: Char::new(), byte: None, len: 1 };
    let mut nlist = Threads::new();
    nlist.set.insert(0);
    let mut thread_caps = vec![None, None];
    
    let mut fsm = Fsm {
        prog: &prog,
        stack: &mut stack,
        input: input,
    };
    
    fsm.add_step(&mut nlist, &mut thread_caps, 0, input_at);
}

#[test]
#[should_panic]
fn test_add_step_case_panic() {
    let mut prog = Program {
        insts: vec![
            Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::new() }),
            Inst::Match(0),
        ],
        matches: vec![],
        captures: vec![Some("capture3".to_string())],
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
    let input_at = InputAt { pos: 1, c: Char::new(), byte: None, len: 0 };
    let mut nlist = Threads::new();
    nlist.set.insert(1);
    let mut thread_caps = vec![None];

    let mut fsm = Fsm {
        prog: &prog,
        stack: &mut stack,
        input: input,
    };

    fsm.add_step(&mut nlist, &mut thread_caps, 2, input_at);
}

