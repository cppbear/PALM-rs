// Answer 0

#[test]
fn test_is_empty_match_word_boundary_ascii() {
    struct Input {
        only_utf8: bool,
        text: String,
    }

    impl Input {
        fn previous_char(&self, at: InputAt) -> Option<char> {
            if at.pos() == 0 {
                None
            } else {
                self.text.chars().nth(at.pos() - 1)
            }
        }

        fn next_char(&self, at: InputAt) -> Option<char> {
            if at.pos() == self.text.len() {
                None
            } else {
                self.text.chars().nth(at.pos())
            }
        }

        fn len(&self) -> usize {
            self.text.len()
        }
    }

    struct InputAt {
        pos: usize,
    }

    impl InputAt {
        fn pos(&self) -> usize {
            self.pos
        }

        fn is_start(&self) -> bool {
            self.pos == 0
        }

        fn is_end(&self) -> bool {
            self.pos == self.text.len()
        }
    }

    struct InstEmptyLook {
        look: prog::EmptyLook,
    }

    let input = Input {
        only_utf8: true,
        text: String::from("a"), // A single word boundary
    };

    let at = InputAt { pos: 1 }; // Cursor at the end of the text
    let empty = InstEmptyLook {
        look: prog::EmptyLook::WordBoundaryAscii,
    };

    let result = input.is_empty_match(at, &empty);
    assert_eq!(result, true);
}

#[test]
#[should_panic]
fn test_is_empty_match_word_boundary_ascii_panic() {
    struct Input {
        only_utf8: bool,
        text: String,
    }

    impl Input {
        fn previous_char(&self, at: InputAt) -> Option<char> {
            if at.pos() == 0 {
                None
            } else {
                self.text.chars().nth(at.pos() - 1)
            }
        }

        fn next_char(&self, at: InputAt) -> Option<char> {
            if at.pos() == self.text.len() {
                None // This should trigger panic
            } else {
                self.text.chars().nth(at.pos())
            }
        }

        fn len(&self) -> usize {
            self.text.len()
        }
    }

    struct InputAt {
        pos: usize,
    }

    impl InputAt {
        fn pos(&self) -> usize {
            self.pos
        }

        fn is_start(&self) -> bool {
            self.pos == 0
        }

        fn is_end(&self) -> bool {
            self.pos == self.text.len()
        }
    }

    struct InstEmptyLook {
        look: prog::EmptyLook,
    }

    let input = Input {
        only_utf8: true,
        text: String::from("a"),
    };

    let at = InputAt { pos: 1 }; // Cursor at the end of the text
    let empty = InstEmptyLook {
        look: prog::EmptyLook::WordBoundaryAscii,
    };

    let _ = input.is_empty_match(at, &empty); // This should panic
}

