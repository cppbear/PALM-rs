// Answer 0

#[test]
fn test_is_empty_match_end_text() {
    struct MockInput {
        content: String,
    }

    impl MockInput {
        fn len(&self) -> usize {
            self.content.len()
        }

        fn previous_char(&self, at: InputAt) -> char {
            if at.pos() > 0 {
                self.content.chars().nth(at.pos() - 1).unwrap_or(' ')
            } else {
                ' '
            }
        }

        fn next_char(&self, at: InputAt) -> char {
            if at.pos() < self.len() {
                self.content.chars().nth(at.pos()).unwrap_or(' ')
            } else {
                ' '
            }
        }
    }

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

    let input = MockInput { content: String::from("test") };
    let empty_look = InstEmptyLook { look: prog::EmptyLook::EndText };
    let at = InputAt { position: input.len() };

    assert_eq!(input.is_empty_match(at, &empty_look), true);
}

#[test]
fn test_is_empty_match_not_end_text() {
    struct MockInput {
        content: String,
    }

    impl MockInput {
        fn len(&self) -> usize {
            self.content.len()
        }

        fn previous_char(&self, at: InputAt) -> char {
            if at.pos() > 0 {
                self.content.chars().nth(at.pos() - 1).unwrap_or(' ')
            } else {
                ' '
            }
        }

        fn next_char(&self, at: InputAt) -> char {
            if at.pos() < self.len() {
                self.content.chars().nth(at.pos()).unwrap_or(' ')
            } else {
                ' '
            }
        }
    }

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

    let input = MockInput { content: String::from("test") };
    let empty_look = InstEmptyLook { look: prog::EmptyLook::EndText };
    let at = InputAt { position: input.len() + 1 }; // Going out of bounds

    assert_eq!(input.is_empty_match(at, &empty_look), false);
}

