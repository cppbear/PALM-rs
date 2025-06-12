// Answer 0

#[test]
fn test_step_empty_look() {
    struct MockInput;
    
    impl Input for MockInput {
        fn at(&self, _: usize) -> InputAt {
            InputAt { pos: 0, c: Char(0), byte: None, len: 1 }
        }
        
        fn next_char(&self, _: InputAt) -> Char { Char(0) }
        
        fn previous_char(&self, _: InputAt) -> Char { Char(0) }
        
        fn is_empty_match(&self, _: &InstEmptyLook) -> bool { false }
        
        fn prefix_at(&self, _: &LiteralSearcher, _: InputAt) -> Option<InputAt> { None }
        
        fn len(&self) -> usize { 1 }
        
        fn is_empty(&self) -> bool { false }
        
        fn as_bytes(&self) -> &[u8] { &[0] }
    }

    let program = Program {
        insts: vec![prog::Inst::EmptyLook(InstEmptyLook)],
        matches: vec![],
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
        prefixes: LiteralSearcher,
        dfa_size_limit: 10,
    };

    let mut stack = vec![];
    let mut matches = vec![false];
    let mut slots = vec![];
    let mut thread_caps = vec![None];
    let input = MockInput;

    let mut fsm = Fsm { prog: &program, stack: &mut stack, input };
    
    let result = fsm.step(
        &mut Threads { set: SparseSet, caps: vec![], slots_per_thread: 0 },
        &mut matches,
        &mut slots,
        &mut thread_caps,
        0,
        fsm.input.at(0),
        fsm.input.at(1),
    );

    assert_eq!(result, false);
}

