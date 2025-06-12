// Answer 0

#[test]
fn test_capture_index_non_capturing() {
    let span = Span { start: Position(0), end: Position(5) };
    let ast = Box::new(Ast::Empty(span));
    let group = Group {
        span,
        kind: GroupKind::NonCapturing,
        ast,
    };
    group.capture_index();
}

#[test]
fn test_capture_index_capture_index() {
    let span = Span { start: Position(0), end: Position(5) };
    let ast = Box::new(Ast::Empty(span));
    let group = Group {
        span,
        kind: GroupKind::CaptureIndex(5),
        ast,
    };
    group.capture_index();
}

#[test]
fn test_capture_index_capture_name() {
    let span = Span { start: Position(0), end: Position(5) };
    let capture_name = CaptureName { span, name: String::from("test"), index: 3 };
    let ast = Box::new(Ast::Empty(span));
    let group = Group {
        span,
        kind: GroupKind::CaptureName(capture_name),
        ast,
    };
    group.capture_index();
}

#[test]
fn test_capture_index_non_capturing_empty() {
    let span = Span { start: Position(0), end: Position(0) };
    let ast = Box::new(Ast::Empty(span));
    let group = Group {
        span,
        kind: GroupKind::NonCapturing,
        ast,
    };
    group.capture_index();
}

#[test]
fn test_capture_index_capture_name_max() {
    let span = Span { start: Position(0), end: Position(5) };
    let capture_name = CaptureName { span, name: String::from("example"), index: 10 };
    let ast = Box::new(Ast::Empty(span));
    let group = Group {
        span,
        kind: GroupKind::CaptureName(capture_name),
        ast,
    };
    group.capture_index();
}

