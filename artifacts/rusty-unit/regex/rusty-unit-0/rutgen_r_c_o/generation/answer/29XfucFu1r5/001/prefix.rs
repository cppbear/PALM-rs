// Answer 0

#[test]
fn test_into_class_set_item_dot() {
    let span = Span { start: 0, end: 1 };
    let primitive = Primitive::Dot(span);
    let parser = ParserI::new((), "");
    let _ = primitive.into_class_set_item(&parser);
}

#[test]
fn test_into_class_set_item_assertion() {
    let span = Span { start: 0, end: 1 };
    let assertion = Assertion { span, kind: AssertionKind::B };
    let primitive = Primitive::Assertion(assertion);
    let parser = ParserI::new((), "");
    let _ = primitive.into_class_set_item(&parser);
}

#[test]
fn test_into_class_set_item_invalid_unicode() {
    let span = Span { start: 0, end: 2 };
    let class_unicode = ClassUnicode { span, negated: false, kind: ClassUnicodeKind::Other };
    let primitive = Primitive::Unicode(class_unicode);
    let parser = ParserI::new((), "");
    let _ = primitive.into_class_set_item(&parser);
}

