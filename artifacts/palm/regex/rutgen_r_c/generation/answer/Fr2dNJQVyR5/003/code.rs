// Answer 0

#[test]
fn test_capture_index_with_capture_index() {
    let span = Span { start: Position { byte: 0 }, end: Position { byte: 5 } };
    let group = Group {
        span,
        kind: GroupKind::CaptureIndex(1),
        ast: Box::new(Ast::Empty(span.clone())),
    };
    assert_eq!(group.capture_index(), Some(1));
}

#[test]
fn test_capture_index_with_capture_name() {
    let span = Span { start: Position { byte: 0 }, end: Position { byte: 5 } };
    let capture_name = CaptureName {
        span,
        name: String::from("test"),
        index: 2,
    };
    let group = Group {
        span,
        kind: GroupKind::CaptureName(capture_name),
        ast: Box::new(Ast::Empty(span.clone())),
    };
    assert_eq!(group.capture_index(), Some(2));
}

#[test]
fn test_capture_index_with_non_capturing() {
    let span = Span { start: Position { byte: 0 }, end: Position { byte: 5 } };
    let group = Group {
        span,
        kind: GroupKind::NonCapturing,
        ast: Box::new(Ast::Empty(span.clone())),
    };
    assert_eq!(group.capture_index(), None);
}

