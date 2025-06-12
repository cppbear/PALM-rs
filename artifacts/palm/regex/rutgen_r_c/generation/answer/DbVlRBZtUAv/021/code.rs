// Answer 0

#[test]
fn test_is_empty_match_end_line_at_end() {
    let text: &[u8] = b"Hello, World!\n";
    let input = ByteInput {
        text,
        only_utf8: true,
    };
    let at = InputAt {
        pos: text.len(),
        c: Char(u32::MAX), // Represents a "none" character
        byte: None,
        len: 0,
    };
    let empty = InstEmptyLook {
        goto: InstPtr::default(), // Placeholder, as we're focused on is_empty_match logic
        look: prog::EmptyLook::EndLine,
    };

    let result = input.is_empty_match(at, &empty);
    assert!(result);
}

#[test]
fn test_is_empty_match_end_line_not_at_end() {
    let text: &[u8] = b"Hello, World!\n";
    let input = ByteInput {
        text,
        only_utf8: true,
    };
    let at = InputAt {
        pos: text.len() - 1,
        c: Char(10), // Represents '\n'
        byte: Some(10), // ASCII for '\n'
        len: 0,
    };
    let empty = InstEmptyLook {
        goto: InstPtr::default(),
        look: prog::EmptyLook::EndLine,
    };

    let result = input.is_empty_match(at, &empty);
    assert!(result);
}

#[test]
fn test_is_empty_match_not_start_line() {
    let text: &[u8] = b"Hello, World!\n";
    let input = ByteInput {
        text,
        only_utf8: true,
    };
    let at = InputAt {
        pos: text.len() - 1,
        c: Char(0x48), // Represents 'H'
        byte: Some(b'H'),
        len: 0,
    };
    let empty = InstEmptyLook {
        goto: InstPtr::default(),
        look: prog::EmptyLook::EndLine,
    };

    let result = input.is_empty_match(at, &empty);
    assert!(!result);
}

