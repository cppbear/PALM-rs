// Answer 0

#[test]
fn test_into_ast_literal() {
    let span = Span { start: Position(0), end: Position(1) };
    let literal = Literal { span, kind: LiteralKind::Byte, c: 'a' };
    let primitive = Primitive::Literal(literal.clone());
    
    if let Ast::Literal(ast_literal) = primitive.into_ast() {
        assert_eq!(ast_literal, literal);
    } else {
        panic!("Expected Ast::Literal");
    }
}

#[test]
fn test_into_ast_assertion() {
    let span = Span { start: Position(0), end: Position(1) };
    let assertion = Assertion { span, kind: AssertionKind::StartOfLine };
    let primitive = Primitive::Assertion(assertion.clone());
    
    if let Ast::Assertion(ast_assertion) = primitive.into_ast() {
        assert_eq!(ast_assertion, assertion);
    } else {
        panic!("Expected Ast::Assertion");
    }
}

#[test]
fn test_into_ast_dot() {
    let span = Span { start: Position(0), end: Position(1) };
    let primitive = Primitive::Dot(span.clone());
    
    if let Ast::Dot(ast_dot) = primitive.into_ast() {
        assert_eq!(ast_dot, span);
    } else {
        panic!("Expected Ast::Dot");
    }
}

#[test]
fn test_into_ast_perl_class() {
    let span = Span { start: Position(0), end: Position(1) };
    let perl_class = ClassPerl { span, kind: ClassPerlKind::Digit, negated: false };
    let primitive = Primitive::Perl(perl_class.clone());
    
    if let Ast::Class(ast_class) = primitive.into_ast() {
        if let ast::Class::Perl(ast_perl_class) = ast_class {
            assert_eq!(ast_perl_class, perl_class);
        } else {
            panic!("Expected ast::Class::Perl");
        }
    } else {
        panic!("Expected Ast::Class");
    }
}

#[test]
fn test_into_ast_unicode_class() {
    let span = Span { start: Position(0), end: Position(1) };
    let unicode_class = ClassUnicode { span, negated: false, kind: ClassUnicodeKind::Letter };
    let primitive = Primitive::Unicode(unicode_class.clone());
    
    if let Ast::Class(ast_class) = primitive.into_ast() {
        if let ast::Class::Unicode(ast_unicode_class) = ast_class {
            assert_eq!(ast_unicode_class, unicode_class);
        } else {
            panic!("Expected ast::Class::Unicode");
        }
    } else {
        panic!("Expected Ast::Class");
    }
}

