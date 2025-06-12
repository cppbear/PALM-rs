// Answer 0

#[test]
fn test_unicode_class_set_item_span_01() {
    let span = Span { start: Position(0), end: Position(5) };
    let unicode = ClassUnicode { span, negated: false, kind: ClassUnicodeKind::Any };
    let item = ClassSetItem::Unicode(unicode);
    item.span();
}

#[test]
fn test_unicode_class_set_item_span_02() {
    let span = Span { start: Position(10), end: Position(15) };
    let unicode = ClassUnicode { span, negated: true, kind: ClassUnicodeKind::Any };
    let item = ClassSetItem::Unicode(unicode);
    item.span();
}

#[test]
fn test_unicode_class_set_item_span_03() {
    let span = Span { start: Position(20), end: Position(25) };
    let unicode = ClassUnicode { span, negated: false, kind: ClassUnicodeKind::Any };
    let item = ClassSetItem::Unicode(unicode);
    item.span();
}

#[test]
fn test_unicode_class_set_item_span_04() {
    let span = Span { start: Position(30), end: Position(35) };
    let unicode = ClassUnicode { span, negated: true, kind: ClassUnicodeKind::Any };
    let item = ClassSetItem::Unicode(unicode);
    item.span();
}

#[test]
fn test_unicode_class_set_item_span_05() {
    let span = Span { start: Position(50), end: Position(55) };
    let unicode = ClassUnicode { span, negated: false, kind: ClassUnicodeKind::Any };
    let item = ClassSetItem::Unicode(unicode);
    item.span();
}

