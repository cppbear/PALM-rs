// Answer 0

#[test]
fn test_span_perl() {
    let span = Span { start: 0, end: 5 };
    let class = Class::Perl(ClassPerl { span, kind: ClassPerlKind::Digit, negated: false });
    assert_eq!(class.span(), &span);
}

#[test]
fn test_span_unicode() {
    let span = Span { start: 5, end: 10 };
    let class = Class::Unicode(ClassUnicode { span, negated: false, kind: ClassUnicodeKind::Letter });
    assert_eq!(class.span(), &span);
}

#[test]
fn test_span_bracketed() {
    let span = Span { start: 10, end: 15 };
    let class = Class::Bracketed(ClassBracketed { span, negated: false, kind: ClassSet::Normal });
    assert_eq!(class.span(), &span);
}

