// Answer 0

#[test]
fn test_with_end_updates_end_position() {
    struct Span {
        start: usize,
        end: usize,
    }

    struct Position(usize);

    impl Span {
        pub fn with_end(self, pos: Position) -> Span {
            Span { end: pos.0, ..self }
        }
    }

    let span = Span { start: 0, end: 5 };
    let pos = Position(10);
    let new_span = span.with_end(pos);
    assert_eq!(new_span.end, 10);
    assert_eq!(new_span.start, 0);
}

#[test]
fn test_with_end_updates_end_position_to_zero() {
    struct Span {
        start: usize,
        end: usize,
    }

    struct Position(usize);

    impl Span {
        pub fn with_end(self, pos: Position) -> Span {
            Span { end: pos.0, ..self }
        }
    }

    let span = Span { start: 0, end: 5 };
    let pos = Position(0);
    let new_span = span.with_end(pos);
    assert_eq!(new_span.end, 0);
    assert_eq!(new_span.start, 0);
}

#[test]
fn test_with_end_retains_start_position() {
    struct Span {
        start: usize,
        end: usize,
    }

    struct Position(usize);

    impl Span {
        pub fn with_end(self, pos: Position) -> Span {
            Span { end: pos.0, ..self }
        }
    }

    let span = Span { start: 5, end: 10 };
    let pos = Position(7);
    let new_span = span.with_end(pos);
    assert_eq!(new_span.end, 7);
    assert_eq!(new_span.start, 5);
}

