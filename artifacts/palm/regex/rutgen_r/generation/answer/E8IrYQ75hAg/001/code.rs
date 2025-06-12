// Answer 0

#[derive(Debug)]
struct Ast {
    span: usize,
    asts: Vec<Ast>,
}

impl Ast {
    fn Empty(span: usize) -> Ast {
        Ast { span, asts: vec![] }
    }

    fn Concat(self) -> Ast {
        Ast { span: self.span, asts: self.asts }
    }
}

#[derive(Debug)]
struct ConcatStruct {
    span: usize,
    asts: Vec<Ast>,
}

impl ConcatStruct {
    fn into_ast(mut self) -> Ast {
        match self.asts.len() {
            0 => Ast::Empty(self.span),
            1 => self.asts.pop().unwrap(),
            _ => Ast::Concat(self),
        }
    }
}

#[test]
fn test_into_ast_empty() {
    let concat_struct = ConcatStruct { span: 0, asts: vec![] };
    let result = concat_struct.into_ast();
    match result {
        Ast::Empty(span) => assert_eq!(span, 0),
        _ => panic!("Expected Ast::Empty"),
    }
}

#[test]
fn test_into_ast_single() {
    let single_ast = Ast { span: 1, asts: vec![] };
    let concat_struct = ConcatStruct { span: 1, asts: vec![single_ast] };
    let result = concat_struct.into_ast();
    match result {
        Ast { span: 1, asts: _ } => (),
        _ => panic!("Expected single AST"),
    }
}

#[test]
fn test_into_ast_concat() {
    let ast1 = Ast { span: 2, asts: vec![] };
    let ast2 = Ast { span: 3, asts: vec![] };
    let concat_struct = ConcatStruct { span: 4, asts: vec![ast1, ast2] };
    let result = concat_struct.into_ast();
    match result {
        Ast { span: 4, asts: _ } => (),
        _ => panic!("Expected Ast::Concat"),
    }
}

