// Answer 0

#[test]
fn test_is_empty_match_end_line_false() {
    struct Input {
        content: String,
        only_utf8: bool,
    }

    impl Input {
        fn previous_char(&self, at: InputAt) -> char {
            if at.pos() == 0 {
                '\0'
            } else {
                self.content.chars().nth(at.pos() - 1).unwrap_or('\0')
            }
        }

        fn next_char(&self, at: InputAt) -> char {
            if at.pos() >= self.content.len() {
                '\0'
            } else {
                self.content.chars().nth(at.pos()).unwrap_or('\0')
            }
        }

        fn len(&self) -> usize {
            self.content.len()
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

        fn is_end(&self, content_length: usize) -> bool {
            self.position >= content_length
        } 
    }

    struct InstEmptyLook {
        look: prog::EmptyLook,
    }

    let input = Input {
        content: String::from("hello\nworld"),
        only_utf8: false,
    };

    let at = InputAt { position: 10 }; // at.pos() == 10, which is not equal to length (12)
    
    let empty = InstEmptyLook {
        look: prog::EmptyLook::EndLine,
    };

    let result = input.is_empty_match(at, &empty);
    assert_eq!(result, false); // This test expects false since at.pos() is not equal to the length
}

#[test]
fn test_is_empty_match_end_line_true() {
    struct Input {
        content: String,
        only_utf8: bool,
    }

    impl Input {
        fn previous_char(&self, at: InputAt) -> char {
            if at.pos() == 0 {
                '\0'
            } else {
                self.content.chars().nth(at.pos() - 1).unwrap_or('\0')
            }
        }

        fn next_char(&self, at: InputAt) -> char {
            if at.pos() >= self.content.len() {
                '\0'
            } else {
                self.content.chars().nth(at.pos()).unwrap_or('\0')
            }
        }

        fn len(&self) -> usize {
            self.content.len()
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

        fn is_end(&self, content_length: usize) -> bool {
            self.position >= content_length
        } 
    }

    struct InstEmptyLook {
        look: prog::EmptyLook,
    }

    let input = Input {
        content: String::from("hello\nworld"),
        only_utf8: false,
    };

    let at = InputAt { position: 0 }; // Test at the start of the string
    
    let empty = InstEmptyLook {
        look: prog::EmptyLook::EndLine,
    };

    let result = input.is_empty_match(at, &empty);
    assert_eq!(result, false); // This test expects false since at.pos() is not equal to the length (12)
}

