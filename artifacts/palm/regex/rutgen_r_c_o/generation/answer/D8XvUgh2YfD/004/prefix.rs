// Answer 0

#[test]
fn test_visit_pre_with_repetition_normal() {
    let span = Span { start: 0, end: 5 };
    let repetition = Repetition { span, op: RepetitionOp::Plus, greedy: true, ast: Box::new(Ast::Empty(span.clone())) };
    let ast = Ast::Repetition(repetition);

    let parser = ParserI { parser: Parser { nest_limit: 1, ..Default::default() }, pattern: "a+" };
    let mut nest_limiter = NestLimiter::new(&parser);
    let _ = nest_limiter.visit_pre(&ast);
}

#[test]
fn test_visit_pre_with_repetition_exceeds_nest_limit() {
    let span = Span { start: 0, end: 5 };
    let repetition = Repetition { span, op: RepetitionOp::Plus, greedy: true, ast: Box::new(Ast::Empty(span.clone())) };
    let ast = Ast::Repetition(repetition);

    let parser = ParserI { parser: Parser { nest_limit: 0, ..Default::default() }, pattern: "a+" };
    let mut nest_limiter = NestLimiter::new(&parser);
    let result = nest_limiter.check(&ast);
    assert!(result.is_err());
} 

#[test]
fn test_visit_pre_with_repetition_nested() {
    let span1 = Span { start: 0, end: 5 };
    let repetition1 = Repetition { span: span1.clone(), op: RepetitionOp::Plus, greedy: true, ast: Box::new(Ast::Empty(span1.clone())) };
    let ast1 = Ast::Repetition(repetition1);

    let span2 = Span { start: 6, end: 11 };
    let repetition2 = Repetition { span: span2.clone(), op: RepetitionOp::Star, greedy: false, ast: Box::new(ast1) };
    let ast2 = Ast::Repetition(repetition2);

    let parser = ParserI { parser: Parser { nest_limit: 2, ..Default::default() }, pattern: "a+?" };
    let mut nest_limiter = NestLimiter::new(&parser);
    let _ = nest_limiter.visit_pre(&ast2);
}

#[test]
fn test_visit_pre_with_repetition_context_increase_depth() {
    let span = Span { start: 0, end: 5 };
    let repetition = Repetition { span, op: RepetitionOp::Star, greedy: true, ast: Box::new(Ast::Empty(span.clone())) };
    let ast = Ast::Repetition(repetition);

    let parser = ParserI { parser: Parser { nest_limit: 1, ..Default::default() }, pattern: "a*" };
    let mut nest_limiter = NestLimiter::new(&parser);
    let _ = nest_limiter.visit_pre(&ast);
}

#[test]
fn test_visit_pre_with_repetition_empty_as_nested() {
    let span = Span { start: 0, end: 0 };
    let repetition = Repetition { span, op: RepetitionOp::Plus, greedy: false, ast: Box::new(Ast::Empty(span.clone())) };
    let ast = Ast::Repetition(repetition);

    let parser = ParserI { parser: Parser { nest_limit: 1, ..Default::default() }, pattern: "" };
    let mut nest_limiter = NestLimiter::new(&parser);
    let _ = nest_limiter.visit_pre(&ast);
}

