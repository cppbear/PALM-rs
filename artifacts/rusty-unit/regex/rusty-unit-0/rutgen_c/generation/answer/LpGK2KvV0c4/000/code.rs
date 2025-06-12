// Answer 0

#[test]
fn test_step_match() {
    struct TestInput;
    
    impl Input for TestInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt { pos: i, c: Char(0), byte: None, len: 1 }
        }
        fn next_char(&self, at: InputAt) -> Char {
            Char(0)
        }
        fn previous_char(&self, at: InputAt) -> Char {
            Char(0)
        }
        fn is_empty_match(&self, at: InputAt, _empty: &InstEmptyLook) -> bool {
            false
        }
        fn prefix_at(&self, _prefixes: &LiteralSearcher, _at: InputAt) -> Option<InputAt> {
            None
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

    let prog = Program { insts: vec![Inst::Match(0)], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::new(), dfa_size_limit: 0 };
    let mut matches = vec![false];
    let mut slots = vec![None];
    let mut cache = Cache { jobs: vec![], visited: vec![0] };
    let mut bounded = Bounded { prog: &prog, input: TestInput, matches: &mut matches, slots: &mut slots, m: &mut cache };
    
    let result = bounded.step(0, InputAt { pos: 0, c: Char(0), byte: None, len: 1 });
    assert!(result);
    assert!(matches[0]);
}

#[test]
fn test_step_empty_look() {
    struct TestInput;

    impl Input for TestInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt { pos: i, c: Char(0), byte: None, len: 1 }
        }
        fn next_char(&self, at: InputAt) -> Char {
            Char(0)
        }
        fn previous_char(&self, at: InputAt) -> Char {
            Char(0)
        }
        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool {
            true
        }
        fn prefix_at(&self, _prefixes: &LiteralSearcher, _at: InputAt) -> Option<InputAt> {
            None
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

    let prog = Program { insts: vec![Inst::EmptyLook(InstEmptyLook { goto: 0, look: EmptyLook::ZeroWidth })], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::new(), dfa_size_limit: 0 };
    let mut matches = vec![false];
    let mut slots = vec![None];
    let mut cache = Cache { jobs: vec![], visited: vec![0] };
    let mut bounded = Bounded { prog: &prog, input: TestInput, matches: &mut matches, slots: &mut slots, m: &mut cache };
    
    let result = bounded.step(0, InputAt { pos: 0, c: Char(0), byte: None, len: 1 });
    assert!(result);
}

#[test]
fn test_step_char_fail() {
    struct TestInput;

    impl Input for TestInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt { pos: i, c: Char(1), byte: None, len: 1 }
        }
        fn next_char(&self, at: InputAt) -> Char {
            Char(1)
        }
        fn previous_char(&self, at: InputAt) -> Char {
            Char(0)
        }
        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool {
            false
        }
        fn prefix_at(&self, _prefixes: &LiteralSearcher, _at: InputAt) -> Option<InputAt> {
            None
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

    let prog = Program { insts: vec![Inst::Char(InstChar { goto: 0, c: 'b' })], matches: vec![], captures: vec![], capture_name_idx: Arc::new(HashMap::new()), start: 0, byte_classes: vec![], only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::new(), dfa_size_limit: 0 };
    let mut matches = vec![false];
    let mut slots = vec![None];
    let mut cache = Cache { jobs: vec![], visited: vec![0] };
    let mut bounded = Bounded { prog: &prog, input: TestInput, matches: &mut matches, slots: &mut slots, m: &mut cache };

    let result = bounded.step(0, InputAt { pos: 0, c: Char(1), byte: None, len: 1 });
    assert!(!result);
}

