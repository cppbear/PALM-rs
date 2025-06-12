// Answer 0

#[test]
fn test_prefix_at_found() {
    #[derive(Clone, Copy)]
    struct TestInput<'t>(&'t [u8]);

    impl<'t> Input for TestInput<'t> {
        fn at(&self, i: usize) -> InputAt {
            InputAt { pos: i, c: Char::from(self.0[i]), byte: Some(self.0[i]), len: 1 }
        }

        fn next_char(&self, at: InputAt) -> Char {
            self.at(at.pos + 1).c
        }

        fn previous_char(&self, at: InputAt) -> Char {
            self.at(at.pos.checked_sub(1).unwrap_or(0)).c
        }

        fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool {
            false // Simplified for testing
        }

        fn len(&self) -> usize {
            self.0.len()
        }

        fn as_bytes(&self) -> &[u8] {
            self.0
        }
    }

    let input = TestInput(b"hello world");
    let prefixes = LiteralSearcher::prefixes(Literals::from_slice(&[b"hello"]));
    let at = input.at(0);
    
    let result = input.prefix_at(&prefixes, at);
    
    assert!(result.is_some());
    assert_eq!(result.unwrap().pos(), 5);
}

#[test]
fn test_prefix_at_not_found() {
    #[derive(Clone, Copy)]
    struct TestInput<'t>(&'t [u8]);

    impl<'t> Input for TestInput<'t> {
        fn at(&self, i: usize) -> InputAt {
            InputAt { pos: i, c: Char::from(self.0[i]), byte: Some(self.0[i]), len: 1 }
        }

        fn next_char(&self, at: InputAt) -> Char {
            self.at(at.pos + 1).c
        }

        fn previous_char(&self, at: InputAt) -> Char {
            self.at(at.pos.checked_sub(1).unwrap_or(0)).c
        }

        fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool {
            false // Simplified for testing
        }

        fn len(&self) -> usize {
            self.0.len()
        }

        fn as_bytes(&self) -> &[u8] {
            self.0
        }
    }

    let input = TestInput(b"goodbye world");
    let prefixes = LiteralSearcher::prefixes(Literals::from_slice(&[b"hello"]));
    let at = input.at(0);
    
    let result = input.prefix_at(&prefixes, at);
    
    assert!(result.is_none());
}

