// Answer 0

#[test]
fn test_visit_post_ast_empty() {
    let ast = Ast::Empty(Span::default());
    let mut limiter = NestLimiter::new(&ParserI {
        parser: Parser::default(),
        pattern: "",
    });
    assert_eq!(limiter.visit_post(&ast), Ok(()));
}

#[test]
fn test_visit_post_ast_flags() {
    let ast = Ast::Flags(SetFlags::default());
    let mut limiter = NestLimiter::new(&ParserI {
        parser: Parser::default(),
        pattern: "",
    });
    assert_eq!(limiter.visit_post(&ast), Ok(()));
}

#[test]
fn test_visit_post_ast_literal() {
    let ast = Ast::Literal(Literal::default());
    let mut limiter = NestLimiter::new(&ParserI {
        parser: Parser::default(),
        pattern: "",
    });
    assert_eq!(limiter.visit_post(&ast), Ok(()));
}

#[test]
fn test_visit_post_ast_dot() {
    let ast = Ast::Dot(Span::default());
    let mut limiter = NestLimiter::new(&ParserI {
        parser: Parser::default(),
        pattern: "",
    });
    assert_eq!(limiter.visit_post(&ast), Ok(()));
}

#[test]
fn test_visit_post_ast_assertion() {
    let ast = Ast::Assertion(Assertion::default());
    let mut limiter = NestLimiter::new(&ParserI {
        parser: Parser::default(),
        pattern: "",
    });
    assert_eq!(limiter.visit_post(&ast), Ok(()));
}

#[test]
fn test_visit_post_ast_class_unicode() {
    let ast = Ast::Class(ast::Class::Unicode(Vec::new()));
    let mut limiter = NestLimiter::new(&ParserI {
        parser: Parser::default(),
        pattern: "",
    });
    assert_eq!(limiter.visit_post(&ast), Ok(()));
}

#[test]
fn test_visit_post_ast_class_perl() {
    let ast = Ast::Class(ast::Class::Perl(Vec::new()));
    let mut limiter = NestLimiter::new(&ParserI {
        parser: Parser::default(),
        pattern: "",
    });
    assert_eq!(limiter.visit_post(&ast), Ok(()));
}

#[test]
fn test_visit_post_ast_class_bracketed() {
    let ast = Ast::Class(ast::Class::Bracketed(Vec::new()));
    let mut limiter = NestLimiter::new(&ParserI {
        parser: Parser::default(),
        pattern: "",
    });
    assert_eq!(limiter.visit_post(&ast), Ok(()));
}

#[test]
fn test_visit_post_ast_repetition() {
    let ast = Ast::Repetition(Repetition::default());
    let mut limiter = NestLimiter::new(&ParserI {
        parser: Parser::default(),
        pattern: "",
    });
    assert_eq!(limiter.visit_post(&ast), Ok(()));
}

#[test]
fn test_visit_post_ast_group() {
    let ast = Ast::Group(Group::default());
    let mut limiter = NestLimiter::new(&ParserI {
        parser: Parser::default(),
        pattern: "",
    });
    assert_eq!(limiter.visit_post(&ast), Ok(()));
}

#[test]
fn test_visit_post_ast_alternation() {
    let ast = Ast::Alternation(Alternation::default());
    let mut limiter = NestLimiter::new(&ParserI {
        parser: Parser::default(),
        pattern: "",
    });
    assert_eq!(limiter.visit_post(&ast), Ok(()));
}

#[test]
fn test_visit_post_ast_concat() {
    let ast = Ast::Concat(Concat::default());
    let mut limiter = NestLimiter::new(&ParserI {
        parser: Parser::default(),
        pattern: "",
    });
    assert_eq!(limiter.visit_post(&ast), Ok(()));
}

