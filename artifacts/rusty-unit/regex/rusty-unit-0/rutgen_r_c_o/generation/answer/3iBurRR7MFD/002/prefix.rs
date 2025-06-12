// Answer 0

#[test]
fn test_perl_class_span() {
    let span = Span { start: 0, end: 10 };
    let perl_class = Class::Perl(ClassPerl { span, kind: ClassPerlKind::Digit, negated: false });
    let _ = perl_class.span();
}

#[test]
fn test_perl_class_span_negated() {
    let span = Span { start: 1, end: 5 };
    let perl_class = Class::Perl(ClassPerl { span, kind: ClassPerlKind::Word, negated: true });
    let _ = perl_class.span();
}

#[test]
fn test_perl_class_span_large_values() {
    let span = Span { start: 1000, end: 2000 };
    let perl_class = Class::Perl(ClassPerl { span, kind: ClassPerlKind::Whitespace, negated: false });
    let _ = perl_class.span();
}

#[test]
fn test_perl_class_span_edge_case() {
    let span = Span { start: 0, end: 1 };
    let perl_class = Class::Perl(ClassPerl { span, kind: ClassPerlKind::NonWord, negated: true });
    let _ = perl_class.span();
}

