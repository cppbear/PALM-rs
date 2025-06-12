// Answer 0

#[test]
fn test_prefix_at_empty_input() {
    struct TestInput {
        data: Vec<u8>,
    }

    impl Input for TestInput {
        fn at(&self, _i: usize) -> InputAt {
            InputAt { pos: 0, c: char::default(), byte: None, len: 0 }
        }

        fn next_char(&self, _at: InputAt) -> Char {
            char::default()
        }

        fn previous_char(&self, _at: InputAt) -> Char {
            char::default()
        }

        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool {
            false
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn as_bytes(&self) -> &[u8] {
            &self.data
        }
    }

    let input = TestInput { data: vec![] };
    let searcher = LiteralSearcher { complete: false, lcp: FreqyPacked::new(), lcs: FreqyPacked::new(), matcher: Matcher::new() };
    let at = InputAt { pos: 0, c: char::default(), byte: None, len: 0 };
    
    let result = input.prefix_at(&searcher, at);
    assert!(result.is_none());
}

#[test]
fn test_prefix_at_non_matching_input() {
    struct TestInput {
        data: Vec<u8>,
    }

    impl Input for TestInput {
        fn at(&self, _i: usize) -> InputAt {
            InputAt { pos: 1, c: 'a', byte: None, len: 1 }
        }

        fn next_char(&self, _at: InputAt) -> Char {
            'a'
        }

        fn previous_char(&self, _at: InputAt) -> Char {
            'a'
        }

        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool {
            false
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn as_bytes(&self) -> &[u8] {
            &self.data
        }
    }

    let input = TestInput { data: b"test".to_vec() };
    let searcher = LiteralSearcher { complete: false, lcp: FreqyPacked::new(), lcs: FreqyPacked::new(), matcher: Matcher::new() };
    let at = InputAt { pos: 1, c: 't', byte: None, len: 1 };

    let result = input.prefix_at(&searcher, at);
    assert!(result.is_none());
}

#[test]
fn test_prefix_at_matching_input() {
    struct TestInput {
        data: Vec<u8>,
    }

    impl Input for TestInput {
        fn at(&self, _i: usize) -> InputAt {
            InputAt { pos: 0, c: 't', byte: None, len: 1 }
        }

        fn next_char(&self, _at: InputAt) -> Char {
            't'
        }

        fn previous_char(&self, _at: InputAt) -> Char {
            't'
        }
        
        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool {
            false
        }

        fn len(&self) -> usize {
            self.data.len()
        }
        
        fn as_bytes(&self) -> &[u8] {
            &self.data
        }
    }

    let input = TestInput { data: b"test".to_vec() };
    let searcher = LiteralSearcher { complete: true, lcp: FreqyPacked::new(), lcs: FreqyPacked::new(), matcher: Matcher::new() };
    let at = InputAt { pos: 0, c: 't', byte: None, len: 1 };

    let result = input.prefix_at(&searcher, at);
    assert!(result.is_some());
}

