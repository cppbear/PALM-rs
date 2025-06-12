// Answer 0

#[test]
fn test_is_empty_match_not_word_boundary_ascii() {
    let input_bytes: &[u8] = b"";
    let input = ByteInput { text: input_bytes, only_utf8: true };
    let at = InputAt { pos: 0, c: Char(u32::MAX), byte: None, len: 1 };
    let empty = InstEmptyLook { goto: InstPtr(0), look: EmptyLook::NotWordBoundaryAscii };

    let _ = input.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_not_word_boundary_ascii_non_empty() {
    let input_bytes: &[u8] = b"A";
    let input = ByteInput { text: input_bytes, only_utf8: true };
    let at = InputAt { pos: 0, c: Char(u32::MAX), byte: None, len: 1 };
    let empty = InstEmptyLook { goto: InstPtr(0), look: EmptyLook::NotWordBoundaryAscii };

    let _ = input.is_empty_match(at, &empty);
}

