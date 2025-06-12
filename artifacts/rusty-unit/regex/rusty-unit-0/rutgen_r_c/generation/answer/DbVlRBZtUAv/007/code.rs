// Answer 0

#[test]
fn test_is_empty_match_not_word_boundary_ascii() {
    struct MockLiteralSearcher;

    struct MockInput {
        text: Vec<u8>,
        only_utf8: bool,
    }

    impl Input for MockInput {
        fn at(&self, _i: usize) -> InputAt {
            InputAt { pos: 1, c: Char(97), byte: Some(97), len: 1 } // 'a'
        }

        fn next_char(&self, _at: InputAt) -> Char {
            Char(98) // 'b'
        }

        fn previous_char(&self, _at: InputAt) -> Char {
            Char(97) // 'a'
        }

        fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool {
            use prog::EmptyLook::*;
            match empty.look {
                NotWordBoundaryAscii => {
                    let (c1, c2) = (self.previous_char(at), self.next_char(at));
                    if self.only_utf8 {
                        if c1.is_none() && !at.is_start() {
                            return false;
                        }
                        if c2.is_none() && !at.is_end() {
                            return false;
                        }
                    }
                    c1.is_word_byte() == c2.is_word_byte()
                }
                _ => false,
            }
        }

        fn len(&self) -> usize {
            self.text.len()
        }

        fn as_bytes(&self) -> &[u8] {
            &self.text
        }
    }

    let empty = InstEmptyLook {
        goto: InstPtr(0),
        look: prog::EmptyLook::NotWordBoundaryAscii,
    };

    let input = MockInput {
        text: vec![b'a', b'b'],
        only_utf8: true,
    };

    let at = InputAt {
        pos: 1,
        c: Char(97),
        byte: Some(97),
        len: 1,
    };

    assert!(input.is_empty_match(at, &empty));
}

#[test]
fn test_is_empty_match_not_word_boundary_ascii_invalid_utf8() {
    struct MockLiteralSearcher;
    
    struct MockInput {
        text: Vec<u8>,
        only_utf8: bool,
    }

    impl Input for MockInput {
        fn at(&self, _i: usize) -> InputAt {
            InputAt { pos: 1, c: Char(0xFFFD), byte: None, len: 1 } // Replacement character
        }

        fn next_char(&self, _at: InputAt) -> Char {
            Char(98) // 'b'
        }

        fn previous_char(&self, _at: InputAt) -> Char {
            Char(0xFFFD) // Replacement character
        }

        fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool {
            use prog::EmptyLook::*;
            match empty.look {
                NotWordBoundaryAscii => {
                    let (c1, c2) = (self.previous_char(at), self.next_char(at));
                    if self.only_utf8 {
                        if c1.is_none() && !at.is_start() {
                            return false;
                        }
                        if c2.is_none() && !at.is_end() {
                            return false;
                        }
                    }
                    c1.is_word_byte() == c2.is_word_byte()
                }
                _ => false,
            }
        }

        fn len(&self) -> usize {
            self.text.len()
        }

        fn as_bytes(&self) -> &[u8] {
            &self.text
        }
    }

    let empty = InstEmptyLook {
        goto: InstPtr(0),
        look: prog::EmptyLook::NotWordBoundaryAscii,
    };

    let input = MockInput {
        text: vec![0xFF], // Invalid UTF-8 byte
        only_utf8: true,
    };

    let at = InputAt {
        pos: 1,
        c: Char(0xFFFD),
        byte: None,
        len: 1,
    };

    assert_eq!(input.is_empty_match(at, &empty), false);
}

