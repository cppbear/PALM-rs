// Answer 0

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(Debug)]
enum Ast {
    Empty(Span),
    Flags { span: Span },
    Literal { span: Span },
    Dot(Span),
    Assertion { span: Span },
    Class { spans: Vec<Span> },
    Repetition { span: Span },
    Group { span: Span },
    Alternation { span: Span },
    Concat { span: Span },
}

impl Ast {
    pub fn span(&self) -> &Span {
        match *self {
            Ast::Empty(ref span) => span,
            Ast::Flags(ref x) => &x.span,
            Ast::Literal(ref x) => &x.span,
            Ast::Dot(ref span) => span,
            Ast::Assertion(ref x) => &x.span,
            Ast::Class(ref x) => &x.spans[0], // assuming span() returns first for simplification
            Ast::Repetition(ref x) => &x.span,
            Ast::Group(ref x) => &x.span,
            Ast::Alternation(ref x) => &x.span,
            Ast::Concat(ref x) => &x.span,
        }
    }
}

#[test]
fn test_ast_empty_span() {
    let ast = Ast::Empty(Span { start: 0, end: 0 });
    let span = ast.span();
    assert_eq!(span.start, 0);
    assert_eq!(span.end, 0);
}

#[test]
fn test_ast_flags_span() {
    let ast = Ast::Flags { span: Span { start: 1, end: 5 } };
    let span = ast.span();
    assert_eq!(span.start, 1);
    assert_eq!(span.end, 5);
}

#[test]
fn test_ast_literal_span() {
    let ast = Ast::Literal { span: Span { start: 2, end: 3 } };
    let span = ast.span();
    assert_eq!(span.start, 2);
    assert_eq!(span.end, 3);
}

#[test]
fn test_ast_dot_span() {
    let ast = Ast::Dot(Span { start: 4, end: 5 });
    let span = ast.span();
    assert_eq!(span.start, 4);
    assert_eq!(span.end, 5);
}

#[test]
fn test_ast_assertion_span() {
    let ast = Ast::Assertion { span: Span { start: 6, end: 7 } };
    let span = ast.span();
    assert_eq!(span.start, 6);
    assert_eq!(span.end, 7);
}

#[test]
fn test_ast_class_span() {
    let ast = Ast::Class { spans: vec![Span { start: 8, end: 12 }] };
    let span = ast.span();
    assert_eq!(span.start, 8);
    assert_eq!(span.end, 12);
}

#[test]
fn test_ast_repetition_span() {
    let ast = Ast::Repetition { span: Span { start: 13, end: 14 } };
    let span = ast.span();
    assert_eq!(span.start, 13);
    assert_eq!(span.end, 14);
}

#[test]
fn test_ast_group_span() {
    let ast = Ast::Group { span: Span { start: 15, end: 16 } };
    let span = ast.span();
    assert_eq!(span.start, 15);
    assert_eq!(span.end, 16);
}

#[test]
fn test_ast_alternation_span() {
    let ast = Ast::Alternation { span: Span { start: 17, end: 18 } };
    let span = ast.span();
    assert_eq!(span.start, 17);
    assert_eq!(span.end, 18);
}

#[test]
fn test_ast_concat_span() {
    let ast = Ast::Concat { span: Span { start: 19, end: 20 } };
    let span = ast.span();
    assert_eq!(span.start, 19);
    assert_eq!(span.end, 20);
}

