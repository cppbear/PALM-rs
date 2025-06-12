// Answer 0

#[test]
fn test_is_empty_match_word_boundary_ascii() {
    struct Input {
        only_utf8: bool,
        length: usize,
    }

    impl Input {
        fn len(&self) -> usize {
            self.length
        }

        fn previous_char(&self, _at: InputAt) -> Option<char> {
            None // c1 is None
        }

        fn next_char(&self, _at: InputAt) -> Option<char> {
            None // c2 is None
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
            self.position == self.length
        }
    }

    struct InstEmptyLook {
        look: prog::EmptyLook,
    }

    let input = Input {
        only_utf8: true,
        length: 5,
    };

    let at = InputAt {
        position: 0, // at.is_start() == true
    };

    let empty = InstEmptyLook {
        look: prog::EmptyLook::WordBoundaryAscii, // matches constraint
    };

    assert_eq!(input.is_empty_match(at, &empty), false); // expected return value is false
}

