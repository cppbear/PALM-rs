// Answer 0

#[derive(Debug)]
struct Position {
    line: usize,
    column: usize,
}

trait R {
    fn peek_position(&self) -> Position;
}

struct TestStruct;

impl R for TestStruct {
    fn peek_position(&self) -> Position {
        Position { line: 1, column: 10 }
    }
}

impl TestStruct {
    fn peek_position(&self) -> Position {
        R::peek_position(self)
    }
}

#[test]
fn test_peek_position() {
    let test_instance = TestStruct;
    let position = test_instance.peek_position();
    assert_eq!(position.line, 1);
    assert_eq!(position.column, 10);
}

