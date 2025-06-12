// Answer 0

#[test]
fn test_is_empty_match_word_boundary_ascii_valid() {
    let input_bytes: &[u8] = b"Hello, World!";
    let input = ByteInput { text: input_bytes, only_utf8: true };
    
    let at = InputAt { pos: 5, c: Char(0x6F), byte: Some(b'o'), len: 1 }; // 'o' in "Hello"
    let empty = InstEmptyLook { goto: InstPtr::default(), look: prog::EmptyLook::WordBoundaryAscii };
    
    input.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_word_boundary_ascii_valid_end() {
    let input_bytes: &[u8] = b"Hello, World!";
    let input = ByteInput { text: input_bytes, only_utf8: true };
    
    let at = InputAt { pos: 6, c: Char(0x2C), byte: Some(b','), len: 1 }; // ',' in "Hello,"
    let empty = InstEmptyLook { goto: InstPtr::default(), look: prog::EmptyLook::WordBoundaryAscii };
    
    input.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_word_boundary_ascii_valid_start() {
    let input_bytes: &[u8] = b"Hello, World!";
    let input = ByteInput { text: input_bytes, only_utf8: true };
    
    let at = InputAt { pos: 1, c: Char(0x65), byte: Some(b'e'), len: 1 }; // 'e' in "Hello"
    let empty = InstEmptyLook { goto: InstPtr::default(), look: prog::EmptyLook::WordBoundaryAscii };
    
    input.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_word_boundary_ascii_non_word() {
    let input_bytes: &[u8] = b"Hello World!";
    let input = ByteInput { text: input_bytes, only_utf8: true };
    
    let at = InputAt { pos: 5, c: Char(0x20), byte: Some(b' '), len: 1 }; // space between "Hello" and "World"
    let empty = InstEmptyLook { goto: InstPtr::default(), look: prog::EmptyLook::WordBoundaryAscii };
    
    input.is_empty_match(at, &empty);
}

