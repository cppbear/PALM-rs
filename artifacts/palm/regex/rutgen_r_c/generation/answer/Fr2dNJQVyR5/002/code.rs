// Answer 0

#[test]
fn test_capture_index_with_capture_name() {
    struct DummyAst;
    
    let span = Span { start: Position { /* values */ }, end: Position { /* values */ } };
    let capture_name = CaptureName { span, name: "test".to_string(), index: 5 };
    let group = Group {
        span,
        kind: GroupKind::CaptureName {
            name: capture_name.name.clone(),
            index: capture_name.index,
        },
        ast: Box::new(DummyAst),
    };

    assert_eq!(group.capture_index(), Some(5));
}

#[test]
fn test_capture_index_with_capture_index() {
    struct DummyAst;

    let span = Span { start: Position { /* values */ }, end: Position { /* values */ } };
    let group = Group {
        span,
        kind: GroupKind::CaptureIndex(3),
        ast: Box::new(DummyAst),
    };

    assert_eq!(group.capture_index(), Some(3));
}

#[test]
fn test_capture_index_with_non_capturing() {
    struct DummyAst;

    let span = Span { start: Position { /* values */ }, end: Position { /* values */ } };
    let group = Group {
        span,
        kind: GroupKind::NonCapturing,
        ast: Box::new(DummyAst),
    };

    assert_eq!(group.capture_index(), None);
}

