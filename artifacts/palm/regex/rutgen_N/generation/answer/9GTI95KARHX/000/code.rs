// Answer 0

#[test]
fn test_position_new() {
    struct Position {
        offset: usize,
        line: usize,
        column: usize,
    }

    fn new(offset: usize, line: usize, column: usize) -> Position {
        Position { offset, line, column }
    }

    let pos1 = new(0, 1, 1);
    assert_eq!(pos1.offset, 0);
    assert_eq!(pos1.line, 1);
    assert_eq!(pos1.column, 1);

    let pos2 = new(10, 2, 5);
    assert_eq!(pos2.offset, 10);
    assert_eq!(pos2.line, 2);
    assert_eq!(pos2.column, 5);

    let pos3 = new(100, 10, 20);
    assert_eq!(pos3.offset, 100);
    assert_eq!(pos3.line, 10);
    assert_eq!(pos3.column, 20);
}

#[test]
fn test_position_new_boundary() {
    struct Position {
        offset: usize,
        line: usize,
        column: usize,
    }

    fn new(offset: usize, line: usize, column: usize) -> Position {
        Position { offset, line, column }
    }

    let pos_boundary1 = new(usize::MAX, usize::MAX, usize::MAX);
    assert_eq!(pos_boundary1.offset, usize::MAX);
    assert_eq!(pos_boundary1.line, usize::MAX);
    assert_eq!(pos_boundary1.column, usize::MAX);

    let pos_boundary2 = new(0, 0, 0); // Assuming that line and column start from 1, this should not panic
    assert_eq!(pos_boundary2.offset, 0);
    assert_eq!(pos_boundary2.line, 0);
    assert_eq!(pos_boundary2.column, 0);
}

