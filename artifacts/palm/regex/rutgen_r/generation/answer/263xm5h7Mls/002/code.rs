// Answer 0

fn test_is_empty_match_word_boundary_ascii() {
    struct DummyInput {
        text: String,
    }

    impl DummyInput {
        fn len(&self) -> usize {
            self.text.len()
        }

        fn previous_char(&self, at: InputAt) -> char {
            if at.pos() == 0 {
                '\0'
            } else {
                self.text.chars().nth(at.pos() - 1).unwrap()
            }
        }

        fn next_char(&self, at: InputAt) -> char {
            if at.pos() >= self.len() {
                '\0'
            } else {
                self.text.chars().nth(at.pos()).unwrap()
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

    mod prog {
        pub enum EmptyLook {
            WordBoundaryAscii,
        }
    }

    let input = DummyInput {
        text: "a b".to_string(),
    };

    let at = InputAt { position: 1 };
    let empty = InstEmptyLook {
        look: prog::EmptyLook::WordBoundaryAscii,
    };
    
    assert!(input.is_empty_match(at, &empty) == true);

    // Additional cases
    let input2 = DummyInput {
        text: "abc".to_string(),
    };

    let at2 = InputAt { position: 0 };
    assert!(input2.is_empty_match(at2, &empty) == false);

    let at3 = InputAt { position: 2 };
    assert!(input2.is_empty_match(at3, &empty) == true);
    
    let input3 = DummyInput {
        text: "123".to_string(),
    };

    let at4 = InputAt { position: 1 };
    assert!(input3.is_empty_match(at4, &empty) == false);
}

