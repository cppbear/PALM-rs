// Answer 0

#[test]
fn test_backtrack_with_matching_conditions() {
    struct TestInput;
    
    impl Input for TestInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt { pos: i, c: Char::from(0), byte: None, len: 1 }
        }
        
        fn next_char(&self, at: InputAt) -> Char {
            Char::from(0)
        }
        
        fn previous_char(&self, at: InputAt) -> Char {
            Char::from(0)
        }
        
        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool {
            true
        }
        
        fn prefix_at(&self, _prefixes: &LiteralSearcher, _at: InputAt) -> Option<InputAt> {
            Some(InputAt { pos: 0, c: Char::from(0), byte: None, len: 1 })
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
        insts: Vec::new(),
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };

    let mut cache = Cache { clist: Threads::new(), nlist: Threads::new(), stack: Vec::new() };
    let mut matches = vec![false];
    let mut slots = vec![None];
    
    let mut bounded = Bounded {
        prog: &prog,
        input: TestInput,
        matches: &mut matches,
        slots: &mut slots,
        m: &mut cache,
    };

    let input_at = InputAt { pos: 0, c: Char::from(0), byte: None, len: 1 };
    
    let result = bounded.backtrack(input_at);
    
    assert!(result);
}

