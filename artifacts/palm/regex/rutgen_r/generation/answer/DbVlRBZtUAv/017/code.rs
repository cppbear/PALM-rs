// Answer 0

#[derive(Debug)]
struct InputAt {
    pos: usize,
}

impl InputAt {
    fn pos(&self) -> usize {
        self.pos
    }

    fn is_start(&self) -> bool {
        self.pos == 0
    }

    fn is_end(&self, len: usize) -> bool {
        self.pos == len
    }
}

#[derive(Debug)]
struct InstEmptyLook {
    look: EmptyLook,
}

#[derive(Debug)]
enum EmptyLook {
    NotWordBoundary,
}

#[derive(Debug)]
struct TestStruct {
    only_utf8: bool,
    data: Vec<u8>,
}

impl TestStruct {
    fn previous_char(&self, at: &InputAt) -> Option<u8> {
        if at.pos() == 0 {
            None
        } else {
            Some(self.data[at.pos() - 1])
        }
    }

    fn next_char(&self, at: &InputAt) -> Option<u8> {
        if at.pos() >= self.data.len() {
            None
        } else {
            Some(self.data[at.pos()])
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

trait Input {
    fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool;
}

impl Input for TestStruct {
    fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool {
        use prog::EmptyLook::*;
        match empty.look {
            NotWordBoundary => {
                let (c1, c2) = (self.previous_char(&at), self.next_char(&at));
                c1.is_word_char() == c2.is_word_char()
            }
            _ => false,
        }
    }
}

#[test]
fn test_not_word_boundary_same_chars() {
    let test_struct = TestStruct { only_utf8: true, data: b"hello".to_vec() };
    let at = InputAt { pos: 2 }; // 'l'
    let empty = InstEmptyLook { look: EmptyLook::NotWordBoundary };

    assert!(test_struct.is_empty_match(at, &empty));
}

#[test]
fn test_not_word_boundary_different_chars() {
    let test_struct = TestStruct { only_utf8: true, data: b"hello1".to_vec() };
    let at = InputAt { pos: 5 }; // '1'
    let empty = InstEmptyLook { look: EmptyLook::NotWordBoundary };

    assert!(!test_struct.is_empty_match(at, &empty));
}

#[test]
fn test_not_word_boundary_boundary_case_start() {
    let test_struct = TestStruct { only_utf8: true, data: b"".to_vec() };
    let at = InputAt { pos: 0 };
    let empty = InstEmptyLook { look: EmptyLook::NotWordBoundary };

    assert!(test_struct.is_empty_match(at, &empty)); // Edge case, no character on both sides
}

#[test]
fn test_not_word_boundary_boundary_case_end() {
    let test_struct = TestStruct { only_utf8: true, data: b"abc".to_vec() };
    let at = InputAt { pos: 3 }; // End of the string
    let empty = InstEmptyLook { look: EmptyLook::NotWordBoundary };

    assert!(test_struct.is_empty_match(at, &empty)); // Edge case, no character on right side
}

