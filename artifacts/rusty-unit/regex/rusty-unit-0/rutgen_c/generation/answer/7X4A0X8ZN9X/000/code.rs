// Answer 0

#[test]
fn test_frame_child_repetition() {
    let repetition_ast = Ast::Literal(Literal::from('a'));
    let repetition = Repetition {
        kind: RepetitionKind::Star,
        greedy: true,
        hir: Box::new(repetition_ast.clone()),
    };
    let frame = Frame::Repetition(&repetition);
    
    assert_eq!(*frame.child(), repetition_ast);
}

#[test]
fn test_frame_child_group() {
    let group_ast = Ast::Literal(Literal::from('b'));
    let group = Group {
        kind: GroupKind::Capturing(0), // Assuming capturing group index 0
        hir: Box::new(group_ast.clone()),
    };
    let frame = Frame::Group(&group);
    
    assert_eq!(*frame.child(), group_ast);
}

#[test]
fn test_frame_child_concat() {
    let head_ast = Ast::Literal(Literal::from('c'));
    let tail_ast = vec![Ast::Literal(Literal::from('d'))];
    let frame = Frame::Concat { head: &head_ast, tail: &tail_ast };
    
    assert_eq!(*frame.child(), head_ast);
}

#[test]
fn test_frame_child_alternation() {
    let head_ast = Ast::Literal(Literal::from('e'));
    let tail_ast = vec![Ast::Literal(Literal::from('f'))];
    let frame = Frame::Alternation { head: &head_ast, tail: &tail_ast };
    
    assert_eq!(*frame.child(), head_ast);
}

