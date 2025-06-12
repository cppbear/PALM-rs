// Answer 0

#[test]
fn test_with_start() {
    struct Position {
        value: usize,
    }

    struct Span {
        start: Position,
        end: Position,
    }

    impl Span {
        fn new(start: Position, end: Position) -> Self {
            Self { start, end }
        }

        pub fn with_start(self, pos: Position) -> Span {
            Span { start: pos, ..self }
        }
    }

    let initial_start = Position { value: 0 };
    let initial_end = Position { value: 10 };
    let original_span = Span::new(initial_start.clone(), initial_end.clone());

    // Test with a valid position
    let new_position = Position { value: 5 };
    let updated_span = original_span.with_start(new_position.clone());
    assert_eq!(updated_span.start.value, new_position.value);
    assert_eq!(updated_span.end.value, initial_end.value);

    // Test with the same position (boundary condition)
    let updated_span_same = original_span.with_start(initial_start.clone());
    assert_eq!(updated_span_same.start.value, initial_start.value);
    assert_eq!(updated_span_same.end.value, initial_end.value);

    // Test with a position at the edge of a possible range
    let edge_position = Position { value: usize::MAX };
    let updated_span_edge = original_span.with_start(edge_position.clone());
    assert_eq!(updated_span_edge.start.value, edge_position.value);
    assert_eq!(updated_span_edge.end.value, initial_end.value);
}

