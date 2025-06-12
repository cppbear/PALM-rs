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
fn test_empty_class_set_item_span() {
    let span = Span { start: 0, end: 1 };
    let item = ClassSetItem::Empty(span);
    let result = item.span();
    assert_eq!(result.start, 0);
    assert_eq!(result.end, 1);
}

#[test]
fn test_literal_class_set_item_span() {
    let span = Span { start: 2, end: 3 };
    let literal = Literal { span };
    let item = ClassSetItem::Literal(literal);
    let result = item.span();
    assert_eq!(result.start, 2);
    assert_eq!(result.end, 3);
}

#[test]
fn test_range_class_set_item_span() {
    let span = Span { start: 4, end: 5 };
    let range = Range { span };
    let item = ClassSetItem::Range(range);
    let result = item.span();
    assert_eq!(result.start, 4);
    assert_eq!(result.end, 5);
}

#[test]
fn test_ascii_class_set_item_span() {
    let span = Span { start: 6, end: 7 };
    let ascii = Ascii { span };
    let item = ClassSetItem::Ascii(ascii);
    let result = item.span();
    assert_eq!(result.start, 6);
    assert_eq!(result.end, 7);
}

#[test]
fn test_perl_class_set_item_span() {
    let span = Span { start: 8, end: 9 };
    let perl = Perl { span };
    let item = ClassSetItem::Perl(perl);
    let result = item.span();
    assert_eq!(result.start, 8);
    assert_eq!(result.end, 9);
}

#[test]
fn test_unicode_class_set_item_span() {
    let span = Span { start: 10, end: 11 };
    let unicode = Unicode { span };
    let item = ClassSetItem::Unicode(unicode);
    let result = item.span();
    assert_eq!(result.start, 10);
    assert_eq!(result.end, 11);
}

#[test]
fn test_bracketed_class_set_item_span() {
    let span = Span { start: 12, end: 13 };
    let bracketed = Bracketed { span };
    let item = ClassSetItem::Bracketed(bracketed);
    let result = item.span();
    assert_eq!(result.start, 12);
    assert_eq!(result.end, 13);
}

#[test]
fn test_union_class_set_item_span() {
    let span = Span { start: 14, end: 15 };
    let union = Union { span };
    let item = ClassSetItem::Union(union);
    let result = item.span();
    assert_eq!(result.start, 14);
    assert_eq!(result.end, 15);
}

