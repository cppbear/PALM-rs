// Answer 0

#[test]
fn test_empty_ast() {
    let span = Span { start: 0, end: 0 };
    let ast = Ast::Empty(span);
    let mut formatter = String::new();
    ast.fmt(&mut formatter).unwrap();
}

#[test]
fn test_literal_ast() {
    let span = Span { start: 0, end: 1 };
    let literal = Literal::Unicode('a');
    let ast = Ast::Literal(Literal { span, kind: literal, c: 'a' });
    let mut formatter = String::new();
    ast.fmt(&mut formatter).unwrap();
}

#[test]
fn test_dot_ast() {
    let span = Span { start: 0, end: 1 };
    let ast = Ast::Dot(span);
    let mut formatter = String::new();
    ast.fmt(&mut formatter).unwrap();
}

#[test]
fn test_assertion_ast() {
    let span = Span { start: 0, end: 1 };
    let assertion = Assertion { span, kind: AssertionKind::StartOfLine };
    let ast = Ast::Assertion(assertion);
    let mut formatter = String::new();
    ast.fmt(&mut formatter).unwrap();
}

#[test]
fn test_concat_ast() {
    let span = Span { start: 0, end: 3 };
    let literal_a = Literal { v: vec![97], cut: false }; // 'a'
    let literal_b = Literal { v: vec![98], cut: false }; // 'b'
    let asts = vec![Ast::Literal(literal_a), Ast::Literal(literal_b)];
    let ast = Ast::Concat(Concat { span, asts });
    let mut formatter = String::new();
    ast.fmt(&mut formatter).unwrap();
}

#[test]
fn test_repetition_ast_greedy() {
    let span = Span { start: 0, end: 3 };
    let literal = Literal { v: vec![97], cut: false }; // 'a'
    let ast = Ast::Repetition(Repetition {
        kind: RepetitionKind::ZeroOrMore,
        greedy: true,
        hir: Box::new(Ast::Literal(literal)),
    });
    let mut formatter = String::new();
    ast.fmt(&mut formatter).unwrap();
}

#[test]
fn test_repetition_ast_non_greedy() {
    let span = Span { start: 0, end: 3 };
    let literal = Literal { v: vec![97], cut: false }; // 'a'
    let ast = Ast::Repetition(Repetition {
        kind: RepetitionKind::ZeroOrMore,
        greedy: false,
        hir: Box::new(Ast::Literal(literal)),
    });
    let mut formatter = String::new();
    ast.fmt(&mut formatter).unwrap();
}

#[test]
fn test_alternation_ast() {
    let span = Span { start: 0, end: 3 };
    let literal_a = Literal { v: vec![97], cut: false }; // 'a'
    let literal_b = Literal { v: vec![98], cut: false }; // 'b'
    let asts = vec![Ast::Literal(literal_a), Ast::Literal(literal_b)];
    let ast = Ast::Alternation(Alternation { span, asts });
    let mut formatter = String::new();
    ast.fmt(&mut formatter).unwrap();
}

#[test]
fn test_group_ast() {
    let span = Span { start: 0, end: 3 };
    let literal = Literal { v: vec![97], cut: false }; // 'a'
    let group = Group {
        span,
        kind: GroupKind::Capturing(0),
        ast: Box::new(Ast::Literal(literal)),
    };
    let ast = Ast::Group(group);
    let mut formatter = String::new();
    ast.fmt(&mut formatter).unwrap();
}

#[test]
fn test_class_ast() {
    let span = Span { start: 0, end: 3 };
    let class = Class::Unicode(ClassUnicode::Latin);
    let ast = Ast::Class(class);
    let mut formatter = String::new();
    ast.fmt(&mut formatter).unwrap();
}

