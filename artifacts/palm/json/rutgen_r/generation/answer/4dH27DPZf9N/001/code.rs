// Answer 0

#[test]
fn test_peek_position_empty() {
    struct TestReader;

    impl TestReader {
        fn peek_position(&self) -> Position {
            Position::new(0, 0) // Assuming starting position is (0, 0)
        }
    }

    let reader = TestReader;
    let position = reader.peek_position();
    assert_eq!(position, Position::new(0, 0));
}

#[test]
fn test_peek_position_non_empty() {
    struct TestReader {
        position: Position,
    }

    impl TestReader {
        fn new(x: usize, y: usize) -> Self {
            TestReader {
                position: Position::new(x, y),
            }
        }

        fn peek_position(&self) -> Position {
            self.position
        }
    }

    let reader = TestReader::new(5, 10);
    let position = reader.peek_position();
    assert_eq!(position, Position::new(5, 10));
}

#[test]
#[should_panic]
fn test_peek_position_invalid() {
    struct TestReader;

    impl TestReader {
        fn peek_position(&self) -> Position {
            panic!("Invalid position") // Simulating a panic condition
        }
    }

    let reader = TestReader;
    reader.peek_position(); // This call should trigger a panic
}

