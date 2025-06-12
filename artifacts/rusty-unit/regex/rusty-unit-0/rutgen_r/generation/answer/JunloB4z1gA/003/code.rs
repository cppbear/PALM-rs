// Answer 0

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct Perl {
    span: Span,
}

enum ClassSetItem {
    Empty(Span),
    Literal(Span),
    Range(Span),
    Ascii(Span),
    Perl(Perl),
    Unicode(Span),
    Bracketed(Span),
    Union(Span),
}

impl ClassSetItem {
    pub fn span(&self) -> &Span {
        match *self {
            ClassSetItem::Empty(ref span) => span,
            ClassSetItem::Literal(ref x) => &x,
            ClassSetItem::Range(ref x) => &x,
            ClassSetItem::Ascii(ref x) => &x,
            ClassSetItem::Perl(ref x) => &x.span,
            ClassSetItem::Unicode(ref x) => &x,
            ClassSetItem::Bracketed(ref x) => &x,
            ClassSetItem::Union(ref x) => &x,
        }
    }
}

#[test]
fn test_class_set_item_span_perl() {
    let perl_span = Span { start: 10, end: 20 };
    let perl_item = ClassSetItem::Perl(Perl { span: perl_span });

    let result = perl_item.span();
    
    assert_eq!(result.start, 10);
    assert_eq!(result.end, 20);
}

