// Answer 0

#[test]
fn test_is_capturing_with_capture_index() {
    let span = Span { start: Position(0), end: Position(1) }; // Assuming Position is defined somewhere
    let group = Group {
        span,
        kind: GroupKind::CaptureIndex(0),
        ast: Box::new(Ast::Empty(span)),
    };
    assert_eq!(group.is_capturing(), true);
}

#[test]
fn test_is_capturing_with_capture_name() {
    let span = Span { start: Position(0), end: Position(1) }; // Assuming Position is defined somewhere
    let group = Group {
        span,
        kind: GroupKind::CaptureName { name: String::from("test"), index: 0 },
        ast: Box::new(Ast::Empty(span)),
    };
    assert_eq!(group.is_capturing(), true);
}

#[test]
fn test_is_capturing_with_non_capturing() {
    let span = Span { start: Position(0), end: Position(1) }; // Assuming Position is defined somewhere
    let group = Group {
        span,
        kind: GroupKind::NonCapturing,
        ast: Box::new(Ast::Empty(span)),
    };
    assert_eq!(group.is_capturing(), false);
}

