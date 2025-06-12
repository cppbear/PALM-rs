// Answer 0

#[test]
fn test_is_end_with_none_char_and_byte() {
    let input_at = InputAt {
        pos: 0,
        c: Char(u32::MAX),
        byte: None,
        len: 0,
    };
    assert!(input_at.is_end());
}

#[test]
fn test_is_end_with_char_present_and_byte_missing() {
    let input_at = InputAt {
        pos: 1,
        c: Char(97), // 'a' in UTF-32
        byte: None,
        len: 1,
    };
    assert!(!input_at.is_end());
}

#[test]
fn test_is_end_with_char_missing_and_byte_present() {
    let input_at = InputAt {
        pos: 2,
        c: Char(u32::MAX),
        byte: Some(97), // 'a'
        len: 1,
    };
    assert!(!input_at.is_end());
}

#[test]
fn test_is_end_with_none_char_and_some_byte() {
    let input_at = InputAt {
        pos: 3,
        c: Char(u32::MAX),
        byte: Some(0), // byte value (could be any valid byte)
        len: 1,
    };
    assert!(!input_at.is_end());
}

#[test]
fn test_is_end_with_char_and_byte_present() {
    let input_at = InputAt {
        pos: 4,
        c: Char(98), // 'b' in UTF-32
        byte: Some(98), // 'b'
        len: 1,
    };
    assert!(!input_at.is_end());
}

