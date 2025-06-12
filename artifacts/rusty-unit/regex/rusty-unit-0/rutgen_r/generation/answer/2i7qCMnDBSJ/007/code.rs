// Answer 0

#[derive(Debug)]
enum Ast {
    Empty(),
    Flags(),
    Literal(),
    Dot(),
    Assertion(),
    Class(),
    Repetition(),
    Group(),
    Alternation(),
    Concat(),
}

impl Ast {
    fn has_subexprs(&self) -> bool {
        match *self {
            Ast::Empty()
            | Ast::Flags()
            | Ast::Literal()
            | Ast::Dot()
            | Ast::Assertion() => false,
            Ast::Class()
            | Ast::Repetition()
            | Ast::Group()
            | Ast::Alternation()
            | Ast::Concat() => true,
        }
    }
}

#[test]
fn test_has_subexprs_empty() {
    let ast = Ast::Empty();
    assert_eq!(ast.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_flags() {
    let ast = Ast::Flags();
    assert_eq!(ast.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_literal() {
    let ast = Ast::Literal();
    assert_eq!(ast.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_dot() {
    let ast = Ast::Dot();
    assert_eq!(ast.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_assertion() {
    let ast = Ast::Assertion();
    assert_eq!(ast.has_subexprs(), false);
}

