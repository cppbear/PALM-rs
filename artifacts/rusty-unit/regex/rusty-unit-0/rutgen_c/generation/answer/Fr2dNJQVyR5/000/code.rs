// Answer 0

#[test]
fn test_capture_index_with_capture_index() {
    let span = Span { start: 0, end: 1 };
    let group = Group {
        span,
        kind: GroupKind::CaptureIndex(1),
        ast: Box::new(Ast::Empty(span)),
    };
    assert_eq!(group.capture_index(), Some(1));
}

#[test]
fn test_capture_index_with_capture_name() {
    let span = Span { start: 0, end: 1 };
    let capture_name = CaptureName {
        span,
        name: "test".to_string(),
        index: 2,
    };
    let group = Group {
        span,
        kind: GroupKind::CaptureName {
            name: capture_name.name,
            index: capture_name.index,
        },
        ast: Box::new(Ast::Empty(span)),
    };
    assert_eq!(group.capture_index(), Some(2));
}

#[test]
fn test_capture_index_with_non_capturing() {
    let span = Span { start: 0, end: 1 };
    let group = Group {
        span,
        kind: GroupKind::NonCapturing,
        ast: Box::new(Ast::Empty(span)),
    };
    assert_eq!(group.capture_index(), None);
}

