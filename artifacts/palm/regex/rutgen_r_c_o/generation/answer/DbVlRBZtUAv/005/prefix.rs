// Answer 0

#[test]
fn test_is_empty_match_not_word_boundary_ascii_case_1() {
    let text: &[u8] = &[b'a']; // c1 is 'a' which is in range [0, 127]
    let byte_input = ByteInput { text, only_utf8: true };

    let at = InputAt { pos: 1, c: Char(0), byte: None, len: 1 }; // at.is_end() true

    let empty = InstEmptyLook { goto: InstPtr, look: prog::EmptyLook::NotWordBoundaryAscii }; // empty.look == NotWordBoundaryAscii
    let result = byte_input.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_not_word_boundary_ascii_case_2() {
    let text: &[u8] = &[b'a', b'\x80']; // c1 is 'a' and c2 is a byte in range [128, 255]
    let byte_input = ByteInput { text, only_utf8: true };

    let at = InputAt { pos: 2, c: Char(0), byte: None, len: 2 }; // at.is_end() true

    let empty = InstEmptyLook { goto: InstPtr, look: prog::EmptyLook::NotWordBoundaryAscii }; // empty.look == NotWordBoundaryAscii
    let result = byte_input.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_not_word_boundary_ascii_case_3() {
    let text: &[u8] = &[b'a', b'z', b'\x81']; // c1 is 'z' and c2 is a byte in range [128, 255]
    let byte_input = ByteInput { text, only_utf8: true };

    let at = InputAt { pos: 3, c: Char(0), byte: None, len: 3 }; // at.is_end() true

    let empty = InstEmptyLook { goto: InstPtr, look: prog::EmptyLook::NotWordBoundaryAscii }; // empty.look == NotWordBoundaryAscii
    let result = byte_input.is_empty_match(at, &empty);
}

