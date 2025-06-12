// Answer 0

#[test]
fn test_is_empty_match_start_line() {
    struct InputMock {
        data: String,
    }

    impl InputMock {
        fn previous_char(&self, at: InputAt) -> char {
            if at.pos() > 0 {
                self.data.chars().nth(at.pos() - 1).unwrap_or('\0')
            } else {
                '\0'
            }
        }

        fn next_char(&self, at: InputAt) -> char {
            if at.pos() < self.data.len() {
                self.data.chars().nth(at.pos()).unwrap_or('\0')
            } else {
                '\0'
            }
        }

        fn len(&self) -> usize {
            self.data.len()
        }
    }

    let input = InputMock { data: "Hello\nWorld".to_string() };
    let at = InputAt::new(0);
    let empty = InstEmptyLook { look: prog::EmptyLook::StartLine };
    
    assert!(input.is_empty_match(at, &empty));
}

#[test]
fn test_is_empty_match_end_line() {
    struct InputMock {
        data: String,
    }

    impl InputMock {
        fn previous_char(&self, at: InputAt) -> char {
            if at.pos() > 0 {
                self.data.chars().nth(at.pos() - 1).unwrap_or('\0')
            } else {
                '\0'
            }
        }

        fn next_char(&self, at: InputAt) -> char {
            if at.pos() < self.data.len() {
                self.data.chars().nth(at.pos()).unwrap_or('\0')
            } else {
                '\0'
            }
        }

        fn len(&self) -> usize {
            self.data.len()
        }
    }

    let input = InputMock { data: "Hello\nWorld".to_string() };
    let at = InputAt::new(input.len());
    let empty = InstEmptyLook { look: prog::EmptyLook::EndLine };
    
    assert!(input.is_empty_match(at, &empty));
}

#[test]
fn test_is_empty_match_start_text() {
    struct InputMock {
        data: String,
    }

    impl InputMock {
        fn previous_char(&self, at: InputAt) -> char {
            if at.pos() > 0 {
                self.data.chars().nth(at.pos() - 1).unwrap_or('\0')
            } else {
                '\0'
            }
        }

        fn next_char(&self, at: InputAt) -> char {
            if at.pos() < self.data.len() {
                self.data.chars().nth(at.pos()).unwrap_or('\0')
            } else {
                '\0'
            }
        }

        fn len(&self) -> usize {
            self.data.len()
        }
    }

    let input = InputMock { data: "Hello".to_string() };
    let at = InputAt::new(0);
    let empty = InstEmptyLook { look: prog::EmptyLook::StartText };
    
    assert!(input.is_empty_match(at, &empty));
}

#[test]
fn test_is_empty_match_end_text() {
    struct InputMock {
        data: String,
    }

    impl InputMock {
        fn previous_char(&self, at: InputAt) -> char {
            if at.pos() > 0 {
                self.data.chars().nth(at.pos() - 1).unwrap_or('\0')
            } else {
                '\0'
            }
        }

        fn next_char(&self, at: InputAt) -> char {
            if at.pos() < self.data.len() {
                self.data.chars().nth(at.pos()).unwrap_or('\0')
            } else {
                '\0'
            }
        }

        fn len(&self) -> usize {
            self.data.len()
        }
    }

    let input = InputMock { data: "Hello".to_string() };
    let at = InputAt::new(input.len());
    let empty = InstEmptyLook { look: prog::EmptyLook::EndText };
    
    assert!(input.is_empty_match(at, &empty));
}

#[test]
fn test_is_empty_match_word_boundary() {
    struct InputMock {
        data: String,
    }

    impl InputMock {
        fn previous_char(&self, at: InputAt) -> char {
            if at.pos() > 0 {
                self.data.chars().nth(at.pos() - 1).unwrap_or('\0')
            } else {
                '\0'
            }
        }

        fn next_char(&self, at: InputAt) -> char {
            if at.pos() < self.data.len() {
                self.data.chars().nth(at.pos()).unwrap_or('\0')
            } else {
                '\0'
            }
        }

        fn len(&self) -> usize {
            self.data.len()
        }
    }

    let input = InputMock { data: "Hello World".to_string() };
    let at = InputAt::new(5); // between "Hello" and "World"
    let empty = InstEmptyLook { look: prog::EmptyLook::WordBoundary };
    
    assert!(input.is_empty_match(at, &empty));
}

#[test]
fn test_is_empty_match_not_word_boundary() {
    struct InputMock {
        data: String,
    }

    impl InputMock {
        fn previous_char(&self, at: InputAt) -> char {
            if at.pos() > 0 {
                self.data.chars().nth(at.pos() - 1).unwrap_or('\0')
            } else {
                '\0'
            }
        }

        fn next_char(&self, at: InputAt) -> char {
            if at.pos() < self.data.len() {
                self.data.chars().nth(at.pos()).unwrap_or('\0')
            } else {
                '\0'
            }
        }

        fn len(&self) -> usize {
            self.data.len()
        }
    }

    let input = InputMock { data: "Hello".to_string() };
    let at = InputAt::new(2); // between "Hel" and "lo"
    let empty = InstEmptyLook { look: prog::EmptyLook::NotWordBoundary };
    
    assert!(input.is_empty_match(at, &empty));
}

