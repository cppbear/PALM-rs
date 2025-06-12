// Answer 0

#[test]
fn test_capture_index_with_positive_index() {
    let group = Group {
        span: Span { start: Position(0), end: Position(10) },
        kind: GroupKind::CaptureIndex(1),
        ast: Box::new(Ast::Empty(Span { start: Position(0), end: Position(0) })),
    };
    group.capture_index();
}

#[test]
fn test_capture_index_with_large_index() {
    let group = Group {
        span: Span { start: Position(0), end: Position(10) },
        kind: GroupKind::CaptureIndex(4294967295),
        ast: Box::new(Ast::Empty(Span { start: Position(0), end: Position(0) })),
    };
    group.capture_index();
}

#[test]
fn test_capture_index_with_named_capture() {
    let capture_name = CaptureName {
        span: Span { start: Position(0), end: Position(10) },
        name: String::from("test"),
        index: 2,
    };
    let group = Group {
        span: Span { start: Position(0), end: Position(10) },
        kind: GroupKind::CaptureName(capture_name),
        ast: Box::new(Ast::Empty(Span { start: Position(0), end: Position(0) })),
    };
    group.capture_index();
}

#[test]
fn test_capture_index_with_non_capturing_group() {
    let group = Group {
        span: Span { start: Position(0), end: Position(10) },
        kind: GroupKind::NonCapturing,
        ast: Box::new(Ast::Empty(Span { start: Position(0), end: Position(0) })),
    };
    group.capture_index();
}

