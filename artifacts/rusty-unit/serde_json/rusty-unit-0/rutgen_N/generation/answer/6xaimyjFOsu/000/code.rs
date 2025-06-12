// Answer 0

#[derive(Debug)]
struct Position {
    line: usize,
    column: usize,
}

trait Delegate {
    fn peek_position(&self) -> Position;
}

struct MyDelegate;

impl Delegate for MyDelegate {
    fn peek_position(&self) -> Position {
        Position { line: 1, column: 20 }
    }
}

struct StructUnderTest {
    delegate: MyDelegate,
}

impl StructUnderTest {
    fn peek_position(&self) -> Position {
        self.delegate.peek_position()
    }
}

#[test]
fn test_peek_position() {
    let struct_under_test = StructUnderTest {
        delegate: MyDelegate,
    };
    let position = struct_under_test.peek_position();
    assert_eq!(position.line, 1);
    assert_eq!(position.column, 20);
}

