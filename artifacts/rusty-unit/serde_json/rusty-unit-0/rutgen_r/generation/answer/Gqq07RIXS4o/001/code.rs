// Answer 0

#[derive(Default)]
struct Delegate {
    position: Position,
}

#[derive(Debug, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Delegate {
    fn position(&self) -> Position {
        self.position.clone()
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
    let position = Position { x: 5, y: 10 };
    let delegate = Delegate { position };
    let my_struct = MyStruct { delegate };

    let result = my_struct.position();
    assert_eq!(result, Position { x: 5, y: 10 });
}

#[test]
fn test_position_default() {
    let delegate = Delegate::default();
    let my_struct = MyStruct { delegate };

    let result = my_struct.position();
    assert_eq!(result, Position { x: 0, y: 0 });
}

#[should_panic]
#[test]
fn test_position_panic() {
    struct PanicDelegate;
    
    impl PanicDelegate {
        fn position(&self) -> Position {
            panic!("Panic triggered in position")
        }
    }

    let my_struct = MyStruct { delegate: PanicDelegate };
    let _ = my_struct.position();
}

