// Answer 0

#[derive(Clone, Debug, Eq, PartialEq)]
struct Position(usize);

#[derive(Clone, Debug, Eq, PartialEq)]
struct SetFlags;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Literal;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Assertion;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Class;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Repetition;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Group;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Concat;

#[test]
fn test_into_ast_empty() {
    let span = Span { start: Position(0), end: Position(0) };
    let alternation = Alternation { span, asts: vec![] };
    let result = alternation.into_ast();
    assert_eq!(result, Ast::Empty(span));
}

#[test]
fn test_into_ast_single() {
    let span = Span { start: Position(0), end: Position(1) };
    let ast_single = Ast::Literal(Literal);
    let alternation = Alternation { span, asts: vec![ast_single.clone()] };
    let result = alternation.into_ast();
    assert_eq!(result, ast_single);
}

#[test]
fn test_into_ast_multiple() {
    let span = Span { start: Position(0), end: Position(2) };
    let ast1 = Ast::Literal(Literal);
    let ast2 = Ast::Flags(SetFlags);
    let alternation = Alternation { span, asts: vec![ast1.clone(), ast2.clone()] };
    let result = alternation.into_ast();
    assert_eq!(result, Ast::Alternation(alternation));
}

