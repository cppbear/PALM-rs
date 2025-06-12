// Answer 0

#[test]
fn test_exec_basic_match() {
    let prog = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![InstPtr(0)],
        captures: vec![None],
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
    
    struct TestInput {
        data: Vec<u8>,
    }
    
    impl Input for TestInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt { pos: i, c: Char::from(self.data[i]), byte: Some(self.data[i]), len: 1 }
        }
        
        fn next_char(&self, at: InputAt) -> Char { Char::from(self.data[at.pos + 1]) }
        fn previous_char(&self, at: InputAt) -> Char { Char::from(self.data[at.pos - 1]) }
        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool { false }
        fn prefix_at(&self, _prefixes: &LiteralSearcher, at: InputAt) -> Option<InputAt> { Some(at) }
        fn len(&self) -> usize { self.data.len() }
        fn is_empty(&self) -> bool { self.data.is_empty() }
        fn as_bytes(&self) -> &[u8] { &self.data }
    }
    
    let input = TestInput { data: vec![b'a', b'b', b'c'] };
    let mut matches = vec![false; 1];
    let mut slots = vec![Slot; 1];
    
    exec(&prog, &ProgramCache::default(), &mut matches, &mut slots, false, input, 0);
}

#[test]
fn test_exec_no_match() {
    let prog = Program {
        insts: vec![Inst::Char(InstChar(b'z'))],
        matches: vec![InstPtr(0)],
        captures: vec![None],
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

    struct TestInput {
        data: Vec<u8>,
    }
    
    impl Input for TestInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt { pos: i, c: Char::from(self.data[i]), byte: Some(self.data[i]), len: 1 }
        }
        
        fn next_char(&self, at: InputAt) -> Char { Char::from(self.data[at.pos + 1]) }
        fn previous_char(&self, at: InputAt) -> Char { Char::from(self.data[at.pos - 1]) }
        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool { false }
        fn prefix_at(&self, _prefixes: &LiteralSearcher, at: InputAt) -> Option<InputAt> { Some(at) }
        fn len(&self) -> usize { self.data.len() }
        fn is_empty(&self) -> bool { self.data.is_empty() }
        fn as_bytes(&self) -> &[u8] { &self.data }
    }
    
    let input = TestInput { data: vec![b'a', b'b', b'c'] };
    let mut matches = vec![false; 1];
    let mut slots = vec![Slot; 1];
    
    exec(&prog, &ProgramCache::default(), &mut matches, &mut slots, false, input, 0);
}

#[test]
fn test_exec_with_quit_after_match() {
    let prog = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![InstPtr(0)],
        captures: vec![None],
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
    
    struct TestInput {
        data: Vec<u8>,
    }
    
    impl Input for TestInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt { pos: i, c: Char::from(self.data[i]), byte: Some(self.data[i]), len: 1 }
        }
        
        fn next_char(&self, at: InputAt) -> Char { Char::from(self.data[at.pos + 1]) }
        fn previous_char(&self, at: InputAt) -> Char { Char::from(self.data[at.pos - 1]) }
        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool { false }
        fn prefix_at(&self, _prefixes: &LiteralSearcher, at: InputAt) -> Option<InputAt> { Some(at) }
        fn len(&self) -> usize { self.data.len() }
        fn is_empty(&self) -> bool { self.data.is_empty() }
        fn as_bytes(&self) -> &[u8] { &self.data }
    }
    
    let input = TestInput { data: vec![b'a', b'b', b'c', b'd'] };
    let mut matches = vec![false; 1];
    let mut slots = vec![Slot; 1];
    
    exec(&prog, &ProgramCache::default(), &mut matches, &mut slots, true, input, 0);
}

#[test]
fn test_exec_empty_input() {
    let prog = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![InstPtr(0)],
        captures: vec![None],
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

    struct TestInput {
        data: Vec<u8>,
    }

    impl Input for TestInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt { pos: i, c: Char::from(self.data[i]), byte: Some(self.data[i]), len: 1 }
        }

        fn next_char(&self, _at: InputAt) -> Char { Char::from(0) }
        fn previous_char(&self, _at: InputAt) -> Char { Char::from(0) }
        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool { false }
        fn prefix_at(&self, _prefixes: &LiteralSearcher, at: InputAt) -> Option<InputAt> { Some(at) }
        fn len(&self) -> usize { self.data.len() }
        fn is_empty(&self) -> bool { self.data.is_empty() }
        fn as_bytes(&self) -> &[u8] { &self.data }
    }

    let input = TestInput { data: vec![] };
    let mut matches = vec![false; 1];
    let mut slots = vec![Slot; 1];

    exec(&prog, &ProgramCache::default(), &mut matches, &mut slots, false, input, 0);
}

