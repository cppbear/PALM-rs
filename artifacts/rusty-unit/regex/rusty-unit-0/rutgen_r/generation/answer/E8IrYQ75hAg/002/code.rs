// Answer 0

#[derive(Debug)]
struct Ast {
    // Assuming span is a simple string for demonstration
    span: String,
}

impl Ast {
    fn empty(span: String) -> Self {
        Ast { span }
    }
}

struct Concat {
    asts: Vec<Ast>,
    span: String,
}

impl Concat {
    fn new(asts: Vec<Ast>, span: String) -> Self {
        Concat { asts, span }
    }

    pub fn into_ast(mut self) -> Ast {
        match self.asts.len() {
            0 => Ast::empty(self.span),
            1 => self.asts.pop().unwrap(),
            _ => Ast::Concat(self),
        }
    }
}

#[test]
fn test_into_ast_empty_case() {
    let concat = Concat::new(vec![], "span".to_string());
    let result = concat.into_ast();
    if let Ast::Empty(_) = result {
        // Test passed
    } else {
        panic!("Expected Ast::Empty but got {:#?}", result);
    }
}

#[test]
fn test_into_ast_single_ast_case() {
    let ast = Ast { span: "single".to_string() };
    let concat = Concat::new(vec![ast], "span".to_string());
    let result = concat.into_ast();
    // Check that the expected AST is returned
    if let Ast { span } = result {
        assert_eq!(span, "single");
    } else {
        panic!("Expected single AST but got {:#?}", result);
    }
}

#[test]
fn test_into_ast_multiple_asts_case() {
    let ast1 = Ast { span: "first".to_string() };
    let ast2 = Ast { span: "second".to_string() };
    let concat = Concat::new(vec![ast1, ast2], "span".to_string());
    let result = concat.into_ast();
    // Check the type returned is a concatenation
    match result {
        Ast::Concat(_) => {} // Test passed
        _ => panic!("Expected Ast::Concat but got {:#?}", result),
    }
}

