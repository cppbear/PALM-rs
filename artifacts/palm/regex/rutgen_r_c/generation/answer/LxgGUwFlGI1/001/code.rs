// Answer 0

#[test]
fn test_next_pos_with_zero_length() {
    let input_at = InputAt {
        pos: 0,
        c: Char(0),
        byte: Some(1),
        len: 0,
    };
    assert_eq!(input_at.next_pos(), 0);
}

#[test]
fn test_next_pos_with_positive_length() {
    let input_at = InputAt {
        pos: 5,
        c: Char(97), // 'a'
        byte: Some(97),
        len: 10,
    };
    assert_eq!(input_at.next_pos(), 15);
}

#[test]
fn test_next_pos_with_max_usize() {
    let input_at = InputAt {
        pos: usize::MAX - 1,
        c: Char(97), // 'a'
        byte: Some(1),
        len: 1,
    };
    assert_eq!(input_at.next_pos(), usize::MAX);
}

#[test]
fn test_next_pos_with_large_length() {
    let input_at = InputAt {
        pos: 1,
        c: Char(97), // 'a'
        byte: Some(1),
        len: usize::MAX - 1,
    };
    assert_eq!(input_at.next_pos(), 0); // Wrap around due to usize overflow
}

