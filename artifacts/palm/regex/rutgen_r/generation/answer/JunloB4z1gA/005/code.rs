// Answer 0

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct Ascii {
    span: Span,
}

#[derive(Debug)]
enum ClassSetItem {
    Empty(Span),
    Literal(Ascii),
    Range(Ascii),
    Ascii(Ascii),
    Perl(Ascii),
    Unicode(Ascii),
    Bracketed(Ascii),
    Union(Ascii),
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
fn test_span_empty() {
    let span = Span { start: 0, end: 0 };
    let item = ClassSetItem::Empty(span);
    assert_eq!(item.span(), &Span { start: 0, end: 0 });
}

#[test]
fn test_span_literal() {
    let span = Span { start: 1, end: 2 };
    let ascii = Ascii { span };
    let item = ClassSetItem::Literal(ascii);
    assert_eq!(item.span(), &Span { start: 1, end: 2 });
}

#[test]
fn test_span_range() {
    let span = Span { start: 3, end: 4 };
    let ascii = Ascii { span };
    let item = ClassSetItem::Range(ascii);
    assert_eq!(item.span(), &Span { start: 3, end: 4 });
}

#[test]
fn test_span_ascii() {
    let span = Span { start: 5, end: 6 };
    let ascii = Ascii { span };
    let item = ClassSetItem::Ascii(ascii);
    assert_eq!(item.span(), &Span { start: 5, end: 6 });
}

#[test]
fn test_span_perl() {
    let span = Span { start: 7, end: 8 };
    let ascii = Ascii { span };
    let item = ClassSetItem::Perl(ascii);
    assert_eq!(item.span(), &Span { start: 7, end: 8 });
}

#[test]
fn test_span_unicode() {
    let span = Span { start: 9, end: 10 };
    let ascii = Ascii { span };
    let item = ClassSetItem::Unicode(ascii);
    assert_eq!(item.span(), &Span { start: 9, end: 10 });
}

#[test]
fn test_span_bracketed() {
    let span = Span { start: 11, end: 12 };
    let ascii = Ascii { span };
    let item = ClassSetItem::Bracketed(ascii);
    assert_eq!(item.span(), &Span { start: 11, end: 12 });
}

#[test]
fn test_span_union() {
    let span = Span { start: 13, end: 14 };
    let ascii = Ascii { span };
    let item = ClassSetItem::Union(ascii);
    assert_eq!(item.span(), &Span { start: 13, end: 14 });
}

