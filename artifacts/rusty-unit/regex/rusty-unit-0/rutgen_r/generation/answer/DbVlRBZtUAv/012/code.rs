// Answer 0

#[test]
fn test_is_empty_match_word_boundary_ascii_false() {
    struct TestInput {
        only_utf8: bool,
        text: &'static str,
    }

    impl TestInput {
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
            self.position >= self.text.len()
        }
    }

    struct InstEmptyLook {
        look: prog::EmptyLook,
    }

    let input = TestInput {
        only_utf8: true,
        text: "abc",
    };

    let at = InputAt { position: 1 }; // Not at start
    let empty_look = InstEmptyLook {
        look: prog::EmptyLook::WordBoundaryAscii,
    };

    let result = input.is_empty_match(at, &empty_look);
    assert_eq!(result, false);
}

