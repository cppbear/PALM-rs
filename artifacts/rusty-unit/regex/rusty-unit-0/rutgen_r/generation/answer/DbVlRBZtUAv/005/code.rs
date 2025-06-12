// Answer 0

#[test]
fn test_is_empty_match_not_word_boundary_ascii() {
    struct MockInput {
        only_utf8: bool,
        content: Vec<u8>,
    }

    impl MockInput {
        fn previous_char(&self, at: InputAt) -> Option<u8> {
            if at.pos() == 0 {
                None
            } else {
                Some(self.content[at.pos() - 1])
            }
        }

        fn next_char(&self, at: InputAt) -> Option<u8> {
            if at.pos() >= self.content.len() {
                None
            } else {
                Some(self.content[at.pos()])
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
        fn new(position: usize) -> Self {
            Self { position }
        }

        fn pos(&self) -> usize {
            self.position
        }

        fn is_start(&self) -> bool {
            self.position == 0
        }

        fn is_end(&self) -> bool {
            self.position >= self.position
        }
    }

    struct InstEmptyLook {
        look: prog::EmptyLook,
    }

    let input = MockInput {
        only_utf8: true,
        content: b"ab".to_vec(),
    };

    let at = InputAt::new(2);
    let empty = InstEmptyLook {
        look: prog::EmptyLook::NotWordBoundaryAscii,
    };

    let result = input.is_empty_match(at, &empty);

    assert_eq!(result, true); // Since 'b' is a word byte and there is no character after it
}

