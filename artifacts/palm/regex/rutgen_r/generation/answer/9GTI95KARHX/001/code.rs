// Answer 0

#[derive(Debug, PartialEq)]
struct Position {
    offset: usize,
    line: usize,
    column: usize,
}

fn new(offset: usize, line: usize, column: usize) -> Position {
    Position { offset, line, column }
}

#[test]
fn test_new_position_valid() {
    let pos = new(0, 1, 1);
    assert_eq!(pos, Position { offset: 0, line: 1, column: 1 });

    let pos = new(5, 2, 3);
    assert_eq!(pos, Position { offset: 5, line: 2, column: 3 });

    let pos = new(10, 3, 15);
    assert_eq!(pos, Position { offset: 10, line: 3, column: 15 });
}

#[test]
#[should_panic]
fn test_new_position_negative_offset() {
    let _pos = new(usize::MAX, 1, 1); // Using usize::MAX to simulate a panic condition
}

#[test]
#[should_panic]
fn test_new_position_zero_line() {
    let _pos = new(0, 0, 1); // Line number should start at 1
}

#[test]
#[should_panic]
fn test_new_position_zero_column() {
    let _pos = new(0, 1, 0); // Column number should start at 1
}

