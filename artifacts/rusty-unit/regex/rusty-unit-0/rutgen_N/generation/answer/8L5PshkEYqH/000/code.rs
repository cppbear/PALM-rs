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
    pub fn new(asts: Vec<Ast>, span: Span) -> Self {
        Self { asts, span }
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
    let alternation = Alternation::new(vec![], span);
    assert!(matches!(alternation.into_ast(), Ast::Empty(_)));
}

#[test]
fn test_into_ast_single() {
    let span = Span;
    let ast = Ast::Empty(span);
    let alternation = Alternation::new(vec![ast], span);
    assert!(matches!(alternation.into_ast(), Ast::Empty(_)));
}

#[test]
fn test_into_ast_multiple() {
    let span = Span;
    let ast1 = Ast::Empty(span);
    let ast2 = Ast::Empty(span);
    let alternation = Alternation::new(vec![ast1.clone(), ast2.clone()], span);
    if let Ast::Alternation(result) = alternation.into_ast() {
        assert_eq!(result.asts.len(), 2);
    } else {
        panic!("Expected to get an Ast::Alternation");
    }
}

