// Answer 0

#[test]
fn test_is_empty_match_end_line_not_at_end() {
    struct Input {
        data: String,
    }

    impl Input {
        fn len(&self) -> usize {
            self.data.len()
        }

        fn previous_char(&self, at: InputAt) -> char {
            self.data.chars().nth(at.pos().saturating_sub(1)).unwrap_or('\0')
        }

        fn next_char(&self, at: InputAt) -> char {
            self.data.chars().nth(at.pos()).unwrap_or('\0')
        }
    }

    #[derive(Clone, Copy)]
    struct InputAt {
        position: usize,
    }

    impl InputAt {
        fn pos(&self) -> usize {
            self.position
        }
    }

    struct InstEmptyLook {
        look: prog::EmptyLook,
    }

    mod prog {
        pub enum EmptyLook {
            EndLine,
        }
    }

    let input = Input { data: String::from("Hello\nWorld") };
    let at = InputAt { position: 5 }; // Not at the end of the input
    let empty = InstEmptyLook { look: prog::EmptyLook::EndLine };

    let result = input.is_empty_match(at, &empty);
    assert_eq!(result, false);
}

#[test]
fn test_is_empty_match_end_line_at_end() {
    struct Input {
        data: String,
    }

    impl Input {
        fn len(&self) -> usize {
            self.data.len()
        }

        fn previous_char(&self, at: InputAt) -> char {
            self.data.chars().nth(at.pos().saturating_sub(1)).unwrap_or('\0')
        }

        fn next_char(&self, at: InputAt) -> char {
            self.data.chars().nth(at.pos()).unwrap_or('\0')
        }
    }

    #[derive(Clone, Copy)]
    struct InputAt {
        position: usize,
    }

    impl InputAt {
        fn pos(&self) -> usize {
            self.position
        }
    }

    struct InstEmptyLook {
        look: prog::EmptyLook,
    }

    mod prog {
        pub enum EmptyLook {
            EndLine,
        }
    }

    let input = Input { data: String::from("Hello\nWorld\n") };
    let at = InputAt { position: 11 }; // At the end of the input
    let empty = InstEmptyLook { look: prog::EmptyLook::EndLine };

    let result = input.is_empty_match(at, &empty);
    assert_eq!(result, true);
}

