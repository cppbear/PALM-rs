// Answer 0

#[test]
fn test_backtrack_success_single_match() {
    struct MockInput {
        data: Vec<u8>,
    }

    impl Input for MockInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt {
                pos: i,
                c: self.data[i] as char,
                byte: Some(self.data[i]),
                len: self.data.len(),
            }
        }

        fn next_char(&self, at: InputAt) -> char {
            self.data[at.pos + 1] as char
        }

        fn previous_char(&self, at: InputAt) -> char {
            self.data[at.pos - 1] as char
        }

        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool {
            false
        }

        fn prefix_at(&self, _prefixes: &LiteralSearcher, _at: InputAt) -> Option<InputAt> {
            None
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }
        
        fn as_bytes(&self) -> &[u8] {
            &self.data
        }
    }

    let program = Program {
        insts: vec![],
        matches: vec![InstPtr::default()],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr::default(),
        byte_classes: vec![],
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

    let mut cache = Cache::default();
    let matches = &mut vec![false];
    let slots = &mut vec![None; 256];

    let input = MockInput { data: vec![b'a', b'b', b'c'] };
    let mut bounded = Bounded {
        prog: &program,
        input,
        matches,
        slots,
        m: &mut cache,
    };

    let start = InputAt {
        pos: 0,
        c: 'a',
        byte: Some(b'a'),
        len: 3,
    };

    let result = bounded.backtrack(start);
    assert!(result);
}

#[test]
fn test_backtrack_no_match() {
    struct MockInput {
        data: Vec<u8>,
    }

    impl Input for MockInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt {
                pos: i,
                c: self.data[i] as char,
                byte: Some(self.data[i]),
                len: self.data.len(),
            }
        }

        fn next_char(&self, at: InputAt) -> char {
            self.data[at.pos + 1] as char
        }

        fn previous_char(&self, at: InputAt) -> char {
            self.data[at.pos - 1] as char
        }

        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool {
            false
        }

        fn prefix_at(&self, _prefixes: &LiteralSearcher, _at: InputAt) -> Option<InputAt> {
            None
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }
        
        fn as_bytes(&self) -> &[u8] {
            &self.data
        }
    }

    let program = Program {
        insts: vec![],
        matches: vec![InstPtr::default()],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr::default(),
        byte_classes: vec![],
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

    let mut cache = Cache::default();
    let matches = &mut vec![false];
    let slots = &mut vec![None; 256];

    let input = MockInput { data: vec![b'd', b'e', b'f'] };
    let mut bounded = Bounded {
        prog: &program,
        input,
        matches,
        slots,
        m: &mut cache,
    };

    let start = InputAt {
        pos: 0,
        c: 'd',
        byte: Some(b'd'),
        len: 3,
    };

    let result = bounded.backtrack(start);
    assert!(!result);
}

