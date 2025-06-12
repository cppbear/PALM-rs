// Answer 0

#[test]
fn test_is_empty_match_not_word_boundary_ascii() {
    struct DummyInput {
        content: Vec<u8>,
        only_utf8: bool,
    }

    impl DummyInput {
        fn previous_char(&self, at: InputAt) -> Option<u8> {
            if at.pos() > 0 {
                Some(self.content[at.pos() - 1])
            } else {
                None
            }
        }

        fn next_char(&self, at: InputAt) -> Option<u8> {
            if at.pos() < self.content.len() {
                Some(self.content[at.pos()])
            } else {
                None
            }
        }

        fn len(&self) -> usize {
            self.content.len()
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
            self.pos == self.content.len()
        }
    }

    struct InstEmptyLook {
        look: EmptyLook,
    }

    #[derive(Debug)]
    enum EmptyLook {
        NotWordBoundaryAscii,
    }

    let input = DummyInput {
        content: b"abc123def".to_vec(),
        only_utf8: true,
    };
    let at = InputAt { pos: 3 }; // position between 'c' and '1'
    let empty = InstEmptyLook {
        look: EmptyLook::NotWordBoundaryAscii,
    };

    // c1 is 'c' (previous char) and c2 is '1' (next char)
    assert_eq!(input.is_empty_match(at, &empty), false); // 'c' and '1' are different word bytes

    let at = InputAt { pos: 2 }; // position between 'b' and 'c'
    assert_eq!(input.is_empty_match(at, &empty), true); // 'b' and 'c' are both word bytes
}

