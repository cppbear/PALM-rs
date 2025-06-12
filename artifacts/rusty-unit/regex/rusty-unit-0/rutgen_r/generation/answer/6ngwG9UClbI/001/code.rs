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
    Class { span: Span },
    Repetition { span: Span },
    Group { span: Span },
    Alternation { span: Span },
    Concat(Box<Ast>),
}

impl Ast {
    pub fn span(&self) -> &Span {
        match *self {
            Ast::Empty(ref span) => span,
            Ast::Flags(ref x) => &x.span,
            Ast::Literal(ref x) => &x.span,
            Ast::Dot(ref span) => span,
            Ast::Assertion(ref x) => &x.span,
            Ast::Class { span } => &span,
            Ast::Repetition { span } => &span,
            Ast::Group { span } => &span,
            Ast::Alternation { span } => &span,
            Ast::Concat(ref x) => &x.span,
        }
    }
}

#[test]
fn test_concat_span() {
    let span1 = Span { start: 0, end: 5 };
    let span2 = Span { start: 5, end: 10 };
    let concat_ast = Ast::Concat(Box::new(Ast::Literal { span: span1 }));
    let result = concat_ast.span();
    assert_eq!(result.start, 0);
    assert_eq!(result.end, 5);
}

#[test]
fn test_empty_span() {
    let empty_ast = Ast::Empty(Span { start: 0, end: 0 });
    let result = empty_ast.span();
    assert_eq!(result.start, 0);
    assert_eq!(result.end, 0);
}

#[test]
fn test_flags_span() {
    let flags_ast = Ast::Flags { span: Span { start: 1, end: 4 } };
    let result = flags_ast.span();
    assert_eq!(result.start, 1);
    assert_eq!(result.end, 4);
}

#[test]
fn test_dot_span() {
    let dot_ast = Ast::Dot(Span { start: 2, end: 3 });
    let result = dot_ast.span();
    assert_eq!(result.start, 2);
    assert_eq!(result.end, 3);
}

#[test]
fn test_assertion_span() {
    let assertion_ast = Ast::Assertion { span: Span { start: 1, end: 1 } };
    let result = assertion_ast.span();
    assert_eq!(result.start, 1);
    assert_eq!(result.end, 1);
}

#[test]
fn test_class_span() {
    let class_ast = Ast::Class { span: Span { start: 0, end: 2 } };
    let result = class_ast.span();
    assert_eq!(result.start, 0);
    assert_eq!(result.end, 2);
}

#[test]
fn test_repetition_span() {
    let repetition_ast = Ast::Repetition { span: Span { start: 3, end: 6 } };
    let result = repetition_ast.span();
    assert_eq!(result.start, 3);
    assert_eq!(result.end, 6);
}

#[test]
fn test_group_span() {
    let group_ast = Ast::Group { span: Span { start: 4, end: 8 } };
    let result = group_ast.span();
    assert_eq!(result.start, 4);
    assert_eq!(result.end, 8);
}

#[test]
fn test_alternation_span() {
    let alternation_ast = Ast::Alternation { span: Span { start: 2, end: 5 } };
    let result = alternation_ast.span();
    assert_eq!(result.start, 2);
    assert_eq!(result.end, 5);
}

