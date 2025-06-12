// Answer 0

#[test]
fn test_input_at_pos() {
    let input_at = InputAt { pos: 5, c: Char(97), byte: Some(97), len: 1 };
    assert_eq!(input_at.pos(), 5);
}

#[test]
fn test_input_at_pos_boundary_start() {
    let input_at = InputAt { pos: 0, c: Char(97), byte: Some(97), len: 1 };
    assert_eq!(input_at.pos(), 0);
}

#[test]
fn test_input_at_pos_boundary_end() {
    let input_at = InputAt { pos: usize::MAX, c: Char(97), byte: Some(97), len: 1 };
    assert_eq!(input_at.pos(), usize::MAX);
}

