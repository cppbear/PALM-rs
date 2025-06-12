// Answer 0

#[test]
fn test_next_pos_empty_input() {
    let input = InputAt {
        pos: 0,
        c: Char(0),
        byte: None,
        len: 0,
    };
    assert_eq!(input.next_pos(), 0);
}

#[test]
fn test_next_pos_non_empty_input() {
    let input = InputAt {
        pos: 5,
        c: Char(1),
        byte: Some(10),
        len: 3,
    };
    assert_eq!(input.next_pos(), 8);
}

#[test]
fn test_next_pos_with_length() {
    let input = InputAt {
        pos: 10,
        c: Char(2),
        byte: Some(20),
        len: 5,
    };
    assert_eq!(input.next_pos(), 15);
}

#[test]
fn test_next_pos_boundary() {
    let input = InputAt {
        pos: usize::MAX - 1,
        c: Char(3),
        byte: Some(30),
        len: 1,
    };
    assert_eq!(input.next_pos(), usize::MAX);
}

