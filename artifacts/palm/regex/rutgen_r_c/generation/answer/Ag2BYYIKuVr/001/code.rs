// Answer 0

#[test]
fn test_byte_some_value() {
    let input = InputAt {
        pos: 0,
        c: Char(65), // 'A'
        byte: Some(65), // ASCII for 'A'
        len: 1,
    };
    assert_eq!(input.byte(), Some(65));
}

#[test]
fn test_byte_none_value() {
    let input = InputAt {
        pos: 1,
        c: Char(0), // Null character
        byte: None,
        len: 0,
    };
    assert_eq!(input.byte(), None);
}

#[test]
fn test_byte_zero_value() {
    let input = InputAt {
        pos: 2,
        c: Char(90), // 'Z'
        byte: Some(0), // Byte value zero
        len: 1,
    };
    assert_eq!(input.byte(), Some(0));
}

#[test]
fn test_byte_boundary() {
    let input = InputAt {
        pos: usize::MAX, // Max position
        c: Char(255), // Some valid character
        byte: Some(255), // Max byte value
        len: 1,
    };
    assert_eq!(input.byte(), Some(255));
}

