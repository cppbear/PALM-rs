// Answer 0

#[test]
fn test_is_empty_match_word_boundary_start() {
    let input = ByteInput { text: b"hello", only_utf8: false };
    let at = InputAt { pos: 0, c: Char(0), byte: None, len: 5 };
    let empty = InstEmptyLook { goto: InstPtr(0), look: prog::EmptyLook::WordBoundary };
    input.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_word_boundary_middle() {
    let input = ByteInput { text: b"hello world", only_utf8: true };
    let at = InputAt { pos: 6, c: Char(0), byte: None, len: 11 };
    let empty = InstEmptyLook { goto: InstPtr(0), look: prog::EmptyLook::WordBoundary };
    input.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_word_boundary_end() {
    let input = ByteInput { text: b"hello ", only_utf8: false };
    let at = InputAt { pos: 6, c: Char(0), byte: None, len: 6 };
    let empty = InstEmptyLook { goto: InstPtr(0), look: prog::EmptyLook::WordBoundary };
    input.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_word_boundary_edge_case() {
    let input = ByteInput { text: b"", only_utf8: true };
    let at = InputAt { pos: 0, c: Char(u32::MAX), byte: None, len: 0 };
    let empty = InstEmptyLook { goto: InstPtr(0), look: prog::EmptyLook::WordBoundary };
    input.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_word_boundary_no_utf8() {
    let input = ByteInput { text: b"a1", only_utf8: false };
    let at = InputAt { pos: 1, c: Char('a' as u32), byte: Some(b'a'), len: 3 };
    let empty = InstEmptyLook { goto: InstPtr(0), look: prog::EmptyLook::WordBoundary };
    input.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_word_boundary_with_invalid_utf8() {
    let input = ByteInput { text: b"\xFF", only_utf8: true };
    let at = InputAt { pos: 0, c: Char(u32::MAX), byte: None, len: 1 };
    let empty = InstEmptyLook { goto: InstPtr(0), look: prog::EmptyLook::WordBoundary };
    input.is_empty_match(at, &empty);
}

