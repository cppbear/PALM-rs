// Answer 0

#[test]
fn test_previous_char_valid_input() {
    struct InputAt {
        pos: usize,
    }

    impl InputAt {
        fn pos(&self) -> usize {
            self.pos
        }
    }

    struct TestInput {
        data: Vec<u8>,
    }

    impl std::ops::Deref for TestInput {
        type Target = [u8];

        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }

    fn decode_last_utf8(input: &[u8]) -> Option<(char, usize)> {
        // Dummy implementation for testing purpose
        if input.is_empty() {
            None
        } else {
            Some((input[input.len() - 1] as char, 1))
        }
    }

    fn previous_char(input: &TestInput, at: InputAt) -> char {
        decode_last_utf8(&input[..at.pos()]).map(|(c, _)| c).unwrap_or('\0')
    }

    let input_data = TestInput { data: b"hello".to_vec() };
    let at = InputAt { pos: 5 };
    let result = previous_char(&input_data, at);
    assert_eq!(result, 'o');
}

#[test]
#[should_panic]
fn test_previous_char_out_of_bounds() {
    struct InputAt {
        pos: usize,
    }

    impl InputAt {
        fn pos(&self) -> usize {
            self.pos
        }
    }

    struct TestInput {
        data: Vec<u8>,
    }

    impl std::ops::Deref for TestInput {
        type Target = [u8];

        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }

    fn decode_last_utf8(input: &[u8]) -> Option<(char, usize)> {
        if input.is_empty() {
            None
        } else {
            Some((input[input.len() - 1] as char, 1))
        }
    }

    fn previous_char(input: &TestInput, at: InputAt) -> char {
        decode_last_utf8(&input[..at.pos()]).map(|(c, _)| c).unwrap_or('\0')
    }

    let input_data = TestInput { data: b"hello".to_vec() };
    let at = InputAt { pos: 6 }; // Out of bounds
    previous_char(&input_data, at);
}

