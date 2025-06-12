// Answer 0

#[test]
fn test_is_empty_match_start_line_at_start_position() {
    struct DummyInput {
        content: String,
        only_utf8: bool,
    }

    impl DummyInput {
        fn previous_char(&self, at: InputAt) -> char {
            if at.pos() == 0 {
                '\0' // Simulate the start of text
            } else {
                self.content.chars().nth(at.pos() - 1).unwrap_or('\0')
            }
        }

        fn next_char(&self, at: InputAt) -> char {
            self.content.chars().nth(at.pos()).unwrap_or('\0')
        }

        fn len(&self) -> usize {
            self.content.len()
        }
    }

    struct InstEmptyLook {
        look: prog::EmptyLook,
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

        fn is_end(&self) -> bool {
            self.position == 0 // Given content is empty in this test
        }
    }

    let input = DummyInput {
        content: String::from(""),
        only_utf8: false,
    };
    let at = InputAt { position: 0 };
    let empty = InstEmptyLook {
        look: prog::EmptyLook::StartLine,
    };

    assert!(input.is_empty_match(at, &empty));
}

#[test]
fn test_is_empty_match_start_line_with_newline_before() {
    struct DummyInput {
        content: String,
        only_utf8: bool,
    }

    impl DummyInput {
        fn previous_char(&self, at: InputAt) -> char {
            if at.pos() == 0 {
                '\n'
            } else {
                self.content.chars().nth(at.pos() - 1).unwrap_or('\0')
            }
        }

        fn next_char(&self, at: InputAt) -> char {
            self.content.chars().nth(at.pos()).unwrap_or('\0')
        }

        fn len(&self) -> usize {
            self.content.len()
        }
    }

    struct InstEmptyLook {
        look: prog::EmptyLook,
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

        fn is_end(&self) -> bool {
            self.position == 0 // Given content is empty in this test
        }
    }

    let input = DummyInput {
        content: String::from("\n"),
        only_utf8: false,
    };
    let at = InputAt { position: 0 };
    let empty = InstEmptyLook {
        look: prog::EmptyLook::StartLine,
    };

    assert!(input.is_empty_match(at, &empty));
}

