// Answer 0

#[test]
fn test_visit_pre_with_class_perl() {
    let pattern = ".*";
    let span = Span { start: 0, end: 2 };
    let ast = Ast::Class(ast::Class::Perl(span.clone()));

    let parser_i = ParserI { parser: Parser { nest_limit: 2, ..Default::default() }, pattern: pattern };
    let mut nest_limiter = NestLimiter::new(&parser_i);
    nest_limiter.visit_pre(&ast).unwrap();
}

#[test]
fn test_visit_pre_with_class_unicode() {
    let pattern = ".*";
    let span = Span { start: 0, end: 2 };
    let ast = Ast::Class(ast::Class::Unicode(span.clone()));

    let parser_i = ParserI { parser: Parser { nest_limit: 2, ..Default::default() }, pattern: pattern };
    let mut nest_limiter = NestLimiter::new(&parser_i);
    nest_limiter.visit_pre(&ast).unwrap();
}

#[test]
fn test_visit_pre_with_class_bracketed() {
    let pattern = ".*";
    let span = Span { start: 0, end: 2 };
    let bracketed_class = ClassBracketed { span: span.clone(), negated: false, kind: ClassSet::Normal };
    let ast = Ast::Class(ast::Class::Bracketed(bracketed_class));

    let parser_i = ParserI { parser: Parser { nest_limit: 2, ..Default::default() }, pattern: pattern };
    let mut nest_limiter = NestLimiter::new(&parser_i);
    nest_limiter.visit_pre(&ast).unwrap();
}

#[test]
fn test_visit_pre_with_literal() {
    let pattern = "abc";
    let span = Span { start: 0, end: 3 };
    let ast = Ast::Literal(Literal { span: span.clone(), value: 'a' });

    let parser_i = ParserI { parser: Parser { nest_limit: 2, ..Default::default() }, pattern: pattern };
    let mut nest_limiter = NestLimiter::new(&parser_i);
    nest_limiter.visit_pre(&ast).unwrap();
}

#[test]
fn test_visit_pre_with_depth_limit() {
    let pattern = ".*";
    let span = Span { start: 0, end: 2 };
    let ast = Ast::Class(ast::Class::Bracketed(ClassBracketed { span: span.clone(), negated: false, kind: ClassSet::Normal }));

    let parser_i = ParserI { parser: Parser { nest_limit: 1, ..Default::default() }, pattern: pattern };
    let mut nest_limiter = NestLimiter::new(&parser_i);
    nest_limiter.visit_pre(&ast).unwrap();
}

