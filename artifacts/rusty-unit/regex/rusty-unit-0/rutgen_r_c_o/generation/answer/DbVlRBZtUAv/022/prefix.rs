// Answer 0

#[test]
fn test_is_empty_match_end_line() {
    let text: &[u8] = b"Hello\nWorld";
    let input = ByteInput { text, only_utf8: true };
    let at = InputAt { pos: 5, c: Char(0x48), byte: Some(b'H'), len: 5 }; // Not at the end position
    let empty = InstEmptyLook { goto: InstPtr(0), look: EmptyLook::EndLine };

    input.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_end_line_not_at_end() {
    let text: &[u8] = b"Sample Text\nAnother Line";
    let input = ByteInput { text, only_utf8: true };
    let at = InputAt { pos: 11, c: Char(0x74), byte: Some(b't'), len: 14 }; // Not at the end position
    let empty = InstEmptyLook { goto: InstPtr(0), look: EmptyLook::EndLine };

    input.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_end_line_with_unicode() {
    let text: &[u8] = "💖\nNext Line".as_bytes();
    let input = ByteInput { text, only_utf8: true };
    let at = InputAt { pos: 4, c: Char(0x1F496), byte: Some(b'\n'), len: 10 }; // Not at the end position
    let empty = InstEmptyLook { goto: InstPtr(0), look: EmptyLook::EndLine };

    input.is_empty_match(at, &empty);
}

#[test]
fn test_is_empty_match_end_line_multiple_lines() {
    let text: &[u8] = b"Line1\nLine2\n";
    let input = ByteInput { text, only_utf8: true };
    let at = InputAt { pos: 7, c: Char(0x4C), byte: Some(b'e'), len: 10 }; // Not at the end position
    let empty = InstEmptyLook { goto: InstPtr(0), look: EmptyLook::EndLine };

    input.is_empty_match(at, &empty);
}

