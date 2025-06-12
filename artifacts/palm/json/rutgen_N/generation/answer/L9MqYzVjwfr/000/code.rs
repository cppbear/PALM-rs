// Answer 0

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

trait R {
    fn position(&self) -> Position;
}

struct TestStruct;

impl R for TestStruct {
    fn position(&self) -> Position {
        Position { x: 10, y: 20 }
    }
}

impl TestStruct {
    fn position(&self) -> Position {
        R::position(self)
    }
}

#[test]
fn test_position() {
    let test_struct = TestStruct;
    let pos = test_struct.position();
    assert_eq!(pos.x, 10);
    assert_eq!(pos.y, 20);
}

