// Answer 0

#[test]
fn test_is_capturing_capture_index() {
    let group = Group {
        span: Span { start: Position(0), end: Position(1) },
        kind: GroupKind::CaptureIndex(1),
        ast: Box::new(Ast::Empty(Span { start: Position(0), end: Position(0) })),
    };
    let _ = group.is_capturing();
}

#[test]
fn test_is_capturing_capture_name() {
    let group = Group {
        span: Span { start: Position(0), end: Position(2) },
        kind: GroupKind::CaptureName { name: String::from("group1"), index: 0 },
        ast: Box::new(Ast::Empty(Span { start: Position(0), end: Position(0) })),
    };
    let _ = group.is_capturing();
}

#[test]
fn test_is_capturing_capture_name_large_index() {
    let group = Group {
        span: Span { start: Position(0), end: Position(3) },
        kind: GroupKind::CaptureName { name: String::from("large_index_group"), index: 4294967295 },
        ast: Box::new(Ast::Empty(Span { start: Position(0), end: Position(0) })),
    };
    let _ = group.is_capturing();
}

#[test]
fn test_is_capturing_empty_name() {
    let group = Group {
        span: Span { start: Position(0), end: Position(1) },
        kind: GroupKind::CaptureName { name: String::from(""), index: 1 },
        ast: Box::new(Ast::Empty(Span { start: Position(0), end: Position(0) })),
    };
    let _ = group.is_capturing();
}

