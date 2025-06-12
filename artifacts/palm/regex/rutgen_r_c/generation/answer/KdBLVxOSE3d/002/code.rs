// Answer 0

#[test]
fn test_is_capturing_with_capture_index() {
    struct CaptureIndexExample {
        span: Span,
        kind: GroupKind,
        ast: Box<Ast>,
    }
    
    let group = Group {
        span: Span { start: Position(0), end: Position(1) },
        kind: GroupKind::CaptureIndex(1),
        ast: Box::new(Ast::Empty(Span { start: Position(0), end: Position(1) })),
    };

    assert!(group.is_capturing());
}

#[test]
fn test_is_capturing_with_capture_name() {
    struct CaptureNameExample {
        span: Span,
        kind: GroupKind,
        ast: Box<Ast>,
    }

    let group = Group {
        span: Span { start: Position(0), end: Position(1) },
        kind: GroupKind::CaptureName {
            name: String::from("example"),
            index: 2,
        },
        ast: Box::new(Ast::Empty(Span { start: Position(0), end: Position(1) })),
    };

    assert!(group.is_capturing());
}

#[test]
fn test_is_capturing_with_non_capturing() {
    let group = Group {
        span: Span { start: Position(0), end: Position(1) },
        kind: GroupKind::NonCapturing,
        ast: Box::new(Ast::Empty(Span { start: Position(0), end: Position(1) })),
    };

    assert!(!group.is_capturing());
}

