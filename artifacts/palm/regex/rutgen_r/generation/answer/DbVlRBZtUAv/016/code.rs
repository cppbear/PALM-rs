// Answer 0

#[test]
fn test_word_boundary_ascii_not_matching() {
    struct Input {
        only_utf8: bool,
        content: Vec<u8>,
    }

    impl Input {
        fn len(&self) -> usize {
            self.content.len()
        }

        fn previous_char(&self, at: InputAt) -> Option<u8> {
            if at.pos() > 0 {
                Some(self.content[at.pos() - 1])
            } else {
                None
            }
        }

        fn next_char(&self, at: InputAt) -> Option<u8> {
            if at.pos() < self.len() {
                Some(self.content[at.pos()])
            } else {
                None
            }
        }
    }

    struct InputAt {
        index: usize,
    }

    impl InputAt {
        fn pos(&self) -> usize {
            self.index
        }

        fn is_start(&self) -> bool {
            self.index == 0
        }

        fn is_end(&self, len: usize) -> bool {
            self.index == len
        }
    }

    struct InstEmptyLook {
        look: prog::EmptyLook,
    }

    let input = Input {
        only_utf8: false,
        content: b"hello world".to_vec(),
    };

    let at_start = InputAt { index: 5 }; // positioned between "hello" and "world"
    let at_end = InputAt { index: 10 }; // positioned at 'w'

    let empty_not_word_boundary_ascii = InstEmptyLook {
        look: prog::EmptyLook::WordBoundaryAscii,
    };

    assert_eq!(input.is_empty_match(at_start, &empty_not_word_boundary_ascii), true);
    assert_eq!(input.is_empty_match(at_end, &empty_not_word_boundary_ascii), false);
}

#[test]
fn test_word_boundary_ascii_matching() {
    struct Input {
        only_utf8: bool,
        content: Vec<u8>,
    }

    impl Input {
        fn len(&self) -> usize {
            self.content.len()
        }

        fn previous_char(&self, at: InputAt) -> Option<u8> {
            if at.pos() > 0 {
                Some(self.content[at.pos() - 1])
            } else {
                None
            }
        }

        fn next_char(&self, at: InputAt) -> Option<u8> {
            if at.pos() < self.len() {
                Some(self.content[at.pos()])
            } else {
                None
            }
        }
    }

    struct InputAt {
        index: usize,
    }

    impl InputAt {
        fn pos(&self) -> usize {
            self.index
        }

        fn is_start(&self) -> bool {
            self.index == 0
        }

        fn is_end(&self, len: usize) -> bool {
            self.index == len
        }
    }

    struct InstEmptyLook {
        look: prog::EmptyLook,
    }

    let input = Input {
        only_utf8: false,
        content: b"hello123".to_vec(),
    };

    let at_between = InputAt { index: 5 }; // positioned between "hello" and "1"

    let empty_word_boundary_ascii = InstEmptyLook {
        look: prog::EmptyLook::WordBoundaryAscii,
    };

    assert_eq!(input.is_empty_match(at_between, &empty_word_boundary_ascii), true);
}

