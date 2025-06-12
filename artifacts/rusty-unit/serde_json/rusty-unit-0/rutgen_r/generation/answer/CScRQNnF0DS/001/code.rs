// Answer 0

#[derive(Debug)]
struct Position {
    line: usize,
    column: usize,
}

struct IteratorMock {
    line_count: usize,
    column_count: usize,
}

impl IteratorMock {
    fn new(line_count: usize, column_count: usize) -> Self {
        IteratorMock {
            line_count,
            column_count,
        }
    }

    fn line(&self) -> usize {
        self.line_count
    }

    fn col(&self) -> usize {
        self.column_count
    }
}

struct TestStruct {
    iter: IteratorMock,
}

impl TestStruct {
    fn position(&self) -> Position {
        Position {
            line: self.iter.line(),
            column: self.iter.col(),
        }
    }
}

#[test]
fn test_position_with_zero_line_column() {
    let test_struct = TestStruct {
        iter: IteratorMock::new(0, 0),
    };
    let pos = test_struct.position();
    assert_eq!(pos.line, 0);
    assert_eq!(pos.column, 0);
}

#[test]
fn test_position_with_small_values() {
    let test_struct = TestStruct {
        iter: IteratorMock::new(1, 2),
    };
    let pos = test_struct.position();
    assert_eq!(pos.line, 1);
    assert_eq!(pos.column, 2);
}

#[test]
fn test_position_with_large_values() {
    let test_struct = TestStruct {
        iter: IteratorMock::new(usize::MAX, usize::MAX),
    };
    let pos = test_struct.position();
    assert_eq!(pos.line, usize::MAX);
    assert_eq!(pos.column, usize::MAX);
}

#[test]
fn test_position_with_non_zero() {
    let test_struct = TestStruct {
        iter: IteratorMock::new(5, 10),
    };
    let pos = test_struct.position();
    assert_eq!(pos.line, 5);
    assert_eq!(pos.column, 10);
}

