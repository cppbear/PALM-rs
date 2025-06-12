// Answer 0

#[derive(Debug)]
struct Span;

#[derive(Debug)]
enum Ast {
    Empty(Span),
    Concat(ConcatAst),
}

#[derive(Debug)]
struct ConcatAst {
    asts: Vec<Ast>,
    span: Span,
}

impl ConcatAst {
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
    let span = Span;
    let concat_ast = ConcatAst {
        asts: Vec::new(),
        span,
    };
    
    let result = concat_ast.into_ast();
    
    match result {
        Ast::Empty(_) => (),
        _ => panic!("Expected Ast::Empty but got {:?}", result),
    }
}

#[test]
fn test_into_ast_single() {
    let span = Span;
    let single_ast = Ast::Empty(Span);
    
    let concat_ast = ConcatAst {
        asts: vec![single_ast],
        span,
    };
    
    let result = concat_ast.into_ast();
    
    match result {
        Ast::Empty(_) => panic!("Expected single AST but got Ast::Empty"),
        _ => (),
    }
}

#[test]
fn test_into_ast_concat() {
    let span = Span;
    let concat_ast_1 = Ast::Empty(Span);
    let concat_ast_2 = Ast::Empty(Span);
    
    let concat_ast = ConcatAst {
        asts: vec![concat_ast_1, concat_ast_2],
        span,
    };
    
    let result = concat_ast.into_ast();
    
    match result {
        Ast::Concat(_) => (),
        _ => panic!("Expected Ast::Concat but got {:?}", result),
    }
}

