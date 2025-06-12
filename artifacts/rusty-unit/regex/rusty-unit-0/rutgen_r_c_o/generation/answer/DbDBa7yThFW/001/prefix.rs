// Answer 0

#[test]
fn test_pos_zero() {
    let input_at = InputAt { pos: 0, c: Char(1), byte: Some(0), len: 1 };
    input_at.pos();
}

#[test]
fn test_pos_small_positive() {
    let input_at = InputAt { pos: 1, c: Char(2), byte: Some(1), len: 2 };
    input_at.pos();
}

#[test]
fn test_pos_large_positive() {
    let input_at = InputAt { pos: usize::MAX, c: Char(3), byte: Some(2), len: 3 };
    input_at.pos();
} 

#[test]
fn test_pos_large_ranged_value() {
    let input_at = InputAt { pos: usize::MAX - 1, c: Char(4), byte: Some(3), len: 4 };
    input_at.pos();
}

#[test]
fn test_pos_mid_range() {
    let input_at = InputAt { pos: usize::MAX / 2, c: Char(5), byte: Some(4), len: 5 };
    input_at.pos();
}

