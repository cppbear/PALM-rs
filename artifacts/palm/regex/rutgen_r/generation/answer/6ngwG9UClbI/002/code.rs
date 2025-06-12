// Answer 0

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct Flags {
    span: Span,
}

#[derive(Debug)]
struct Literal {
    span: Span,
}

#[derive(Debug)]
struct Dot {
    span: Span,
}

#[derive(Debug)]
struct Assertion {
    span: Span,
}

#[derive(Debug)]
struct Class {
    span: Span,
}

impl Class {
    fn span(&self) -> &Span {
        &self.span
    }
}

#[derive(Debug)]
struct Repetition {
    span: Span,
}

#[derive(Debug)]
struct Group {
    span: Span,
}

#[derive(Debug)]
struct Alternation {
    span: Span,
}

impl Alternation {
    fn span(&self) -> &Span {
        &self.span
    }
}

#[derive(Debug)]
struct Concat {
    span: Span,
}

#[derive(Debug)]
enum Ast {
    Empty(Span),
    Flags(Flags),
    Literal(Literal),
    Dot(Dot),
    Assertion(Assertion),
    Class(Class),
    Repetition(Repetition),
    Group(Group),
    Alternation(Alternation),
    Concat(Concat),
}

impl Ast {
    pub fn span(&self) -> &Span {
        match *self {
            Ast::Empty(ref span) => span,
            Ast::Flags(ref x) => &x.span,
            Ast::Literal(ref x) => &x.span,
            Ast::Dot(ref span) => span,
            Ast::Assertion(ref x) => &x.span,
            Ast::Class(ref x) => x.span(),
            Ast::Repetition(ref x) => &x.span,
            Ast::Group(ref x) => &x.span,
            Ast::Alternation(ref x) => &x.span,
            Ast::Concat(ref x) => &x.span,
        }
    }
}

#[test]
fn test_alternation_span() {
    let span = Span { start: 0, end: 10 };
    let alternation = Alternation { span };
    let ast = Ast::Alternation(alternation);

    let result = ast.span();
    assert_eq!(result.start, 0);
    assert_eq!(result.end, 10);
}

