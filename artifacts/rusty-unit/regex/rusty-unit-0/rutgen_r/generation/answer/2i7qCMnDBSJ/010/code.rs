// Answer 0

#[derive(Debug)]
enum Ast {
    Empty(()),
    Flags(()),
    Literal(String),
    Dot(()),
    Assertion(()),
    Class(()),
    Repetition(()),
    Group(()),
    Alternation(()),
    Concat(()),
}

impl Ast {
    fn has_subexprs(&self) -> bool {
        match *self {
            Ast::Empty(_)
            | Ast::Flags(_)
            | Ast::Literal(_)
            | Ast::Dot(_)
            | Ast::Assertion(_) => false,
            Ast::Class(_)
            | Ast::Repetition(_)
            | Ast::Group(_)
            | Ast::Alternation(_)
            | Ast::Concat(_) => true,
        }
    }
}

#[test]
fn test_has_subexprs_empty() {
    let ast = Ast::Empty(());
    assert_eq!(ast.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_literal() {
    let ast = Ast::Literal(String::from("test"));
    assert_eq!(ast.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_flags() {
    let ast = Ast::Flags(());
    assert_eq!(ast.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_assertion() {
    let ast = Ast::Assertion(());
    assert_eq!(ast.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_dot() {
    let ast = Ast::Dot(());
    assert_eq!(ast.has_subexprs(), false);
}

