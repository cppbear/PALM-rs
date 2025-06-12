// Answer 0

#[test]
fn test_previous_char_valid_input() {
    struct InputAt {
        position: usize,
    }

    impl InputAt {
        fn pos(&self) -> usize {
            self.position
        }
    }

    struct TestInput(&'static str);

    impl Deref for TestInput {
        type Target = str;

        fn deref(&self) -> &Self::Target {
            self.0
        }
    }

    fn decode_last_utf8(input: &str) -> Option<(char, usize)> {
        // Mock implementation for testing purposes
        // Assuming it can decode valid utf-8 and returns the last character and its byte length
        if let Some(last_char) = input.chars().last() {
            let last_char_len = last_char.len_utf8();
            return Some((last_char, last_char_len));
        }
        None
    }

    impl TestInput {
        fn previous_char(&self, at: InputAt) -> char {
            decode_last_utf8(&self[..at.pos()]).map(|(c, _)| c).unwrap_or('\0')
        }
    }

    let input = TestInput("Hello, World!");
    let at = InputAt { position: 13 }; // last character position
    assert_eq!(input.previous_char(at), '!');
}

#[test]
fn test_previous_char_empty_input() {
    struct InputAt {
        position: usize,
    }

    impl InputAt {
        fn pos(&self) -> usize {
            self.position
        }
    }

    struct TestInput(&'static str);

    impl Deref for TestInput {
        type Target = str;

        fn deref(&self) -> &Self::Target {
            self.0
        }
    }

    fn decode_last_utf8(input: &str) -> Option<(char, usize)> {
        if let Some(last_char) = input.chars().last() {
            let last_char_len = last_char.len_utf8();
            return Some((last_char, last_char_len));
        }
        None
    }

    impl TestInput {
        fn previous_char(&self, at: InputAt) -> char {
            decode_last_utf8(&self[..at.pos()]).map(|(c, _)| c).unwrap_or('\0')
        }
    }

    let input = TestInput("");
    let at = InputAt { position: 0 }; // invalid position
    assert_eq!(input.previous_char(at), '\0');
}

