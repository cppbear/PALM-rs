// Answer 0

#[test]
fn test_flags_capture_index() {
    let span = Span { start: Position::from(0), end: Position::from(1) };
    let group = Group {
        span,
        kind: GroupKind::CaptureIndex(1),
        ast: Box::new(Ast::Empty(span)),
    };
    
    assert_eq!(group.flags(), None);
}

#[test]
fn test_flags_capture_name() {
    let span = Span { start: Position::from(0), end: Position::from(1) };
    let group = Group {
        span,
        kind: GroupKind::CaptureName { name: String::from("group1"), index: 0 },
        ast: Box::new(Ast::Empty(span)),
    };
    
    assert_eq!(group.flags(), None);
}

#[test]
fn test_flags_non_capturing_with_flags() {
    let span = Span { start: Position::from(0), end: Position::from(1) };
    let flags = Flags {
        span,
        items: vec![], // Empty items for this test case
    };
    let group = Group {
        span,
        kind: GroupKind::NonCapturing(flags),
        ast: Box::new(Ast::Empty(span)),
    };
    
    assert_eq!(group.flags(), Some(&group.kind.as_non_capturing_flags().unwrap()));
}

