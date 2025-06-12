// Answer 0

#[test]
fn test_pos_valid_position() {
    let input = InputAt {
        pos: 5,
        c: Char(97), // 'a'
        byte: Some(97),
        len: 1,
    };
    assert_eq!(input.pos(), 5);
}

#[test]
fn test_pos_zero_position() {
    let input = InputAt {
        pos: 0,
        c: Char(98), // 'b'
        byte: Some(98),
        len: 1,
    };
    assert_eq!(input.pos(), 0);
}

#[test]
fn test_pos_large_position() {
    let input = InputAt {
        pos: usize::MAX,
        c: Char(99), // 'c'
        byte: Some(99),
        len: 1,
    };
    assert_eq!(input.pos(), usize::MAX);
}

#[test]
fn test_pos_with_non_zero_len() {
    let input = InputAt {
        pos: 10,
        c: Char(100), // 'd'
        byte: Some(100),
        len: 3,
    };
    assert_eq!(input.pos(), 10);
}

