// Answer 0

#[derive(Debug)]
enum Ast {
    Empty,
    Flags,
    Literal,
    Dot,
    Assertion,
    Class,
    Repetition,
    Group(Box<Ast>),
    Alternation(Box<Ast>, Box<Ast>),
    Concat(Box<Ast>, Box<Ast>),
}

impl Ast {
    fn has_subexprs(&self) -> bool {
        match *self {
            Ast::Empty
            | Ast::Flags
            | Ast::Literal
            | Ast::Dot
            | Ast::Assertion => false,
            Ast::Class
            | Ast::Repetition
            | Ast::Group(_)
            | Ast::Alternation(_, _)
            | Ast::Concat(_, _) => true,
        }
    }
}

#[test]
fn test_has_subexprs_group() {
    let group_ast = Ast::Group(Box::new(Ast::Literal)); // Group containing a Literal, which is valid
    assert!(group_ast.has_subexprs()); // This should return true
}

#[test]
fn test_has_subexprs_empty_group() {
    let empty_group_ast = Ast::Group(Box::new(Ast::Empty)); // Group containing an Empty, which is valid
    assert!(empty_group_ast.has_subexprs()); // This should still return true, a group has subexpressions
}

