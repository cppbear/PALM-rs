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
struct Literal { span: Span }
#[derive(Debug)]
struct Range { span: Span }
#[derive(Debug)]
struct Ascii { span: Span }
#[derive(Debug)]
struct Perl { span: Span }
#[derive(Debug)]
struct Unicode { span: Span }
#[derive(Debug)]
struct Bracketed { span: Span }
#[derive(Debug)]
struct Union { span: Span }

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
    let span = Span { start: 0, end: 0 };
    let item = ClassSetItem::Empty(span);
    assert_eq!(item.span(), &Span { start: 0, end: 0 });
}

#[test]
fn test_literal_class_set_item_span() {
    let span = Span { start: 1, end: 1 };
    let item = ClassSetItem::Literal(Literal { span });
    assert_eq!(item.span(), &Span { start: 1, end: 1 });
}

#[test]
fn test_range_class_set_item_span() {
    let span = Span { start: 2, end: 3 };
    let item = ClassSetItem::Range(Range { span });
    assert_eq!(item.span(), &Span { start: 2, end: 3 });
}

#[test]
fn test_ascii_class_set_item_span() {
    let span = Span { start: 4, end: 4 };
    let item = ClassSetItem::Ascii(Ascii { span });
    assert_eq!(item.span(), &Span { start: 4, end: 4 });
}

#[test]
fn test_perl_class_set_item_span() {
    let span = Span { start: 5, end: 5 };
    let item = ClassSetItem::Perl(Perl { span });
    assert_eq!(item.span(), &Span { start: 5, end: 5 });
}

#[test]
fn test_unicode_class_set_item_span() {
    let span = Span { start: 6, end: 6 };
    let item = ClassSetItem::Unicode(Unicode { span });
    assert_eq!(item.span(), &Span { start: 6, end: 6 });
}

#[test]
fn test_bracketed_class_set_item_span() {
    let span = Span { start: 7, end: 7 };
    let item = ClassSetItem::Bracketed(Bracketed { span });
    assert_eq!(item.span(), &Span { start: 7, end: 7 });
}

#[test]
fn test_union_class_set_item_span() {
    let span = Span { start: 8, end: 8 };
    let item = ClassSetItem::Union(Union { span });
    assert_eq!(item.span(), &Span { start: 8, end: 8 });
}

