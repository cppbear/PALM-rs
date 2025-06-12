// Answer 0

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

struct Delegate;

impl Delegate {
    fn position(&self) -> Position {
        Position { x: 10, y: 20 }
    }
}

struct MyStruct {
    delegate: Delegate,
}

impl MyStruct {
    fn position(&self) -> Position {
        self.delegate.position()
    }
}

#[test]
fn test_position() {
    let my_struct = MyStruct { delegate: Delegate };
    let pos = my_struct.position();
    assert_eq!(pos.x, 10);
    assert_eq!(pos.y, 20);
}

