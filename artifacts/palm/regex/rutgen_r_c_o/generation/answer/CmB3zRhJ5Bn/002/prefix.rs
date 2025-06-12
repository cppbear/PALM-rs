// Answer 0

#[test]
fn test_into_ast_perl_class() {
    let span = Span { start: 0, end: 10 };
    let class_perl = ClassPerl { span, kind: ClassPerlKind::Digit, negated: false };
    let primitive = Primitive::Perl(class_perl);
    let _ast = primitive.into_ast();
}

#[test]
fn test_into_ast_perl_class_negated() {
    let span = Span { start: 5, end: 15 };
    let class_perl = ClassPerl { span, kind: ClassPerlKind::Word, negated: true };
    let primitive = Primitive::Perl(class_perl);
    let _ast = primitive.into_ast();
}

#[test]
fn test_into_ast_perl_class_different_span() {
    let span = Span { start: 1, end: 20 };
    let class_perl = ClassPerl { span, kind: ClassPerlKind::Space, negated: false };
    let primitive = Primitive::Perl(class_perl);
    let _ast = primitive.into_ast();
}

