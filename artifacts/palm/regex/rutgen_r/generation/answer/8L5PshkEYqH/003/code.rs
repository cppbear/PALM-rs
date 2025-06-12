// Answer 0

#[derive(Debug)]
struct Span;

#[derive(Debug)]
enum Ast {
    Empty(Span),
    Alternation(Alternation),
}

#[derive(Debug)]
struct Alternation {
    asts: Vec<Ast>,
    span: Span,
}

impl Alternation {
    pub fn new(span: Span) -> Self {
        Alternation {
            asts: Vec::new(),
            span,
        }
    }

    pub fn into_ast(mut self) -> Ast {
        match self.asts.len() {
            0 => Ast::Empty(self.span),
            1 => self.asts.pop().unwrap(),
            _ => Ast::Alternation(self),
        }
    }
}

#[test]
fn test_into_ast_empty() {
    let span = Span;
    let alternation = Alternation::new(span);
    let result = alternation.into_ast();
    match result {
        Ast::Empty(_) => {}
        _ => panic!("Expected Ast::Empty but got {:?}", result),
    }
}

#[test]
fn test_into_ast_single_element() {
    let span = Span;
    let mut alternation = Alternation::new(span);
    alternation.asts.push(Ast::Empty(span)); // adding one AST
    let result = alternation.into_ast();
    match result {
        Ast::Empty(_) => panic!("Expected single AST but got Ast::Empty"),
        _ => {}
    }
}

#[test]
fn test_into_ast_multiple_elements() {
    let span = Span;
    let mut alternation = Alternation::new(span);
    alternation.asts.push(Ast::Empty(span)); // adding one AST
    alternation.asts.push(Ast::Empty(span)); // adding another AST
    let result = alternation.into_ast();
    match result {
        Ast::Alternation(_) => {}
        _ => panic!("Expected Ast::Alternation but got {:?}", result),
    }
}

