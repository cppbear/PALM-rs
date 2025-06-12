// Answer 0

#[test]
fn test_position() {
    struct Delegate {
        position: Position,
    }

    impl Delegate {
        fn new(position: Position) -> Self {
            Self { position }
        }

        fn position(&self) -> Position {
            self.position
        }
    }

    struct TestStruct {
        delegate: Delegate,
    }

    impl TestStruct {
        fn new(delegate: Delegate) -> Self {
            Self { delegate }
        }

        fn position(&self) -> Position {
            self.delegate.position()
        }
    }

    let pos = Position { x: 5, y: 10 }; // Assuming Position has fields x and y
    let delegate = Delegate::new(pos);
    let test_struct = TestStruct::new(delegate);

    let result = test_struct.position();
    assert_eq!(result.x, 5);
    assert_eq!(result.y, 10);
}

