// Answer 0

#[test]
fn test_new_with_minimum_values() {
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 0, line: 1, column: 1 };
    let span = new(start, end);
}

#[test]
fn test_new_with_equal_offsets() {
    let start = Position { offset: 10, line: 2, column: 1 };
    let end = Position { offset: 10, line: 2, column: 1 };
    let span = new(start, end);
}

#[test]
fn test_new_with_increasing_offsets() {
    let start = Position { offset: 5, line: 3, column: 5 };
    let end = Position { offset: 15, line: 3, column: 10 };
    let span = new(start, end);
}

#[test]
fn test_new_with_different_lines() {
    let start = Position { offset: 100, line: 1, column: 10 };
    let end = Position { offset: 150, line: 2, column: 5 };
    let span = new(start, end);
}

#[test]
fn test_new_with_full_range() {
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 1000, line: 100, column: 100 };
    let span = new(start, end);
}

#[test]
fn test_new_with_same_column_different_lines() {
    let start = Position { offset: 20, line: 2, column: 30 };
    let end = Position { offset: 25, line: 3, column: 30 };
    let span = new(start, end);
}

#[test]
fn test_new_with_maximum_values() {
    let start = Position { offset: 1000, line: 100, column: 100 };
    let end = Position { offset: 1000, line: 100, column: 100 };
    let span = new(start, end);
}

