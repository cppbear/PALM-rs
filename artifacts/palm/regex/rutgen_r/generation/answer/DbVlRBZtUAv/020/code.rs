// Answer 0

#[test]
fn test_is_empty_match_start_text_true() {
    struct Input {
        content: String,
        only_utf8: bool,
    }

    impl Input {
        fn len(&self) -> usize {
            self.content.len()
        }

        fn previous_char(&self, at: InputAt) -> char {
            if at.pos() == 0 {
                '\0' // No character before the start
            } else {
                self.content.chars().nth(at.pos() - 1).unwrap_or('\0')
            }
        }

        fn next_char(&self, at: InputAt) -> char {
            self.content.chars().nth(at.pos()).unwrap_or('\0') // Default to null char if out of bounds
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

        fn is_end(&self, len: usize) -> bool {
            self.position == len
        }
    }

    struct InstEmptyLook {
        look: prog::EmptyLook,
    }

    let input = Input {
        content: String::from(""),
        only_utf8: false,
    };
    let at = InputAt { position: 0 };
    let empty = InstEmptyLook { look: prog::EmptyLook::StartText };

    assert_eq!(input.is_empty_match(at, &empty), true);
}

#[test]
fn test_is_empty_match_start_text_non_zero() {
    struct Input {
        content: String,
        only_utf8: bool,
    }

    impl Input {
        fn len(&self) -> usize {
            self.content.len()
        }

        fn previous_char(&self, at: InputAt) -> char {
            if at.pos() == 0 {
                '\0' 
            } else {
                self.content.chars().nth(at.pos() - 1).unwrap_or('\0')
            }
        }

        fn next_char(&self, at: InputAt) -> char {
            self.content.chars().nth(at.pos()).unwrap_or('\0')
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

        fn is_end(&self, len: usize) -> bool {
            self.position == len
        }
    }

    struct InstEmptyLook {
        look: prog::EmptyLook,
    }

    let input = Input {
        content: String::from("abc"),
        only_utf8: false,
    };
    let at = InputAt { position: 1 };
    let empty = InstEmptyLook { look: prog::EmptyLook::StartText };

    assert_eq!(input.is_empty_match(at, &empty), false);
}

