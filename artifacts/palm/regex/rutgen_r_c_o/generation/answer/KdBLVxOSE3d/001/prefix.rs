// Answer 0

#[test]
fn test_is_capturing_non_capturing_group() {
    let span = Span { start: Position(0), end: Position(1) };
    let non_capturing_group = Group {
        span,
        kind: GroupKind::NonCapturing,
        ast: Box::new(Ast::Empty(span)),
    };
    non_capturing_group.is_capturing();
}

#[test]
fn test_is_capturing_non_capturing_group_with_ast() {
    let span = Span { start: Position(5), end: Position(10) };
    let non_capturing_group = Group {
        span,
        kind: GroupKind::NonCapturing,
        ast: Box::new(Ast::Flags(SetFlags::default())),
    };
    non_capturing_group.is_capturing();
}

