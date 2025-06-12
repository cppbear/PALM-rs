// Answer 0

#[test]
fn test_is_empty_match_start_line_false() {
    let text: &[u8] = b"Hello\nWorld";
    let input = ByteInput { text, only_utf8: true };

    let at = InputAt {
        pos: 1,
        c: Char(72), // 'H'
        byte: Some(b'H'),
        len: 5,
    };

    let empty = InstEmptyLook {
        goto: InstPtr { /* initialize appropriately */ },
        look: prog::EmptyLook::StartLine,
    };

    assert_eq!(input.is_empty_match(at, &empty), false);
}

#[test]
fn test_is_empty_match_start_line_true() {
    let text: &[u8] = b"";
    let input = ByteInput { text, only_utf8: true };

    let at = InputAt {
        pos: 0,
        c: Char(u32::MAX), // representing none
        byte: None,
        len: 0,
    };

    let empty = InstEmptyLook {
        goto: InstPtr { /* initialize appropriately */ },
        look: prog::EmptyLook::StartLine,
    };

    assert_eq!(input.is_empty_match(at, &empty), true);
}

