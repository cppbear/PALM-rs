// Answer 0

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(Debug)]
enum Ast {
    Empty(Span),
    Concat(Concatenation),
    // Assuming a placeholder for a potential AST node
}

#[derive(Debug)]
struct Concatenation {
    asts: Vec<Ast>,
    span: Span,
}

impl Concatenation {
    pub fn into_ast(mut self) -> Ast {
        match self.asts.len() {
            0 => Ast::Empty(self.span),
            1 => self.asts.pop().unwrap(),
            _ => Ast::Concat(self),
        }
    }
}

#[test]
fn test_into_ast_empty() {
    let span = Span { start: 0, end: 0 };
    let concat = Concatenation { asts: vec![], span };
    if let Ast::Empty(_) = concat.into_ast() {
        // Test passes
    } else {
        panic!("Expected Ast::Empty");
    }
}

#[test]
fn test_into_ast_single() {
    let span = Span { start: 0, end: 1 };
    let ast_node = Ast::Empty(span); // Placeholder for a real AST node
    let concat = Concatenation { asts: vec![ast_node], span };
    if let Ast::Empty(_) = concat.into_ast() {
        panic!("Expected single AST to be returned");
    }
}

#[test]
fn test_into_ast_concat() {
    let span = Span { start: 0, end: 2 };
    let ast_node1 = Ast::Empty(span); // Placeholder for first AST node
    let ast_node2 = Ast::Empty(span); // Placeholder for second AST node
    let concat = Concatenation { asts: vec![ast_node1, ast_node2], span };
    if let Ast::Concat(_) = concat.into_ast() {
        // Test passes
    } else {
        panic!("Expected Ast::Concat for multiple ASTs");
    }
}

