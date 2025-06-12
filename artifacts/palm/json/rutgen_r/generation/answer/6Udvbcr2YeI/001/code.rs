// Answer 0

#[test]
fn test_peek_position() {
    struct MockIterator {
        pos: Position,
    }

    impl MockIterator {
        fn new(line: usize, column: usize) -> Self {
            MockIterator {
                pos: Position { line, column },
            }
        }
        
        fn position(&self) -> Position {
            self.pos.clone()
        }

        fn peek_position(&self) -> Position {
            self.position()
        }
    }

    // Test case: standard position
    let iterator = MockIterator::new(1, 1);
    let pos = iterator.peek_position();
    assert_eq!(pos.line, 1);
    assert_eq!(pos.column, 1);
    
    // Test case: zero position
    let iterator_zero = MockIterator::new(0, 0);
    let pos_zero = iterator_zero.peek_position();
    assert_eq!(pos_zero.line, 0);
    assert_eq!(pos_zero.column, 0);
    
    // Test case: large numbers
    let iterator_large = MockIterator::new(usize::MAX, usize::MAX);
    let pos_large = iterator_large.peek_position();
    assert_eq!(pos_large.line, usize::MAX);
    assert_eq!(pos_large.column, usize::MAX);
}

