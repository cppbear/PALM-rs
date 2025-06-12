// Answer 0

#[test]
fn test_prefix_at_valid_input() {
    struct TestInput {
        data: Vec<u8>,
    }

    impl Input for TestInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt { pos: i, c: 'a', byte: Some(self.data[i]), len: 1 }
        }

        fn next_char(&self, at: InputAt) -> Char {
            'a'
        }

        fn previous_char(&self, at: InputAt) -> Char {
            'a'
        }

        fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool {
            false
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn as_bytes(&self) -> &[u8] {
            &self.data
        }
    }

    let input_data = TestInput { data: b"hello world".to_vec() };
    let prefixes = LiteralSearcher::empty();
    let at = InputAt { pos: 0, c: 'h', byte: Some(b'h'), len: 1 };

    let result = input_data.prefix_at(&prefixes, at);
    assert!(result.is_none());
}

#[test]
#[should_panic]
fn test_prefix_at_panic_out_of_bounds() {
    struct TestInput {
        data: Vec<u8>,
    }

    impl Input for TestInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt { pos: i, c: 'a', byte: Some(self.data[i]), len: 1 }
        }

        fn next_char(&self, at: InputAt) -> Char {
            'a'
        }

        fn previous_char(&self, at: InputAt) -> Char {
            'a'
        }

        fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool {
            false
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn as_bytes(&self) -> &[u8] {
            &self.data
        }
    }

    let input_data = TestInput { data: b"hello world".to_vec() };
    let prefixes = LiteralSearcher::empty();
    let at = InputAt { pos: 15, c: 'a', byte: None, len: 0 }; // Out of bounds position

    // This will panic due to the out of bounds in `self[at.pos()..]`
    let _ = input_data.prefix_at(&prefixes, at);
}

#[test]
fn test_prefix_at_empty_input() {
    struct TestInput {
        data: Vec<u8>,
    }

    impl Input for TestInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt { pos: i, c: 'a', byte: Some(self.data[i]), len: 1 }
        }

        fn next_char(&self, at: InputAt) -> Char {
            'a'
        }

        fn previous_char(&self, at: InputAt) -> Char {
            'a'
        }

        fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool {
            false
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn as_bytes(&self) -> &[u8] {
            &self.data
        }
    }

    let input_data = TestInput { data: Vec::new() };
    let prefixes = LiteralSearcher::empty();
    let at = InputAt { pos: 0, c: 'a', byte: None, len: 0 }; // Position 0 in empty input

    let result = input_data.prefix_at(&prefixes, at);
    assert!(result.is_none());
}

