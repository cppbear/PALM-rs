// Answer 0

#[test]
fn test_is_empty_match_start_line() {
    struct Input {
        text: String,
        only_utf8: bool,
    }

    impl Input {
        fn len(&self) -> usize {
            self.text.len()
        }
        
        fn previous_char(&self, at: InputAt) -> char {
            self.text.chars().nth(at.pos().saturating_sub(1)).unwrap_or('\0')
        }

        fn next_char(&self, at: InputAt) -> char {
            self.text.chars().nth(at.pos()).unwrap_or('\0')
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
    }

    struct InstEmptyLook {
        look: prog::EmptyLook,
    }

    let input = Input {
        text: String::from("\nHello"),
        only_utf8: true,
    };
    let at = InputAt { position: 0 };
    let empty = InstEmptyLook {
        look: prog::EmptyLook::StartLine,
    };
    
    assert!(input.is_empty_match(at, &empty));
}

#[test]
fn test_is_empty_match_end_line() {
    struct Input {
        text: String,
        only_utf8: bool,
    }

    impl Input {
        fn len(&self) -> usize {
            self.text.len()
        }
        
        fn previous_char(&self, at: InputAt) -> char {
            self.text.chars().nth(at.pos().saturating_sub(1)).unwrap_or('\0')
        }

        fn next_char(&self, at: InputAt) -> char {
            self.text.chars().nth(at.pos()).unwrap_or('\0')
        }
    }

    struct InputAt {
        position: usize,
    }

    impl InputAt {
        fn pos(&self) -> usize {
            self.position
        }

        fn is_end(&self) -> bool {
            self.position >= self.len()
        }
    }

    struct InstEmptyLook {
        look: prog::EmptyLook,
    }

    let input = Input {
        text: String::from("Hello\n"),
        only_utf8: true,
    };
    let at = InputAt { position: 5 };
    let empty = InstEmptyLook {
        look: prog::EmptyLook::EndLine,
    };
    
    assert!(input.is_empty_match(at, &empty));
}

#[test]
fn test_is_empty_match_start_text() {
    struct Input {
        text: String,
        only_utf8: bool,
    }

    impl Input {
        fn len(&self) -> usize {
            self.text.len()
        }
        
        fn previous_char(&self, at: InputAt) -> char {
            self.text.chars().nth(at.pos().saturating_sub(1)).unwrap_or('\0')
        }

        fn next_char(&self, at: InputAt) -> char {
            self.text.chars().nth(at.pos()).unwrap_or('\0')
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

    let input = Input {
        text: String::from("Hello"),
        only_utf8: true,
    };
    let at = InputAt { position: 0 };
    let empty = InstEmptyLook {
        look: prog::EmptyLook::StartText,
    };
    
    assert!(input.is_empty_match(at, &empty));
}

#[test]
fn test_is_empty_match_end_text() {
    struct Input {
        text: String,
        only_utf8: bool,
    }

    impl Input {
        fn len(&self) -> usize {
            self.text.len()
        }
        
        fn previous_char(&self, at: InputAt) -> char {
            self.text.chars().nth(at.pos().saturating_sub(1)).unwrap_or('\0')
        }

        fn next_char(&self, at: InputAt) -> char {
            self.text.chars().nth(at.pos()).unwrap_or('\0')
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

    let input = Input {
        text: String::from("Hello"),
        only_utf8: true,
    };
    let at = InputAt { position: 5 };
    let empty = InstEmptyLook {
        look: prog::EmptyLook::EndText,
    };
    
    assert!(input.is_empty_match(at, &empty));
}

