// Answer 0

#[test]
fn test_visit_post_with_concat() {
    use ast::{Ast, Concat};
    let span = Span::new(0, 1);
    let concat_ast = Concat {
        span,
        expressions: vec![],
    };
    let ast_input = Ast::Concat(concat_ast);
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    writer.visit_post(&ast_input);
}

#[test]
fn test_visit_post_with_empty_concat() {
    use ast::{Ast, Concat};
    let span = Span::new(0, 0);
    let concat_ast = Concat {
        span,
        expressions: vec![],
    };
    let ast_input = Ast::Concat(concat_ast);
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    writer.visit_post(&ast_input);
}

#[test]
fn test_visit_post_with_single_expression_concat() {
    use ast::{Ast, Concat, Literal, Span};
    let span = Span::new(0, 1);
    let literal = Literal {
        span,
        kind: LiteralKind::Verbatim,
        c: 'a',
    };
    let concat_ast = Concat {
        span: Span::new(0, 1),
        expressions: vec![Ast::Literal(literal)],
    };
    let ast_input = Ast::Concat(concat_ast);
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    writer.visit_post(&ast_input);
}

