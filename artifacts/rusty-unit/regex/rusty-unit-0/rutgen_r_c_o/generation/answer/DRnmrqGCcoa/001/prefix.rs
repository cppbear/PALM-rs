// Answer 0

#[test]
fn test_flags_with_capture_index() {
    let span = Span { start: Position(0), end: Position(10) };
    let group = Group {
        span,
        kind: GroupKind::CaptureIndex(0),
        ast: Box::new(Ast::Empty(span)),
    };
    group.flags();
}

#[test]
fn test_flags_with_capture_name() {
    let span = Span { start: Position(0), end: Position(10) };
    let capture_name = GroupKind::CaptureName { name: "test".to_string(), index: 1 };
    let group = Group {
        span,
        kind: capture_name,
        ast: Box::new(Ast::Empty(span)),
    };
    group.flags();
}

#[test]
fn test_flags_with_non_capturing() {
    let span = Span { start: Position(0), end: Position(10) };
    let flags = Flags { span, items: Vec::new() };
    let group = Group {
        span,
        kind: GroupKind::NonCapturing,
        ast: Box::new(Ast::Empty(span)),
    };
    group.flags();
}

