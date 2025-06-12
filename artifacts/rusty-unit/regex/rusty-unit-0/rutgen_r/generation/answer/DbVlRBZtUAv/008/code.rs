// Answer 0

#[derive(Debug)]
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

#[derive(Debug)]
struct InstEmptyLook {
    look: EmptyLook,
}

#[derive(Debug)]
enum EmptyLook {
    StartLine,
    EndLine,
    StartText,
    EndText,
    WordBoundary,
    NotWordBoundary,
    WordBoundaryAscii,
    NotWordBoundaryAscii,
}

#[derive(Debug)]
struct Tester {
    only_utf8: bool,
    data: Vec<u8>,
}

impl Tester {
    fn new(only_utf8: bool, data: Vec<u8>) -> Self {
        Tester { only_utf8, data }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn previous_char(&self, at: InputAt) -> Option<u8> {
        if at.pos() > 0 {
            Some(self.data[at.pos() - 1])
        } else {
            None
        }
    }

    fn next_char(&self, at: InputAt) -> Option<u8> {
        if at.pos() < self.len() {
            Some(self.data[at.pos()])
        } else {
            None
        }
    }

    fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool {
        use EmptyLook::*;
        match empty.look {
            EmptyLook::NotWordBoundaryAscii => {
                let (c1, c2) = (self.previous_char(at), self.next_char(at));
                c1.is_word_byte() == c2.is_word_byte()
            }
            _ => unreachable!(),
        }
    }
}

impl u8 {
    fn is_word_byte(self) -> bool {
        self.is_ascii_alphanumeric() || self == b'_'
    }
}

#[test]
fn test_not_word_boundary_ascii_matching() {
    let tester = Tester::new(false, b"hello_world".to_vec());
    let at = InputAt { position: 5 }; // Position between 'o' and '_'
    let empty = InstEmptyLook {
        look: EmptyLook::NotWordBoundaryAscii,
    };
    
    // Both characters are valid word bytes (ASCII)
    assert!(tester.is_empty_match(at, &empty));

    let at_end = InputAt { position: 0 }; // Position at start
    // Previous char does not exist (None), next is 'h' (valid word byte)
    assert!(!tester.is_empty_match(at_end, &empty));

    let at_middle = InputAt { position: 10 }; // Position after 'd'
    // Previous is 'd' (valid), next does not exist (None)
    assert!(!tester.is_empty_match(at_middle, &empty));
}

#[test]
fn test_not_word_boundary_ascii_non_matching() {
    let tester = Tester::new(false, b"hello123".to_vec());
    let at = InputAt { position: 5 }; // Position between 'o' and '1'
    let empty = InstEmptyLook {
        look: EmptyLook::NotWordBoundaryAscii,
    };
    
    // Previous byte is valid (o), next byte is numeric (1)
    assert!(tester.is_empty_match(at, &empty));

    let at_end = InputAt { position: 6 }; // Position at '1'
    // Previous is 'l' (valid), next is '2' (valid)
    assert!(!tester.is_empty_match(at_end, &empty));
}

