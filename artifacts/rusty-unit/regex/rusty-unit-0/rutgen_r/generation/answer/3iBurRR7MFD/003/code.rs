// Answer 0

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

enum Class {
    Perl(PerlClass),
    Unicode(UnicodeClass),
    Bracketed(BracketedClass),
}

struct PerlClass {
    span: Span,
}

struct UnicodeClass {
    span: Span,
}

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
fn test_unicode_class_span() {
    let unicode_class = UnicodeClass { span: Span { start: 5, end: 10 } };
    let class = Class::Unicode(unicode_class);
    let span = class.span();
    assert_eq!(span.start, 5);
    assert_eq!(span.end, 10);
}

#[test]
fn test_perl_class_span() {
    let perl_class = PerlClass { span: Span { start: 1, end: 3 } };
    let class = Class::Perl(perl_class);
    let span = class.span();
    assert_eq!(span.start, 1);
    assert_eq!(span.end, 3);
}

#[test]
fn test_bracketed_class_span() {
    let bracketed_class = BracketedClass { span: Span { start: 0, end: 15 } };
    let class = Class::Bracketed(bracketed_class);
    let span = class.span();
    assert_eq!(span.start, 0);
    assert_eq!(span.end, 15);
}

