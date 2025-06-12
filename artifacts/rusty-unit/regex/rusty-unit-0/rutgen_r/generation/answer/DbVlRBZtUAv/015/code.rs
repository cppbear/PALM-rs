// Answer 0

#[test]
fn test_is_empty_match_word_boundary_ascii() {
    struct DummyInput {
        data: Vec<u8>,
        only_utf8: bool,
    }

    impl DummyInput {
        fn len(&self) -> usize {
            self.data.len()
        }

        fn next_char(&self, at: InputAt) -> Option<char> {
            if at.pos() < self.len() {
                Some(self.data[at.pos()] as char)
            } else {
                None
            }
        }

        fn previous_char(&self, at: InputAt) -> Option<char> {
            if at.pos() > 0 {
                Some(self.data[at.pos() - 1] as char)
            } else {
                None
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

        fn is_start(&self) -> bool {
            self.pos() == 0
        }

        fn is_end(&self, input_length: usize) -> bool {
            self.pos() == input_length
        }
    }

    struct InstEmptyLook {
        look: prog::EmptyLook,
    }

    let input_data = DummyInput { 
        data: vec![b'a', b' '], // Word boundary between 'a' and space
        only_utf8: true,
    };
    
    let at = InputAt { position: 1 }; // Position between 'a' and ' '
    let empty_look = InstEmptyLook { look: prog::EmptyLook::WordBoundaryAscii };

    let result = input_data.is_empty_match(at, &empty_look);

    assert!(result); // c1.is_word_byte() (for 'a') != c2.is_word_byte() (for ' ')
}

#[test]
fn test_is_empty_match_word_boundary_ascii_no_word_boundary() {
    struct DummyInput {
        data: Vec<u8>,
        only_utf8: bool,
    }

    impl DummyInput {
        fn len(&self) -> usize {
            self.data.len()
        }

        fn next_char(&self, at: InputAt) -> Option<char> {
            if at.pos() < self.len() {
                Some(self.data[at.pos()] as char)
            } else {
                None
            }
        }

        fn previous_char(&self, at: InputAt) -> Option<char> {
            if at.pos() > 0 {
                Some(self.data[at.pos() - 1] as char)
            } else {
                None
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

        fn is_start(&self) -> bool {
            self.pos() == 0
        }

        fn is_end(&self, input_length: usize) -> bool {
            self.pos() == input_length
        }
    }

    struct InstEmptyLook {
        look: prog::EmptyLook,
    }

    let input_data = DummyInput { 
        data: vec![b'a', b'a'], // No word boundary between two 'a's
        only_utf8: true,
    };
    
    let at = InputAt { position: 1 }; // Position between 'a' and 'a'
    let empty_look = InstEmptyLook { look: prog::EmptyLook::WordBoundaryAscii };

    let result = input_data.is_empty_match(at, &empty_look);

    assert!(!result); // c1.is_word_byte() (for 'a') == c2.is_word_byte() (for 'a')
}

