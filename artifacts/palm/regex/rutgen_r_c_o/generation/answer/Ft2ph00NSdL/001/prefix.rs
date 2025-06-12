// Answer 0

#[test]
fn test_char_valid_small() {
    let input = InputAt {
        pos: 0,
        c: Char(0),
        byte: Some(0),
        len: 1,
    };
    let _ = input.char();
}

#[test]
fn test_char_valid_large() {
    let input = InputAt {
        pos: 1,
        c: Char(1),
        byte: Some(1),
        len: 2,
    };
    let _ = input.char();
}

#[test]
fn test_char_boundaries() {
    let input_start = InputAt {
        pos: 0,
        c: Char(u32::MAX),
        byte: None,
        len: 1,
    };
    let _ = input_start.char();

    let input_end = InputAt {
        pos: u32::MAX as usize, // may depend on actual valid range of pos
        c: Char(u32::MAX - 1),
        byte: None,
        len: 1,
    };
    let _ = input_end.char();
}

