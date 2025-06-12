// Answer 0

#[test]
fn test_is_start_at_beginning() {
    let input_at = InputAt {
        pos: 0,
        c: Char(97), // 'a'
        byte: Some(97),
        len: 1,
    };
    assert!(input_at.is_start());
}

#[test]
fn test_is_start_not_at_beginning() {
    let input_at = InputAt {
        pos: 1,
        c: Char(97), // 'a'
        byte: Some(97),
        len: 1,
    };
    assert!(!input_at.is_start());
}

#[test]
fn test_is_start_at_position_two() {
    let input_at = InputAt {
        pos: 2,
        c: Char(97), // 'a'
        byte: Some(97),
        len: 1,
    };
    assert!(!input_at.is_start());
}

