// Answer 0

#[test]
fn test_previous_char_valid_input() {
    struct TestInput {
        pos: usize,
        c: Char,
        byte: Option<u8>,
        len: usize,
    }

    impl Input for TestInput {
        fn at(&self, _i: usize) -> InputAt {
            InputAt { pos: self.pos, c: self.c, byte: self.byte, len: self.len }
        }

        fn next_char(&self, at: InputAt) -> Char {
            Char(at.c.0 + 1)
        }

        fn previous_char(&self, at: InputAt) -> Char {
            Char(if at.c.0 > 0 { at.c.0 - 1 } else { 0 })
        }

        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool {
            false
        }

        fn prefix_at(&self, _prefixes: &LiteralSearcher, _at: InputAt) -> Option<InputAt> {
            None
        }

        fn len(&self) -> usize {
            self.len
        }

        fn as_bytes(&self) -> &[u8] {
            &[]
        }
    }

    let test_input = TestInput { pos: 1, c: Char(1), byte: Some(50), len: 3 };
    let input_ref: &dyn Input = &test_input;
    let at = input_ref.at(test_input.pos);
    let _ = input_ref.previous_char(at);
}

#[test]
fn test_previous_char_edge_case_minimum() {
    struct TestInput {
        pos: usize,
        c: Char,
        byte: Option<u8>,
        len: usize,
    }

    impl Input for TestInput {
        fn at(&self, _i: usize) -> InputAt {
            InputAt { pos: self.pos, c: self.c, byte: self.byte, len: self.len }
        }

        fn next_char(&self, at: InputAt) -> Char {
            Char(at.c.0 + 1)
        }

        fn previous_char(&self, at: InputAt) -> Char {
            Char(if at.c.0 > 0 { at.c.0 - 1 } else { 0 })
        }

        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool {
            false
        }

        fn prefix_at(&self, _prefixes: &LiteralSearcher, _at: InputAt) -> Option<InputAt> {
            None
        }

        fn len(&self) -> usize {
            self.len
        }

        fn as_bytes(&self) -> &[u8] {
            &[]
        }
    }

    let test_input = TestInput { pos: 0, c: Char(0), byte: None, len: 1 };
    let input_ref: &dyn Input = &test_input;
    let at = input_ref.at(test_input.pos);
    let _ = input_ref.previous_char(at);
}

#[test]
fn test_previous_char_edge_case_boundary() {
    struct TestInput {
        pos: usize,
        c: Char,
        byte: Option<u8>,
        len: usize,
    }

    impl Input for TestInput {
        fn at(&self, _i: usize) -> InputAt {
            InputAt { pos: self.pos, c: self.c, byte: self.byte, len: self.len }
        }

        fn next_char(&self, at: InputAt) -> Char {
            Char(at.c.0 + 1)
        }

        fn previous_char(&self, at: InputAt) -> Char {
            Char(if at.c.0 > 0 { at.c.0 - 1 } else { 0 })
        }

        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool {
            false
        }

        fn prefix_at(&self, _prefixes: &LiteralSearcher, _at: InputAt) -> Option<InputAt> {
            None
        }

        fn len(&self) -> usize {
            self.len
        }

        fn as_bytes(&self) -> &[u8] {
            &[]
        }
    }

    let test_input = TestInput { pos: 2_u32 as usize, c: Char(2), byte: Some(200), len: 3 };
    let input_ref: &dyn Input = &test_input;
    let at = input_ref.at(test_input.pos);
    let _ = input_ref.previous_char(at);
}

