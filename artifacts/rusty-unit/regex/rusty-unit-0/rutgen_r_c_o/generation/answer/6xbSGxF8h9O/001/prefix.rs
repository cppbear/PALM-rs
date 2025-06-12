// Answer 0

#[test]
fn test_is_empty_with_zero_length() {
    let input_at = InputAt {
        pos: 0,
        c: Char(0),
        byte: None,
        len: 0,
    };
    input_at.is_empty();
}

#[test]
fn test_is_empty_with_non_zero_length() {
    let input_at = InputAt {
        pos: 1,
        c: Char(1),
        byte: Some(1),
        len: 1,
    };
    input_at.is_empty();
}

