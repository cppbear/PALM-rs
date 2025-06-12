// Answer 0

#[test]
fn test_span_perl_valid() {
    let span = Span { start: Position(100), end: Position(200) };
    let perl_class = ClassPerl { span, kind: ClassPerlKind::Digit, negated: false };
    let primitive = Primitive::Perl(perl_class);
    primitive.span();
}

#[test]
fn test_span_perl_edge_case_start_zero() {
    let span = Span { start: Position(0), end: Position(500) };
    let perl_class = ClassPerl { span, kind: ClassPerlKind::Word, negated: false };
    let primitive = Primitive::Perl(perl_class);
    primitive.span();
}

#[test]
fn test_span_perl_edge_case_end_2000() {
    let span = Span { start: Position(300), end: Position(2000) };
    let perl_class = ClassPerl { span, kind: ClassPerlKind::Space, negated: true };
    let primitive = Primitive::Perl(perl_class);
    primitive.span();
}

#[test]
fn test_span_perl_large_range() {
    let span = Span { start: Position(850), end: Position(1500) };
    let perl_class = ClassPerl { span, kind: ClassPerlKind::Punctuation, negated: false };
    let primitive = Primitive::Perl(perl_class);
    primitive.span();
}

#[test]
fn test_span_perl_maximal_end() {
    let span = Span { start: Position(900), end: Position(2000) };
    let perl_class = ClassPerl { span, kind: ClassPerlKind::Unicode, negated: true };
    let primitive = Primitive::Perl(perl_class);
    primitive.span();
}

