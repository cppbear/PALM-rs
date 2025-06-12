// Answer 0

#[test]
fn test_is_empty_match_end_line_at_pos_equal_len() {
    let text: &[u8] = b"Hello, World!\n";
    let empty_look = InstEmptyLook {
        goto: InstPtr::new(),
        look: EmptyLook::EndLine,
    };
    let input = ByteInput { text, only_utf8: true };
    let at = InputAt { pos: text.len(), c: Char(10), byte: Some(b'\n'), len: 1 };

    input.is_empty_match(at, &empty_look);
}

#[test]
fn test_is_empty_match_end_line_at_pos_equal_len_with_previous_newline() {
    let text: &[u8] = b"Hello, World!\n";
    let empty_look = InstEmptyLook {
        goto: InstPtr::new(),
        look: EmptyLook::EndLine,
    };
    let input = ByteInput { text, only_utf8: true };
    let at = InputAt { pos: text.len(), c: Char(u32::MAX), byte: None, len: 0 };

    input.is_empty_match(at, &empty_look);
}

#[test]
fn test_is_empty_match_end_line_at_pos_equal_len_with_non_newline() {
    let text: &[u8] = b"Hello, World!";
    let empty_look = InstEmptyLook {
        goto: InstPtr::new(),
        look: EmptyLook::EndLine,
    };
    let input = ByteInput { text, only_utf8: true };
    let at = InputAt { pos: text.len(), c: Char(u32::MAX), byte: None, len: 0 };

    input.is_empty_match(at, &empty_look);
}

