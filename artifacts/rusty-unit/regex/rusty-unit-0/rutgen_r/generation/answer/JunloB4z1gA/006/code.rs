// Answer 0

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(Debug)]
enum ClassSetItem {
    Empty(Span),
    Literal { span: Span },
    Range { span: Span },
    Ascii { span: Span },
    Perl { span: Span },
    Unicode { span: Span },
    Bracketed { span: Span },
    Union { span: Span },
}

impl ClassSetItem {
    pub fn span(&self) -> &Span {
        match *self {
            ClassSetItem::Empty(ref span) => span,
            ClassSetItem::Literal { ref span } => span,
            ClassSetItem::Range { ref span } => span,
            ClassSetItem::Ascii { ref span } => span,
            ClassSetItem::Perl { ref span } => span,
            ClassSetItem::Unicode { ref span } => span,
            ClassSetItem::Bracketed { ref span } => span,
            ClassSetItem::Union { ref span } => span,
        }
    }
}

#[test]
fn test_class_set_item_span_range() {
    let range_span = Span { start: 1, end: 5 };
    let class_set_item = ClassSetItem::Range { span: range_span };

    let result = class_set_item.span();
    
    assert_eq!(result.start, 1);
    assert_eq!(result.end, 5);
}

#[test]
fn test_class_set_item_span_empty() {
    let empty_span = Span { start: 0, end: 0 };
    let class_set_item = ClassSetItem::Empty(empty_span);

    let result = class_set_item.span();

    assert_eq!(result.start, 0);
    assert_eq!(result.end, 0);
}

#[test]
fn test_class_set_item_span_literal() {
    let literal_span = Span { start: 2, end: 3 };
    let class_set_item = ClassSetItem::Literal { span: literal_span };

    let result = class_set_item.span();

    assert_eq!(result.start, 2);
    assert_eq!(result.end, 3);
}

