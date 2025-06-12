// Answer 0

#[test]
fn test_step_save_instruction() {
    use std::sync::Arc;
    use std::collections::HashMap;

    struct MockInput;

    impl Input for MockInput {
        fn at(&self, _: usize) -> InputAt { InputAt { pos: 0, c: Char(0), byte: None, len: 0 } }
        fn next_char(&self, _: InputAt) -> Char { Char(0) }
        fn previous_char(&self, _: InputAt) -> Char { Char(0) }
        fn is_empty_match(&self, _: &InstEmptyLook) -> bool { false }
        fn prefix_at(&self, _: &LiteralSearcher, _: InputAt) -> Option<InputAt> { None }
        fn len(&self) -> usize { 0 }
        fn is_empty(&self) -> bool { true }
        fn as_bytes(&self) -> &[u8] { &[] }
    }

    let program = Program {
        insts: vec![Inst::Save(InstSave),],
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
        prefixes: LiteralSearcher,
        dfa_size_limit: 0,
    };

    let mut stack = Vec::new();
    let mut nlist = Threads { set: SparseSet::default(), caps: vec![], slots_per_thread: 0 };
    let mut matches = vec![false];
    let mut slots = vec![Slot::default()];
    let mut thread_caps = vec![None];
    
    let mut fsm = Fsm { prog: &program, stack: &mut stack, input: MockInput };

    let result = fsm.step(&mut nlist, &mut matches, &mut slots, &mut thread_caps, 0, InputAt { pos: 0, c: Char(0), byte: None, len: 0 }, InputAt { pos: 1, c: Char(1), byte: None, len: 1 });

    assert!(!result);
}

