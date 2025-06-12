// Answer 0

#[test]
fn test_is_capturing_capture_index() {
    struct DummyAst; // Dummy struct to satisfy the type for the Group struct
    let group = Group {
        span: Span { start: Position(0), end: Position(5) },
        kind: GroupKind::CaptureIndex(1),
        ast: Box::new(Ast::Empty(Span { start: Position(0), end: Position(0) })),
    };
    assert_eq!(group.is_capturing(), true);
}

#[test]
fn test_is_capturing_capture_name() {
    struct DummyAst; // Dummy struct to satisfy the type for the Group struct
    let group = Group {
        span: Span { start: Position(0), end: Position(5) },
        kind: GroupKind::CaptureName { name: String::from("group1"), index: 0 },
        ast: Box::new(Ast::Empty(Span { start: Position(0), end: Position(0) })),
    };
    assert_eq!(group.is_capturing(), true);
}

#[test]
fn test_is_capturing_non_capturing() {
    struct DummyAst; // Dummy struct to satisfy the type for the Group struct
    let group = Group {
        span: Span { start: Position(0), end: Position(5) },
        kind: GroupKind::NonCapturing,
        ast: Box::new(Ast::Empty(Span { start: Position(0), end: Position(0) })),
    };
    assert_eq!(group.is_capturing(), false);
}

