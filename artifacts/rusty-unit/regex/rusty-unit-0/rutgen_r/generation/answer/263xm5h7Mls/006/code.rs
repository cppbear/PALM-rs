// Answer 0

#[test]
fn test_is_empty_match_start_text() {
    struct TestInput {
        data: String,
    }

    impl TestInput {
        fn previous_char(&self, at: InputAt) -> char {
            if at.pos() == 0 { '\0' } else { self.data.chars().nth(at.pos() - 1).unwrap() }
        }

        fn next_char(&self, at: InputAt) -> char {
            if at.pos() >= self.data.len() { '\0' } else { self.data.chars().nth(at.pos()).unwrap() }
        }

        fn len(&self) -> usize {
            self.data.len()
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

    let input = TestInput { data: String::from("") };
    let at = InputAt { position: 0 };
    let empty = InstEmptyLook { look: prog::EmptyLook::StartText };

    assert_eq!(input.is_empty_match(at, &empty), true);
}

#[test]
fn test_is_empty_match_start_text_non_empty() {
    struct TestInput {
        data: String,
    }

    impl TestInput {
        fn previous_char(&self, at: InputAt) -> char {
            if at.pos() == 0 { '\0' } else { self.data.chars().nth(at.pos() - 1).unwrap() }
        }

        fn next_char(&self, at: InputAt) -> char {
            if at.pos() >= self.data.len() { '\0' } else { self.data.chars().nth(at.pos()).unwrap() }
        }

        fn len(&self) -> usize {
            self.data.len()
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

    let input = TestInput { data: String::from("non-empty") };
    let at = InputAt { position: 0 };
    let empty = InstEmptyLook { look: prog::EmptyLook::StartText };

    assert_eq!(input.is_empty_match(at, &empty), true);
}

