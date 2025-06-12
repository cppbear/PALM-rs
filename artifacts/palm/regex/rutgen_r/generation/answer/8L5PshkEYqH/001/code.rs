// Answer 0

#[derive(Debug)]
struct Ast {
    span: usize,
    asts: Vec<Ast>,
}

impl Ast {
    fn Empty(span: usize) -> Ast {
        Ast { span, asts: Vec::new() }
    }

    fn Alternation(self) -> Ast {
        Ast { span: self.span, asts: self.asts }
    }
}

#[derive(Debug)]
struct Alternation {
    asts: Vec<Ast>,
    span: usize,
}

impl Alternation {
    fn new(asts: Vec<Ast>, span: usize) -> Self {
        Alternation { asts, span }
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
    let alternation = Alternation::new(Vec::new(), 0);
    let result = alternation.into_ast();
    match result {
        Ast { span, asts } if span == 0 && asts.is_empty() => (),
        _ => panic!("Expected Ast::Empty(0)"),
    }
}

#[test]
fn test_into_ast_single() {
    let single_ast = Ast { span: 1, asts: Vec::new() };
    let alternation = Alternation::new(vec![single_ast], 0);
    let result = alternation.into_ast();
    match result {
        Ast { span, asts } if span == 1 && asts.is_empty() => (),
        _ => panic!("Expected Ast with span 1 and empty asts"),
    }
}

#[test]
fn test_into_ast_multiple() {
    let ast1 = Ast { span: 1, asts: Vec::new() };
    let ast2 = Ast { span: 2, asts: Vec::new() };
    let alternation = Alternation::new(vec![ast1, ast2], 0);
    let result = alternation.into_ast();
    match result {
        Ast { span, asts } if span == 0 && asts.len() == 2 => (),
        _ => panic!("Expected Ast::Alternation with 2 asts"),
    }
}

