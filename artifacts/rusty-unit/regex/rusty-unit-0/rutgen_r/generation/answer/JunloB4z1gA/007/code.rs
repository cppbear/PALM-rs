// Answer 0

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(Debug)]
enum ClassSetItem {
    Empty(Span),
    Literal(Literal),
    Range(Range),
    Ascii(Ascii),
    Perl(Perl),
    Unicode(Unicode),
    Bracketed(Bracketed),
    Union(Union),
}

#[derive(Debug)]
struct Literal {
    span: Span,
}

#[derive(Debug)]
struct Range {
    span: Span,
}

#[derive(Debug)]
struct Ascii {
    span: Span,
}

#[derive(Debug)]
struct Perl {
    span: Span,
}

#[derive(Debug)]
struct Unicode {
    span: Span,
}

#[derive(Debug)]
struct Bracketed {
    span: Span,
}

#[derive(Debug)]
struct Union {
    span: Span,
}

impl ClassSetItem {
    pub fn span(&self) -> &Span {
        match *self {
            ClassSetItem::Empty(ref span) => span,
            ClassSetItem::Literal(ref x) => &x.span,
            ClassSetItem::Range(ref x) => &x.span,
            ClassSetItem::Ascii(ref x) => &x.span,
            ClassSetItem::Perl(ref x) => &x.span,
            ClassSetItem::Unicode(ref x) => &x.span,
            ClassSetItem::Bracketed(ref x) => &x.span,
            ClassSetItem::Union(ref x) => &x.span,
        }
    }
}

#[test]
fn test_span_literal() {
    let literal = Literal { span: Span { start: 0, end: 1 } };
    let class_set_item = ClassSetItem::Literal(literal);
    let result_span = class_set_item.span();
    assert_eq!(result_span.start, 0);
    assert_eq!(result_span.end, 1);
}

#[test]
fn test_span_empty() {
    let class_set_item = ClassSetItem::Empty(Span { start: 2, end: 3 });
    let result_span = class_set_item.span();
    assert_eq!(result_span.start, 2);
    assert_eq!(result_span.end, 3);
}

#[test]
fn test_span_range() {
    let range = Range { span: Span { start: 4, end: 5 } };
    let class_set_item = ClassSetItem::Range(range);
    let result_span = class_set_item.span();
    assert_eq!(result_span.start, 4);
    assert_eq!(result_span.end, 5);
}

#[test]
fn test_span_ascii() {
    let ascii = Ascii { span: Span { start: 6, end: 7 } };
    let class_set_item = ClassSetItem::Ascii(ascii);
    let result_span = class_set_item.span();
    assert_eq!(result_span.start, 6);
    assert_eq!(result_span.end, 7);
}

#[test]
fn test_span_perl() {
    let perl = Perl { span: Span { start: 8, end: 9 } };
    let class_set_item = ClassSetItem::Perl(perl);
    let result_span = class_set_item.span();
    assert_eq!(result_span.start, 8);
    assert_eq!(result_span.end, 9);
}

#[test]
fn test_span_unicode() {
    let unicode = Unicode { span: Span { start: 10, end: 11 } };
    let class_set_item = ClassSetItem::Unicode(unicode);
    let result_span = class_set_item.span();
    assert_eq!(result_span.start, 10);
    assert_eq!(result_span.end, 11);
}

#[test]
fn test_span_bracketed() {
    let bracketed = Bracketed { span: Span { start: 12, end: 13 } };
    let class_set_item = ClassSetItem::Bracketed(bracketed);
    let result_span = class_set_item.span();
    assert_eq!(result_span.start, 12);
    assert_eq!(result_span.end, 13);
}

#[test]
fn test_span_union() {
    let union = Union { span: Span { start: 14, end: 15 } };
    let class_set_item = ClassSetItem::Union(union);
    let result_span = class_set_item.span();
    assert_eq!(result_span.start, 14);
    assert_eq!(result_span.end, 15);
}

