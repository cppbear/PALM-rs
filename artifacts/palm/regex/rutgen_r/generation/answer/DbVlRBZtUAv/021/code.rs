// Answer 0

#[derive(Clone, Copy)]
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
    
    fn is_end(&self, length: usize) -> bool {
        self.position == length
    }
}

struct InstEmptyLook {
    look: prog::EmptyLook,
}

struct TestInput {
    only_utf8: bool,
    data: String,
}

impl TestInput {
    fn len(&self) -> usize {
        self.data.len()
    }
    
    fn previous_char(&self, at: InputAt) -> Option<char> {
        if at.pos() == 0 {
            None
        } else {
            self.data.chars().nth(at.pos() - 1)
        }
    }
    
    fn next_char(&self, at: InputAt) -> Option<char> {
        if at.pos() >= self.len() {
            None
        } else {
            self.data.chars().nth(at.pos())
        }
    }
    
    fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool {
        use prog::EmptyLook::*;
        match empty.look {
            EndLine => {
                let c = self.next_char(at);
                at.pos() == self.len() || c == Some('\n')
            }
            _ => panic!("Test only for EndLine case"),
        }
    }
}

#[test]
fn test_is_empty_match_end_line() {
    let input_data = TestInput {
        only_utf8: true,
        data: String::from("Hello\nWorld\n"),
    };
    let at = InputAt { position: input_data.len() };
    let empty_look = InstEmptyLook { look: prog::EmptyLook::EndLine };

    let result = input_data.is_empty_match(at, &empty_look);
    assert!(result);
}

#[test]
fn test_is_empty_match_end_line_with_no_newline() {
    let input_data = TestInput {
        only_utf8: true,
        data: String::from("Hello World"),
    };
    let at = InputAt { position: input_data.len() };
    let empty_look = InstEmptyLook { look: prog::EmptyLook::EndLine };

    let result = input_data.is_empty_match(at, &empty_look);
    assert!(result);
}

#[test]
#[should_panic]
fn test_is_empty_match_invalid_case() {
    let input_data = TestInput {
        only_utf8: true,
        data: String::from("Panic Test"),
    };
    let at = InputAt { position: input_data.len() };
    let empty_look = InstEmptyLook { look: prog::EmptyLook::StartLine };

    // This should panic since we are testing an invalid case meant for EndLine only
    let _ = input_data.is_empty_match(at, &empty_look);
}

