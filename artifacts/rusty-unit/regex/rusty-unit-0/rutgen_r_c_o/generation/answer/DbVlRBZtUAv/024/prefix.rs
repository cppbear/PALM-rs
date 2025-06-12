// Answer 0

#[test]
fn test_is_empty_match_start_line_at_start() {
    let text: &[u8] = b"";
    let byte_input = ByteInput { text, only_utf8: true };
    let at = InputAt { pos: 0, c: Char(u32::MAX), byte: None, len: 0 };
    let empty = InstEmptyLook { goto: InstPtr {}, look: prog::EmptyLook::StartLine };

    byte_input.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_start_line_non_empty() {
    let text: &[u8] = b"\n";
    let byte_input = ByteInput { text, only_utf8: true };
    let at = InputAt { pos: 0, c: Char(10), byte: None, len: 1 };
    let empty = InstEmptyLook { goto: InstPtr {}, look: prog::EmptyLook::StartLine };

    byte_input.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_start_line_edge_case() {
    let text: &[u8] = b"ABC";
    let byte_input = ByteInput { text, only_utf8: true };
    let at = InputAt { pos: 0, c: Char(u32::MAX), byte: None, len: 3 };
    let empty = InstEmptyLook { goto: InstPtr {}, look: prog::EmptyLook::StartLine };

    byte_input.is_empty_match(at, &empty);
}

