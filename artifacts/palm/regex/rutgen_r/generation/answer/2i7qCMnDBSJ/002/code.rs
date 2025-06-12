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
    Alternation(Box<Ast>, Box<Ast>),
    Concat(Vec<Ast>),
}

#[test]
fn test_has_subexprs_alternation() {
    let subexpr1 = Ast::Literal();
    let subexpr2 = Ast::Class();

    let alternation_expr = Ast::Alternation(Box::new(subexpr1), Box::new(subexpr2));

    assert_eq!(alternation_expr.has_subexprs(), true);
}

