// Answer 0

#[test]
fn test_is_empty_match_start_text() {
    struct TestByteInput {
        text: &'static [u8],
        only_utf8: bool,
    }

    impl Input for TestByteInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt {
                pos: i,
                c: Char(u32::MAX), // use Char::is_none to indicate no character
                byte: None,
                len: self.len(),
            }
        }

        fn next_char(&self, at: InputAt) -> Char {
            if at.pos < self.len() {
                Char(self.text[at.pos] as u32) // simplistic next char
            } else {
                Char(u32::MAX) // no next char
            }
        }

        fn previous_char(&self, at: InputAt) -> Char {
            if at.pos > 0 {
                Char(self.text[at.pos - 1] as u32) // simplistic previous char
            } else {
                Char(u32::MAX) // no previous char
            }
        }

        fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool {
            if empty.look == prog::EmptyLook::StartText {
                return at.pos() == 0; // specific behavior for StartText
            }
            false // other cases not needed
        }

        fn prefix_at(&self, _: &LiteralSearcher, _: InputAt) -> Option<InputAt> {
            None // not implemented
        }

        fn len(&self) -> usize {
            self.text.len()
        }

        fn as_bytes(&self) -> &[u8] {
            self.text
        }
    }

    let input = TestByteInput {
        text: b"Hello, world!",
        only_utf8: true,
    };
    
    let at = input.at(0);
    let empty = InstEmptyLook {
        goto: 0, // not used in this test
        look: prog::EmptyLook::StartText,
    };

    assert!(input.is_empty_match(at, &empty));
}

#[test]
fn test_is_empty_match_not_start_text() {
    struct TestByteInput {
        text: &'static [u8],
        only_utf8: bool,
    }

    impl Input for TestByteInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt {
                pos: i,
                c: Char(u32::MAX), // use Char::is_none to indicate no character
                byte: None,
                len: self.len(),
            }
        }

        fn next_char(&self, at: InputAt) -> Char {
            if at.pos < self.len() {
                Char(self.text[at.pos] as u32) // simplistic next char
            } else {
                Char(u32::MAX) // no next char
            }
        }

        fn previous_char(&self, at: InputAt) -> Char {
            if at.pos > 0 {
                Char(self.text[at.pos - 1] as u32) // simplistic previous char
            } else {
                Char(u32::MAX) // no previous char
            }
        }

        fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool {
            if empty.look == prog::EmptyLook::StartText {
                return at.pos() == 0; // specific behavior for StartText
            }
            false // other cases not needed
        }

        fn prefix_at(&self, _: &LiteralSearcher, _: InputAt) -> Option<InputAt> {
            None // not implemented
        }

        fn len(&self) -> usize {
            self.text.len()
        }

        fn as_bytes(&self) -> &[u8] {
            self.text
        }
    }

    let input = TestByteInput {
        text: b"Hello, world!",
        only_utf8: true,
    };
    
    let at = input.at(1); // Not at the start
    let empty = InstEmptyLook {
        goto: 0, // not used in this test
        look: prog::EmptyLook::StartText,
    };

    assert!(!input.is_empty_match(at, &empty));
}

