// Answer 0

#[test]
fn test_into_ast_literal() {
    let span = Span { start: Position(0), end: Position(1) };
    let lit = Literal { span, kind: LiteralKind::Unicode, c: 'a' };
    let primitive = Primitive::Literal(lit.clone());
    if let Ast::Literal(l) = primitive.into_ast() {
        assert_eq!(l, lit);
    } else {
        panic!("Expected Ast::Literal variant");
    }
}

#[test]
fn test_into_ast_assertion() {
    let span = Span { start: Position(0), end: Position(1) };
    let assertion = Assertion { span, kind: AssertionKind::StartOfLine };
    let primitive = Primitive::Assertion(assertion.clone());
    if let Ast::Assertion(a) = primitive.into_ast() {
        assert_eq!(a, assertion);
    } else {
        panic!("Expected Ast::Assertion variant");
    }
}

#[test]
fn test_into_ast_dot() {
    let span = Span { start: Position(0), end: Position(1) };
    let primitive = Primitive::Dot(span.clone());
    if let Ast::Dot(s) = primitive.into_ast() {
        assert_eq!(s, span);
    } else {
        panic!("Expected Ast::Dot variant");
    }
}

#[test]
fn test_into_ast_perl() {
    let span = Span { start: Position(0), end: Position(1) };
    let perl_class = ClassPerl { span, kind: ClassPerlKind::Digit, negated: false };
    let primitive = Primitive::Perl(perl_class.clone());
    if let Ast::Class(ast::Class::Perl(cls)) = primitive.into_ast() {
        assert_eq!(cls, perl_class);
    } else {
        panic!("Expected Ast::Class variant for Perl class");
    }
}

#[test]
fn test_into_ast_unicode() {
    let span = Span { start: Position(0), end: Position(1) };
    let unicode_class = ClassUnicode { span, negated: false, kind: ClassUnicodeKind::Letter };
    let primitive = Primitive::Unicode(unicode_class.clone());
    if let Ast::Class(ast::Class::Unicode(cls)) = primitive.into_ast() {
        assert_eq!(cls, unicode_class);
    } else {
        panic!("Expected Ast::Class variant for Unicode class");
    }
}

