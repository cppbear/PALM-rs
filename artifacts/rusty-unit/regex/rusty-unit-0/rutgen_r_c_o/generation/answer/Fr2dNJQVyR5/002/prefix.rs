// Answer 0

#[test]
fn test_capture_index_capture_name_zero() {
    let group = Group {
        span: Span { start: Position(0), end: Position(1) },
        kind: GroupKind::CaptureName { name: String::from("test"), index: 0 },
        ast: Box::new(Ast::Empty(Span { start: Position(0), end: Position(1) })),
    };
    group.capture_index();
}

#[test]
fn test_capture_index_capture_name_max() {
    let group = Group {
        span: Span { start: Position(0), end: Position(1) },
        kind: GroupKind::CaptureName { name: String::from("test"), index: u32::MAX },
        ast: Box::new(Ast::Empty(Span { start: Position(0), end: Position(1) })),
    };
    group.capture_index();
}

#[test]
fn test_capture_index_capture_name_mid() {
    let group = Group {
        span: Span { start: Position(0), end: Position(1) },
        kind: GroupKind::CaptureName { name: String::from("example"), index: 123 },
        ast: Box::new(Ast::Empty(Span { start: Position(0), end: Position(1) })),
    };
    group.capture_index();
}

#[test]
fn test_capture_index_capture_name_large_index() {
    let group = Group {
        span: Span { start: Position(0), end: Position(1) },
        kind: GroupKind::CaptureName { name: String::from("large_index"), index: 99999 },
        ast: Box::new(Ast::Empty(Span { start: Position(0), end: Position(1) })),
    };
    group.capture_index();
}

