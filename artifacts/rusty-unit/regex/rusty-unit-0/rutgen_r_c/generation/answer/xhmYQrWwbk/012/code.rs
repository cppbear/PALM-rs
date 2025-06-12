// Answer 0

#[test]
fn test_add_step_with_empty_thread_caps() {
    use prog::{Inst::Save, InstSave, InstPtr};

    let prog = Program {
        insts: vec![
            Save(InstSave { goto: 1, slot: 0 }),  // This will be executed first
            Save(InstSave { goto: 2, slot: 1 }),  // This will be executed second but will panic
            Match(0),
        ],
        matches: vec![InstPtr(0)],
        captures: vec![],
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

    let nlist = &mut Threads::new();
    let mut thread_caps = vec![None; 1]; // Only one slot is defined, but we will attempt to access index 1
    let mut stack: Vec<FollowEpsilon> = vec![];

    let mut fsm = Fsm {
        prog: &prog,
        stack: &mut stack,
        input: MockInput::new(),
    };

    let at = InputAt { pos: 0, c: Char::new(0), byte: None, len: 1 };
    
    let ip = 0;  // Starting IP
    
    // This should panic since inst.slot will be greater than thread_caps.len()
    let result = std::panic::catch_unwind(|| {
        fsm.add_step(nlist, &mut thread_caps, ip, at);
    });

    assert!(result.is_err());
}

#[test]
fn test_add_step_with_valid_input() {
    use prog::{Inst::Save, InstSave, InstPtr};

    let prog = Program {
        insts: vec![
            Save(InstSave { goto: 1, slot: 0 }),  // This will save capture index 0
            Match(0),                             // This will be the matching instruction
        ],
        matches: vec![InstPtr(0)],
        captures: vec![],
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

    let nlist = &mut Threads::new();
    let mut thread_caps = vec![None; 1]; // Valid thread_caps of size 1
    let mut stack: Vec<FollowEpsilon> = vec![];

    let mut fsm = Fsm {
        prog: &prog,
        stack: &mut stack,
        input: MockInput::new(),
    };

    let at = InputAt { pos: 0, c: Char::new(5), byte: None, len: 1 }; 
    let ip = 0;  // Starting IP
    
    // Execute the add_step function
    fsm.add_step(nlist, &mut thread_caps, ip, at);

    assert_eq!(thread_caps[0], Some(0)); // Check if the first capture slot was filled
}

#[test]
#[should_panic]
fn test_add_step_with_duplicate_ip() {
    use prog::{Inst::Save, InstSave, InstPtr};

    let prog = Program {
        insts: vec![
            Save(InstSave { goto: 1, slot: 0 }),  // Saving will happen here
            Match(0),                             // This will be the matching instruction
        ],
        matches: vec![InstPtr(0)],
        captures: vec![],
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

    let nlist = &mut Threads::new();
    let mut thread_caps = vec![None; 1]; // Valid thread_caps of size 1
    let mut stack: Vec<FollowEpsilon> = vec![];

    let mut fsm = Fsm {
        prog: &prog,
        stack: &mut stack,
        input: MockInput::new(),
    };

    let at = InputAt { pos: 0, c: Char::new(0), byte: None, len: 1 };
    let ip = 0;  // Starting IP

    // Simulate adding the same ip again which should panic
    nlist.set.insert(ip);  // This line simulates the duplication before calling add_step

    let _ = std::panic::catch_unwind(|| {
        fsm.add_step(nlist, &mut thread_caps, ip, at);
    });
}

