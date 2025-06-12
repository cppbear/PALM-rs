// Answer 0

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(Debug)]
enum Class {
    Perl(PerlClass),
    Unicode(UnicodeClass),
    Bracketed(BracketedClass),
}

#[derive(Debug)]
struct PerlClass {
    span: Span,
}

#[derive(Debug)]
struct UnicodeClass {
    span: Span,
}

#[derive(Debug)]
struct BracketedClass {
    span: Span,
}

impl Class {
    pub fn span(&self) -> &Span {
        match *self {
            Class::Perl(ref x) => &x.span,
            Class::Unicode(ref x) => &x.span,
            Class::Bracketed(ref x) => &x.span,
        }
    }
}

#[test]
fn test_span_perl_class() {
    let perl_class = PerlClass { span: Span { start: 1, end: 5 } };
    let class = Class::Perl(perl_class);
    let span = class.span();
    assert_eq!(span.start, 1);
    assert_eq!(span.end, 5);
}

#[test]
fn test_span_unicode_class() {
    let unicode_class = UnicodeClass { span: Span { start: 2, end: 6 } };
    let class = Class::Unicode(unicode_class);
    let span = class.span();
    assert_eq!(span.start, 2);
    assert_eq!(span.end, 6);
}

#[test]
fn test_span_bracketed_class() {
    let bracketed_class = BracketedClass { span: Span { start: 3, end: 7 } };
    let class = Class::Bracketed(bracketed_class);
    let span = class.span();
    assert_eq!(span.start, 3);
    assert_eq!(span.end, 7);
}

