// Answer 0

#[test]
fn test_position_new_zero_values() {
    let position = Position::new(0, 1, 1);
}

#[test]
fn test_position_new_large_values() {
    let position = Position::new(1_000_000_000_000_000_000, 1_000_000_000_000_000_000, 1_000_000_000_000_000_000);
}

#[test]
fn test_position_new_incremental_values() {
    for i in 0..5 {
        let position = Position::new(i, i + 1, i + 1);
    }
}

#[test]
fn test_position_new_max_values() {
    let max_offset = u64::MAX;
    let max_line = u64::MAX;
    let max_column = u64::MAX;
    let position = Position::new(max_offset, max_line, max_column);
}

#[test]
fn test_position_new_with_one() {
    let position = Position::new(1, 1, 1);
}

#[test]
fn test_position_new_high_values() {
    let position = Position::new(2_000_000_000, 2_000_000_000, 2_000_000_000);
}

