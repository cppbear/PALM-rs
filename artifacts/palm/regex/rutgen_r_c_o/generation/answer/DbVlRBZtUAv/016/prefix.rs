// Answer 0

#[test]
fn test_is_empty_match_word_boundary_ascii_true() {
    let text: &[u8] = b"word, test."; // ASCII input
    let input = ByteInput { text, only_utf8: false };
    
    let at = InputAt { pos: 4, c: Char(100), byte: Some(b'w'), len: 4 }; // Position on 'w' in "word"
    let empty = InstEmptyLook { goto: InstPtr, look: prog::EmptyLook::WordBoundaryAscii };
    
    input.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_word_boundary_ascii_false() {
    let text: &[u8] = b"word"; // ASCII input
    let input = ByteInput { text, only_utf8: false };
    
    let at = InputAt { pos: 3, c: Char(100), byte: Some(b'd'), len: 4 }; // Position on 'd' in "word"
    let empty = InstEmptyLook { goto: InstPtr, look: prog::EmptyLook::WordBoundaryAscii };
    
    input.is_empty_match(at, &empty);
}

