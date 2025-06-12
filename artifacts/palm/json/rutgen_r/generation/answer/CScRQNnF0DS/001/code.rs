// Answer 0

#[derive(Debug)]
struct Position {
    line: usize,
    column: usize,
}

struct Iter {
    line: usize,
    col: usize,
}

impl Iter {
    fn new(line: usize, col: usize) -> Self {
        Iter { line, col }
    }
    
    fn line(&self) -> usize {
        self.line
    }
    
    fn col(&self) -> usize {
        self.col
    }
}

struct TestStruct {
    iter: Iter,
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
fn test_position() {
    let test_struct = TestStruct {
        iter: Iter::new(1, 5),
    };
    
    let pos = test_struct.position();
    assert_eq!(pos.line, 1);
    assert_eq!(pos.column, 5);
}

#[test]
fn test_position_with_zero() {
    let test_struct = TestStruct {
        iter: Iter::new(0, 0),
    };
    
    let pos = test_struct.position();
    assert_eq!(pos.line, 0);
    assert_eq!(pos.column, 0);
}

#[test]
fn test_position_with_large_values() {
    let test_struct = TestStruct {
        iter: Iter::new(usize::MAX, usize::MAX),
    };
    
    let pos = test_struct.position();
    assert_eq!(pos.line, usize::MAX);
    assert_eq!(pos.column, usize::MAX);
}

