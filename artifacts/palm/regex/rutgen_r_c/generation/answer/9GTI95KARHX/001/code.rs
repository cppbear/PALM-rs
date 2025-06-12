// Answer 0

#[test]
fn test_new_position_valid() {
    let pos = Position::new(10, 1, 1);
    assert_eq!(pos.offset, 10);
    assert_eq!(pos.line, 1);
    assert_eq!(pos.column, 1);
}

#[test]
fn test_new_position_zero_values() {
    let pos = Position::new(0, 1, 1);
    assert_eq!(pos.offset, 0);
    assert_eq!(pos.line, 1);
    assert_eq!(pos.column, 1);
}

#[test]
fn test_new_position_large_values() {
    let pos = Position::new(usize::MAX, 1000, 1000);
    assert_eq!(pos.offset, usize::MAX);
    assert_eq!(pos.line, 1000);
    assert_eq!(pos.column, 1000);
}

#[test]
fn test_new_position_extreme_line_column() {
    let pos = Position::new(5, 100, 1);
    assert_eq!(pos.offset, 5);
    assert_eq!(pos.line, 100);
    assert_eq!(pos.column, 1);
}

