// Answer 0

#[test]
fn test_position_new_valid() {
    let pos = Position::new(0, 1, 1);
    assert_eq!(pos.offset, 0);
    assert_eq!(pos.line, 1);
    assert_eq!(pos.column, 1);
}

#[test]
fn test_position_new_middle() {
    let pos = Position::new(10, 2, 5);
    assert_eq!(pos.offset, 10);
    assert_eq!(pos.line, 2);
    assert_eq!(pos.column, 5);
}

#[test]
fn test_position_new_large_values() {
    let pos = Position::new(1000, 100, 50);
    assert_eq!(pos.offset, 1000);
    assert_eq!(pos.line, 100);
    assert_eq!(pos.column, 50);
}

#[test]
fn test_position_new_zero_values() {
    let pos = Position::new(0, 0, 0);
    assert_eq!(pos.offset, 0);
    assert_eq!(pos.line, 0);
    assert_eq!(pos.column, 0);
}

#[test]
fn test_position_new_non_zero_line_column() {
    let pos = Position::new(5, 1, 3);
    assert_eq!(pos.offset, 5);
    assert_eq!(pos.line, 1);
    assert_eq!(pos.column, 3);
}

