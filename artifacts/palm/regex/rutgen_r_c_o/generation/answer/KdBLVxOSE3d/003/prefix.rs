// Answer 0

#[test]
fn test_is_capturing_capture_index() {
    let group = Group {
        span: Span {
            start: Position(0),
            end: Position(1),
        },
        kind: GroupKind::CaptureIndex(1),
        ast: Box::new(Ast::Empty(Span {
            start: Position(0),
            end: Position(0),
        })),
    };
    group.is_capturing();
}

#[test]
fn test_is_capturing_capture_name() {
    let group = Group {
        span: Span {
            start: Position(0),
            end: Position(1),
        },
        kind: GroupKind::CaptureName {
            name: String::from("name"),
            index: 2,
        },
        ast: Box::new(Ast::Empty(Span {
            start: Position(0),
            end: Position(0),
        })),
    };
    group.is_capturing();
}

#[test]
fn test_is_capturing_high_index_capture_name() {
    let group = Group {
        span: Span {
            start: Position(0),
            end: Position(1),
        },
        kind: GroupKind::CaptureName {
            name: String::from("high_index"),
            index: 1000,
        },
        ast: Box::new(Ast::Empty(Span {
            start: Position(0),
            end: Position(0),
        })),
    };
    group.is_capturing();
}

#[test]
fn test_is_capturing_non_capturing() {
    let group = Group {
        span: Span {
            start: Position(0),
            end: Position(1),
        },
        kind: GroupKind::NonCapturing,
        ast: Box::new(Ast::Empty(Span {
            start: Position(0),
            end: Position(0),
        })),
    };
    group.is_capturing(); // This should not panic, but returns false.
}

