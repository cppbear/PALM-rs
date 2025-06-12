// Answer 0

#[test]
fn test_into_ast_perl_class() {
    let span = Span { start: Position(0), end: Position(5) };
    let kind = ClassPerlKind::Digit; // Assume ClassPerlKind has a variant "Digit"
    let perl_class = ClassPerl { span, kind, negated: false };
    let primitive = Primitive::Perl(perl_class.clone());

    if let Ast::Class(ast::Class::Perl(cls)) = primitive.into_ast() {
        assert_eq!(cls, perl_class);
    } else {
        panic!("Expected Ast::Class(ast::Class::Perl(cls)), but got a different variant.");
    }
}

#[test]
fn test_into_ast_unicode_class() {
    let span = Span { start: Position(0), end: Position(5) };
    let kind = ClassUnicodeKind::Letter; // Assume ClassUnicodeKind has a variant "Letter"
    let unicode_class = ClassUnicode { span, negated: false, kind };
    let primitive = Primitive::Unicode(unicode_class.clone());

    if let Ast::Class(ast::Class::Unicode(cls)) = primitive.into_ast() {
        assert_eq!(cls, unicode_class);
    } else {
        panic!("Expected Ast::Class(ast::Class::Unicode(cls)), but got a different variant.");
    }
}

#[test]
fn test_into_ast_literal_class() {
    let span = Span { start: Position(0), end: Position(1) };
    let literal = Literal { span, kind: LiteralKind::Byte, c: 'a' }; // Assume LiteralKind has a variant "Byte"
    let primitive = Primitive::Literal(literal.clone());

    if let Ast::Literal(lit) = primitive.into_ast() {
        assert_eq!(lit, literal);
    } else {
        panic!("Expected Ast::Literal(lit), but got a different variant.");
    }
}

#[test]
fn test_into_ast_assertion_class() {
    let span = Span { start: Position(2), end: Position(3) };
    let assertion = Assertion { span, kind: AssertionKind::WordBoundary }; // Assume AssertionKind has a variant "WordBoundary"
    let primitive = Primitive::Assertion(assertion.clone());

    if let Ast::Assertion(assert) = primitive.into_ast() {
        assert_eq!(assert, assertion);
    } else {
        panic!("Expected Ast::Assertion(assert), but got a different variant.");
    }
}

#[test]
fn test_into_ast_dot_class() {
    let span = Span { start: Position(4), end: Position(5) };
    let primitive = Primitive::Dot(span.clone());

    if let Ast::Dot(s) = primitive.into_ast() {
        assert_eq!(s, span);
    } else {
        panic!("Expected Ast::Dot(span), but got a different variant.");
    }
}

