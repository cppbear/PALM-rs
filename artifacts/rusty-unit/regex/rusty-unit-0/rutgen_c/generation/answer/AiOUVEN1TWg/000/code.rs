// Answer 0

#[test]
fn test_is_start_at_zero() {
    let input = InputAt { pos: 0, c: Char(0), byte: None, len: 1 };
    assert!(input.is_start());
}

#[test]
fn test_is_start_not_at_zero() {
    let input = InputAt { pos: 1, c: Char(0), byte: None, len: 1 };
    assert!(!input.is_start());
}

#[test]
fn test_is_start_at_large_position() {
    let input = InputAt { pos: 100, c: Char(0), byte: None, len: 1 };
    assert!(!input.is_start());
}

