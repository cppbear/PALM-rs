// Answer 0

#[test]
fn test_next_char_valid() {
    struct TestInput {
        data: Vec<u8>,
    }

    impl Input for TestInput {
        fn at(&self, i: usize) -> InputAt {
            let c = Char(self.data[i] as u32);
            InputAt {
                pos: i,
                c,
                byte: Some(self.data[i]),
                len: self.data.len(),
            }
        }

        fn next_char(&self, at: InputAt) -> Char {
            if at.pos + 1 < self.data.len() {
                Char(self.data[at.pos + 1] as u32)
            } else {
                Char(0) // or any valid Char you want for "no next char"
            }
        }

        fn previous_char(&self, at: InputAt) -> Char {
            if at.pos > 0 {
                Char(self.data[at.pos - 1] as u32)
            } else {
                Char(0) // or any valid Char you want for "no previous char"
            }
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

        fn as_bytes(&self) -> &[u8] {
            &self.data
        }
    }

    let input = TestInput { data: vec![b'a', b'b', b'c'] };
    let at = input.at(1); // Position of 'b'

    let next_char = (&input).next_char(at);

    assert_eq!(next_char.0, b'c' as u32);
}

#[test]
fn test_next_char_boundary() {
    struct TestInput {
        data: Vec<u8>,
    }

    impl Input for TestInput {
        fn at(&self, i: usize) -> InputAt {
            let c = Char(self.data[i] as u32);
            InputAt {
                pos: i,
                c,
                byte: Some(self.data[i]),
                len: self.data.len(),
            }
        }

        fn next_char(&self, at: InputAt) -> Char {
            if at.pos + 1 < self.data.len() {
                Char(self.data[at.pos + 1] as u32)
            } else {
                Char(0) // or any valid Char you want for "no next char"
            }
        }

        fn previous_char(&self, at: InputAt) -> Char {
            if at.pos > 0 {
                Char(self.data[at.pos - 1] as u32)
            } else {
                Char(0) // or any valid Char you want for "no previous char"
            }
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

        fn as_bytes(&self) -> &[u8] {
            &self.data
        }
    }

    let input = TestInput { data: vec![b'a', b'b', b'c'] };
    let at = input.at(2); // Position of 'c', last character

    let next_char = (&input).next_char(at);

    assert_eq!(next_char.0, 0); // Check for no next character
}

