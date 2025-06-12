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
fn test_unicode_span() {
    let span = Span { start: 1, end: 5 };
    let item = ClassSetItem::Unicode { span };
    assert_eq!(item.span(), &Span { start: 1, end: 5 });
}

#[test]
fn test_empty_span() {
    let span = Span { start: 0, end: 0 };
    let item = ClassSetItem::Empty(span);
    assert_eq!(item.span(), &Span { start: 0, end: 0 });
}

#[test]
fn test_literal_span() {
    let span = Span { start: 2, end: 3 };
    let item = ClassSetItem::Literal { span };
    assert_eq!(item.span(), &Span { start: 2, end: 3 });
}

#[test]
fn test_range_span() {
    let span = Span { start: 4, end: 6 };
    let item = ClassSetItem::Range { span };
    assert_eq!(item.span(), &Span { start: 4, end: 6 });
}

#[test]
fn test_ascii_span() {
    let span = Span { start: 0, end: 1 };
    let item = ClassSetItem::Ascii { span };
    assert_eq!(item.span(), &Span { start: 0, end: 1 });
}

#[test]
fn test_perl_span() {
    let span = Span { start: 3, end: 7 };
    let item = ClassSetItem::Perl { span };
    assert_eq!(item.span(), &Span { start: 3, end: 7 });
}

#[test]
fn test_bracketed_span() {
    let span = Span { start: 5, end: 8 };
    let item = ClassSetItem::Bracketed { span };
    assert_eq!(item.span(), &Span { start: 5, end: 8 });
}

#[test]
fn test_union_span() {
    let span = Span { start: 1, end: 9 };
    let item = ClassSetItem::Union { span };
    assert_eq!(item.span(), &Span { start: 1, end: 9 });
}

