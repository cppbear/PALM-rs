// Answer 0

#[test]
fn test_repetition_with_empty_inner() {
    let inner_ast = Ast::Empty(Span { start: Position, end: Position });
    let repetition = Repetition { 
        kind: RepetitionKind, 
        greedy: true,
        hir: Box::new(inner_ast) 
    };
    let ast = Ast::Repetition(repetition);
    ast.has_subexprs();
}

#[test]
fn test_repetition_with_literal_inner() {
    let inner_ast = Ast::Literal(Literal { span: Span { start: Position, end: Position }, kind: LiteralKind, c: 'a' });
    let repetition = Repetition { 
        kind: RepetitionKind, 
        greedy: false,
        hir: Box::new(inner_ast) 
    };
    let ast = Ast::Repetition(repetition);
    ast.has_subexprs();
}

#[test]
fn test_repetition_with_dot_inner() {
    let inner_ast = Ast::Dot(Span { start: Position, end: Position });
    let repetition = Repetition { 
        kind: RepetitionKind, 
        greedy: true,
        hir: Box::new(inner_ast) 
    };
    let ast = Ast::Repetition(repetition);
    ast.has_subexprs();
}

#[test]
fn test_repetition_with_group_inner() {
    let inner_group = Group { 
        span: Span { start: Position, end: Position }, 
        kind: GroupKind, 
        ast: Box::new(Ast::Empty(Span { start: Position, end: Position })) 
    };
    let repetition = Repetition { 
        kind: RepetitionKind, 
        greedy: true,
        hir: Box::new(Ast::Group(inner_group)) 
    };
    let ast = Ast::Repetition(repetition);
    ast.has_subexprs();
}

#[test]
fn test_repetition_with_alternation_inner() {
    let inner_alternation = Alternation { 
        span: Span { start: Position, end: Position }, 
        asts: vec![Ast::Literal(Literal { span: Span { start: Position, end: Position }, kind: LiteralKind, c: 'b' })] 
    };
    let repetition = Repetition { 
        kind: RepetitionKind, 
        greedy: false,
        hir: Box::new(Ast::Alternation(inner_alternation)) 
    };
    let ast = Ast::Repetition(repetition);
    ast.has_subexprs();
}

