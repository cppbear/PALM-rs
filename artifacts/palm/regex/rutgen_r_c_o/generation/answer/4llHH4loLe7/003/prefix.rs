// Answer 0

#[test]
fn test_concat_with_empty_and_literal() {
    let span_empty = Span { start: 0, end: 0 };
    let span_literal = Span { start: 0, end: 1 };
    let lit = Literal { span: span_literal, kind: LiteralKind::Character, c: 'a' };
    let ast = Ast::Concat(vec![Ast::Empty(span_empty), Ast::Literal(lit)]);
    let mut translator = TranslatorI::new(&Translator::default(), "");
    translator.visit_post(&ast).unwrap();
}

#[test]
fn test_concat_with_empty_and_repetition() {
    let span_empty = Span { start: 0, end: 0 };
    let span_repetition = Span { start: 0, end: 0 };
    let lit = Literal { span: Span { start: 0, end: 1 }, kind: LiteralKind::Character, c: 'b' };
    let rep = Repetition { span: span_repetition, op: RepetitionOp::ZeroOrMore, greedy: true, ast: Box::new(Ast::Literal(lit)) };
    let ast = Ast::Concat(vec![Ast::Empty(span_empty), Ast::Repetition(rep)]);
    let mut translator = TranslatorI::new(&Translator::default(), "");
    translator.visit_post(&ast).unwrap();
}

#[test]
fn test_concat_with_only_empty() {
    let span_empty = Span { start: 0, end: 0 };
    let ast = Ast::Concat(vec![Ast::Empty(span_empty)]);
    let mut translator = TranslatorI::new(&Translator::default(), "");
    translator.visit_post(&ast).unwrap();
}

#[test]
fn test_concat_with_single_literal() {
    let span_literal = Span { start: 1, end: 2 };
    let lit = Literal { span: span_literal, kind: LiteralKind::Character, c: 'c' };
    let ast = Ast::Concat(vec![Ast::Literal(lit)]);
    let mut translator = TranslatorI::new(&Translator::default(), "");
    translator.visit_post(&ast).unwrap();
}

