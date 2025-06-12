// Answer 0

#[test]
fn test_prefix_at_valid() {
    struct DummyInput<'t>(&'t [u8]);

    impl<'t> Input for DummyInput<'t> {
        fn at(&self, i: usize) -> InputAt {
            InputAt { pos: i, c: char::from(self.0[i]), byte: Some(self.0[i]), len: 1 }
        }

        fn next_char(&self, at: InputAt) -> Char {
            // Simplified for this test
            Char::from(self.0[at.pos + 1])
        }

        fn previous_char(&self, at: InputAt) -> Char {
            // Simplified for this test
            Char::from(self.0[at.pos - 1])
        }

        fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool {
            false
        }

        fn prefix_at(&self, prefixes: &LiteralSearcher, at: InputAt) -> Option<InputAt> {
            prefixes.find(&self.0[at.pos..]).map(|(s, _)| self.at(at.pos + s))
        }

        fn len(&self) -> usize {
            self.0.len()
        }

        fn as_bytes(&self) -> &[u8] {
            self.0
        }
    }

    let haystack = b"hello world";
    let prefix = LiteralSearcher::prefixes(Literals::from(vec![b"hello".to_vec()]));
    let input = DummyInput(haystack);
    let at = InputAt { pos: 0, c: 'h', byte: Some(b'h'), len: 5 };
    let result = input.prefix_at(&prefix, at);
    assert!(result.is_some());
    assert_eq!(result.unwrap().pos(), 5);
}

#[test]
#[should_panic]
fn test_prefix_at_out_of_bounds() {
    struct DummyInput<'t>(&'t [u8]);

    impl<'t> Input for DummyInput<'t> {
        fn at(&self, i: usize) -> InputAt {
            InputAt { pos: i, c: char::from(self.0[i]), byte: Some(self.0[i]), len: 1 }
        }

        fn next_char(&self, at: InputAt) -> Char {
            Char::from(self.0[at.pos + 1])
        }

        fn previous_char(&self, at: InputAt) -> Char {
            Char::from(self.0[at.pos - 1])
        }

        fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool {
            false
        }

        fn prefix_at(&self, prefixes: &LiteralSearcher, at: InputAt) -> Option<InputAt> {
            prefixes.find(&self.0[at.pos..]).map(|(s, _)| self.at(at.pos + s))
        }

        fn len(&self) -> usize {
            self.0.len()
        }

        fn as_bytes(&self) -> &[u8] {
            self.0
        }
    }

    let haystack = b"hello world";
    let prefix = LiteralSearcher::prefixes(Literals::from(vec![b"hello".to_vec()]));
    let input = DummyInput(haystack);
    let at = InputAt { pos: 12, c: 'd', byte: None, len: 5 }; // Out of bounds
    let _ = input.prefix_at(&prefix, at); // This will panic
}

