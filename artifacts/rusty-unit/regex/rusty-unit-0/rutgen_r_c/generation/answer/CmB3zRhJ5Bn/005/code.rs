// Answer 0

#[test]
fn test_into_ast_literal() {
    let span = Span { start: Position(0), end: Position(1) };
    let lit = Literal { span: span.clone(), kind: LiteralKind::Byte, c: 'a' };
    let primitive = Primitive::Literal(lit.clone());
    
    assert_eq!(primitive.into_ast(), Ast::Literal(lit));
}

#[test]
fn test_into_ast_assertion() {
    let span = Span { start: Position(0), end: Position(1) };
    let assertion = Assertion { span: span.clone(), kind: AssertionKind::WordBoundary };
    let primitive = Primitive::Assertion(assertion.clone());
    
    assert_eq!(primitive.into_ast(), Ast::Assertion(assertion));
}

#[test]
fn test_into_ast_dot() {
    let span = Span { start: Position(0), end: Position(1) };
    let primitive = Primitive::Dot(span.clone());
    
    assert_eq!(primitive.into_ast(), Ast::Dot(span));
}

#[test]
fn test_into_ast_perl() {
    let span = Span { start: Position(0), end: Position(1) };
    let perl_class = ClassPerl { span: span.clone(), kind: ClassPerlKind::Digit, negated: false };
    let primitive = Primitive::Perl(perl_class.clone());
    
    assert_eq!(primitive.into_ast(), Ast::Class(Class::Perl(perl_class)));
}

#[test]
fn test_into_ast_unicode() {
    let span = Span { start: Position(0), end: Position(1) };
    let unicode_class = ClassUnicode { span: span.clone(), negated: false, kind: ClassUnicodeKind::Letter };
    let primitive = Primitive::Unicode(unicode_class.clone());
    
    assert_eq!(primitive.into_ast(), Ast::Class(Class::Unicode(unicode_class)));
}

