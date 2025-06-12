// Answer 0

#[test]
fn test_is_empty_match_start_line() {
    let data = b"\nHello";
    let input = ByteInput { text: data, only_utf8: true };
    let at = InputAt { pos: 0, c: Char(u32::MAX), byte: None, len: 0 };
    let empty = InstEmptyLook { goto: InstPtr::default(), look: EmptyLook::StartLine };

    assert!(input.is_empty_match(at, &empty));
}

#[test]
fn test_is_empty_match_end_line() {
    let data = b"Hello\n";
    let input = ByteInput { text: data, only_utf8: true };
    let at = InputAt { pos: 6, c: Char(u32::MAX), byte: None, len: 0 };
    let empty = InstEmptyLook { goto: InstPtr::default(), look: EmptyLook::EndLine };

    assert!(input.is_empty_match(at, &empty));
}

#[test]
fn test_is_empty_match_start_text() {
    let data = b"Hello";
    let input = ByteInput { text: data, only_utf8: true };
    let at = InputAt { pos: 0, c: Char(u32::MAX), byte: None, len: 0 };
    let empty = InstEmptyLook { goto: InstPtr::default(), look: EmptyLook::StartText };

    assert!(input.is_empty_match(at, &empty));
}

#[test]
fn test_is_empty_match_end_text() {
    let data = b"Hello";
    let input = ByteInput { text: data, only_utf8: true };
    let at = InputAt { pos: 5, c: Char(u32::MAX), byte: None, len: 0 };
    let empty = InstEmptyLook { goto: InstPtr::default(), look: EmptyLook::EndText };

    assert!(input.is_empty_match(at, &empty));
}

#[test]
fn test_is_empty_match_word_boundary() {
    let data = b"Hello World";
    let input = ByteInput { text: data, only_utf8: true };
    let at_before = InputAt { pos: 5, c: Char(u32::from(' ' as u32)), byte: None, len: 0 };
    let empty_boundary = InstEmptyLook { goto: InstPtr::default(), look: EmptyLook::WordBoundary };

    assert!(input.is_empty_match(at_before, &empty_boundary));
}

#[test]
fn test_is_empty_match_not_word_boundary() {
    let data = b"Hello World";
    let input = ByteInput { text: data, only_utf8: true };
    let at_inside = InputAt { pos: 6, c: Char(u32::from('W' as u32)), byte: None, len: 0 };
    let empty_not_boundary = InstEmptyLook { goto: InstPtr::default(), look: EmptyLook::NotWordBoundary };

    assert!(input.is_empty_match(at_inside, &empty_not_boundary));
}

