// Answer 0

#[test]
fn test_is_empty_match_word_boundary() {
    struct DummyInput {
        data: String,
        only_utf8: bool,
    }

    impl DummyInput {
        fn previous_char(&self, at: InputAt) -> char {
            let pos = at.pos();
            if pos > 0 {
                self.data.chars().nth(pos - 1).unwrap_or(' ')
            } else {
                ' '
            }
        }

        fn next_char(&self, at: InputAt) -> char {
            let pos = at.pos();
            self.data.chars().nth(pos).unwrap_or(' ')
        }

        fn len(&self) -> usize {
            self.data.len()
        }
    }

    impl DummyInput {
        fn is_word_char(c: char) -> bool {
            c.is_alphanumeric() || c == '_'
        }

        fn is_word_byte(c: char) -> bool {
            c.is_ascii_alphanumeric() || c == '_'
        }
    }

    struct InputAt {
        position: usize,
    }

    impl InputAt {
        fn pos(&self) -> usize {
            self.position
        }

        fn is_start(&self) -> bool {
            self.position == 0
        }

        fn is_end(&self, input_len: usize) -> bool {
            self.position >= input_len
        }
    }
    
    struct InstEmptyLook {
        look: prog::EmptyLook,
    }
    
    let input_data = "hello world";
    let input = DummyInput {
        data: input_data.to_string(),
        only_utf8: true,
    };
    
    let at_boundary = InputAt { position: 5 }; // position between "hello" and "world"

    // Test for WordBoundary
    let empty_look_boundary = InstEmptyLook {
        look: prog::EmptyLook::WordBoundary,
    };

    assert_eq!(input.is_empty_match(at_boundary, &empty_look_boundary), true);

    let at_inside_word = InputAt { position: 4 }; // position inside "hello"
    
    // Test for NotWordBoundary
    let empty_look_not_boundary = InstEmptyLook {
        look: prog::EmptyLook::NotWordBoundary,
    };

    assert_eq!(input.is_empty_match(at_inside_word, &empty_look_not_boundary), false);
}

