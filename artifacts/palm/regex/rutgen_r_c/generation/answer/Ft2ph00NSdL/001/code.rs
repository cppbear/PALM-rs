// Answer 0

#[test]
fn test_char_valid_position() {
    let input_at = InputAt {
        pos: 0,
        c: Char(97), // Character 'a'
        byte: Some(97),
        len: 1,
    };
    assert_eq!(input_at.char(), Char(97));
}

#[test]
fn test_char_valid_position_non_ascii() {
    let input_at = InputAt {
        pos: 1,
        c: Char(0x1F600), // Character '😀'
        byte: Some(128512),
        len: 4,
    };
    assert_eq!(input_at.char(), Char(0x1F600));
}

#[test]
fn test_char_boundaries_start() {
    let input_at = InputAt {
        pos: 0,
        c: Char(0), // First character in UTF-32
        byte: Some(0),
        len: 1,
    };
    assert_eq!(input_at.char(), Char(0));
}

#[test]
fn test_char_boundaries_end() {
    let input_at = InputAt {
        pos: 4,
        c: Char(255), // Character with maximum valid UTF-32
        byte: Some(255),
        len: 1,
    };
    assert_eq!(input_at.char(), Char(255));
}

#[test]
fn test_char_empty_position() {
    let input_at = InputAt {
        pos: 3,
        c: Char(0), // Assuming empty character case
        byte: None,
        len: 0,
    };
    assert_eq!(input_at.char(), Char(0));
}

