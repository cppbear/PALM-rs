// Answer 0

#[test]
fn test_is_empty_match_not_word_boundary_ascii_pos_zero() {
    let text: &[u8] = b"hello";
    let input = ByteInput { text, only_utf8: false };
    let at = InputAt { pos: 0, c: Char(104), byte: Some(b'h'), len: 5 };
    let empty = InstEmptyLook { goto: InstPtr::default(), look: prog::EmptyLook::NotWordBoundaryAscii };
    input.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_not_word_boundary_ascii_middle() {
    let text: &[u8] = b"hello";
    let input = ByteInput { text, only_utf8: false };
    let at = InputAt { pos: 2, c: Char(108), byte: Some(b'l'), len: 5 };
    let empty = InstEmptyLook { goto: InstPtr::default(), look: prog::EmptyLook::NotWordBoundaryAscii };
    input.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_not_word_boundary_ascii_pos_end() {
    let text: &[u8] = b"hello";
    let input = ByteInput { text, only_utf8: false };
    let at = InputAt { pos: 5, c: Char(0), byte: None, len: 5 };
    let empty = InstEmptyLook { goto: InstPtr::default(), look: prog::EmptyLook::NotWordBoundaryAscii };
    input.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_not_word_boundary_ascii_single_byte() {
    let text: &[u8] = b"a";
    let input = ByteInput { text, only_utf8: false };
    let at = InputAt { pos: 0, c: Char(97), byte: Some(b'a'), len: 1 };
    let empty = InstEmptyLook { goto: InstPtr::default(), look: prog::EmptyLook::NotWordBoundaryAscii };
    input.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_not_word_boundary_ascii_surrounding_space() {
    let text: &[u8] = b" hello ";
    let input = ByteInput { text, only_utf8: false };
    let at = InputAt { pos: 1, c: Char(104), byte: Some(b' '), len: 7 };
    let empty = InstEmptyLook { goto: InstPtr::default(), look: prog::EmptyLook::NotWordBoundaryAscii };
    input.is_empty_match(at, &empty);
}

