// Answer 0

#[test]
fn test_add_step_valid_scenario() {
    let program = Program {
        insts: vec![
            Inst::Save(InstSave { goto: 1, slot: 0 }),
            Inst::Match(0),
        ],
        matches: vec![InstPtr(0)],
        captures: vec![Some("capture0".to_string())],
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
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1000,
    };
    
    let mut nlist = Threads::new();
    let mut thread_caps = vec![None; 1];
    let ip = 0;
    let input_at = InputAt {
        pos: 0,
        c: Char::from('\0'),
        byte: Some(0),
        len: 1,
    };

    let mut fsm = Fsm { prog: &program, stack: &mut vec![], input: MockInput {} };
    
    fsm.add_step(&mut nlist, &mut thread_caps, ip, input_at);
}

#[test]
fn test_add_step_empty_look_and_stack_push() {
    let program = Program {
        insts: vec![
            Inst::Save(InstSave { goto: 1, slot: 0 }),
            Inst::EmptyLook(InstEmptyLook { goto: 1, look: EmptyLook::Claim }),
            Inst::Match(0),
        ],
        matches: vec![InstPtr(0)],
        captures: vec![Some("capture0".to_string())],
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
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1000,
    };

    let mut nlist = Threads::new();
    let mut thread_caps = vec![None; 2];
    let ip = 0;
    let input_at = InputAt {
        pos: 0,
        c: Char::from('\0'),
        byte: Some(0),
        len: 1,
    };

    let mut fsm = Fsm { prog: &program, stack: &mut vec![], input: MockInput {} };

    fsm.add_step(&mut nlist, &mut thread_caps, ip, input_at);
}

#[test]
fn test_add_step_valid_with_multiple_saves() {
    let program = Program {
        insts: vec![
            Inst::Save(InstSave { goto: 1, slot: 0 }),
            Inst::Save(InstSave { goto: 2, slot: 1 }),
            Inst::Match(0),
        ],
        matches: vec![InstPtr(0)],
        captures: vec![Some("capture0".to_string()), Some("capture1".to_string())],
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
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1000,
    };

    let mut nlist = Threads::new();
    let mut thread_caps = vec![None, None]; 
    let ip = 0;
    let input_at = InputAt {
        pos: 0,
        c: Char::from('\0'),
        byte: Some(0),
        len: 1,
    };

    let mut fsm = Fsm { prog: &program, stack: &mut vec![], input: MockInput {} };

    fsm.add_step(&mut nlist, &mut thread_caps, ip, input_at);
}

