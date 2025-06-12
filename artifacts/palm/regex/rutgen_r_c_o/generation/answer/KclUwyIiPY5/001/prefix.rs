// Answer 0

#[test]
fn test_is_end_case_none_char_and_none_byte() {
    let input_at = InputAt {
        pos: 0,
        c: Char(u32::MAX),
        byte: None,
        len: 0,
    };
    input_at.is_end();
}

#[test]
fn test_is_end_case_none_char_and_some_byte() {
    let input_at = InputAt {
        pos: 0,
        c: Char(u32::MAX),
        byte: Some(1),
        len: 0,
    };
    input_at.is_end();
}

#[test]
fn test_is_end_case_some_char_and_none_byte() {
    let input_at = InputAt {
        pos: 0,
        c: Char(0),
        byte: None,
        len: 0,
    };
    input_at.is_end();
}

#[test]
fn test_is_end_case_some_char_and_some_byte() {
    let input_at = InputAt {
        pos: 0,
        c: Char(1),
        byte: Some(1),
        len: 0,
    };
    input_at.is_end();
}

#[test]
fn test_is_end_case_none_char_with_non_zero_len() {
    let input_at = InputAt {
        pos: 0,
        c: Char(u32::MAX),
        byte: None,
        len: 1,
    };
    input_at.is_end();
}

#[test]
fn test_is_end_case_none_char_with_max_pos() {
    let input_at = InputAt {
        pos: usize::MAX,
        c: Char(u32::MAX),
        byte: None,
        len: 0,
    };
    input_at.is_end();
}

