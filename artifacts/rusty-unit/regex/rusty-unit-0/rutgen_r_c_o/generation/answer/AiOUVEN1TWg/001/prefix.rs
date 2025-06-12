// Answer 0

#[test]
fn test_is_start_at_position_zero() {
    let input = InputAt { pos: 0, c: Char(97), byte: Some(97), len: 1 };
    input.is_start();
}

#[test]
fn test_is_start_at_position_one() {
    let input = InputAt { pos: 1, c: Char(98), byte: Some(98), len: 1 };
    input.is_start();
}

#[test]
fn test_is_start_at_position_two() {
    let input = InputAt { pos: 2, c: Char(99), byte: Some(99), len: 1 };
    input.is_start();
}

#[test]
fn test_is_start_at_max_unsigned_position() {
    let input = InputAt { pos: u32::MAX as usize, c: Char(100), byte: Some(100), len: 1 };
    input.is_start();
}

