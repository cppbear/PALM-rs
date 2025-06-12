// Answer 0

#[test]
fn test_flags_non_capturing_with_flags() {
    let span = Span { start: Position(0), end: Position(10) };
    let flags_item = FlagsItem::Flag('i');
    let flags = Flags { span, items: vec![flags_item] };
    let group = Group { span, kind: GroupKind::NonCapturing(flags.clone()), ast: Box::new(Ast::Empty(span)) };
    
    assert_eq!(group.flags(), Some(&flags));
}

#[test]
fn test_flags_non_capturing_without_flags() {
    let span = Span { start: Position(0), end: Position(10) };
    let group = Group { span, kind: GroupKind::NonCapturing(Flags { span, items: vec![] }), ast: Box::new(Ast::Empty(span)) };
    
    assert_eq!(group.flags().unwrap().items.len(), 0);
}

#[test]
fn test_flags_capturing_group() {
    let span = Span { start: Position(0), end: Position(10) };
    let group = Group { span, kind: GroupKind::CaptureIndex(1), ast: Box::new(Ast::Empty(span)) };
    
    assert_eq!(group.flags(), None);
}

#[test]
fn test_flags_named_capturing_group() {
    let span = Span { start: Position(0), end: Position(10) };
    let group = Group { span, kind: GroupKind::CaptureName { name: "name".to_string(), index: 1 }, ast: Box::new(Ast::Empty(span)) };
    
    assert_eq!(group.flags(), None);
}

