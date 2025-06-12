// Answer 0

#[test]
fn test_byte_some() {
    let input_at = InputAt {
        pos: 0,
        c: Char(97), // 'a'
        byte: Some(97), // ASCII value of 'a'
        len: 1,
    };
    assert_eq!(input_at.byte(), Some(97));
}

#[test]
fn test_byte_none() {
    let input_at = InputAt {
        pos: 0,
        c: Char(97), // 'a'
        byte: None,
        len: 1,
    };
    assert_eq!(input_at.byte(), None);
}

