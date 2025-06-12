// Answer 0

#[test]
fn test_frame_child_group_with_non_empty_hir() {
    let span = Span::new(0, 10);
    let literal = Literal::from('a');
    let group = Group {
        span,
        kind: GroupKind::Capturing(0, None),
        ast: Box::new(Ast::Literal(literal)),
    };
    let frame = Frame::Group(&group);
    let result = frame.child();
}

#[test]
fn test_frame_child_group_with_empty_hir() {
    let span = Span::new(0, 5);
    let group = Group {
        span,
        kind: GroupKind::Capturing(1, None),
        ast: Box::new(Ast::Empty(span)),
    };
    let frame = Frame::Group(&group);
    let result = frame.child();
}

#[test]
fn test_frame_child_group_with_class() {
    let span = Span::new(3, 8);
    let class = Class::new(vec!['a', 'b', 'c']);
    let group = Group {
        span,
        kind: GroupKind::Capturing(2, None),
        ast: Box::new(Ast::Class(class)),
    };
    let frame = Frame::Group(&group);
    let result = frame.child();
}

#[test]
fn test_frame_child_repetition() {
    let span = Span::new(0, 15);
    let literal = Literal::from('b');
    let repetition = Repetition {
        span,
        op: RepetitionOp::ZeroOrMore,
        greedy: true,
        ast: Box::new(Ast::Literal(literal)),
    };
    let frame = Frame::Repetition(&repetition);
    let result = frame.child();
}

#[test]
fn test_frame_child_concat() {
    let span = Span::new(1, 11);
    let ast1 = Ast::Literal(Literal::from('x'));
    let ast2 = Ast::Literal(Literal::from('y'));
    let concat = Concat {
        span,
        ops: vec![ast1.clone(), ast2],
    };
    let frame = Frame::Concat { head: &ast1, tail: &[] };
    let result = frame.child();
}

#[test]
fn test_frame_child_alternation() {
    let span = Span::new(0, 20);
    let ast1 = Ast::Class(Class::new(vec!['d', 'e', 'f']));
    let ast2 = Ast::Literal(Literal::from('z'));
    let alternation = Alternation {
        span,
        ops: vec![ast1.clone(), ast2],
    };
    let frame = Frame::Alternation { head: &ast1, tail: &[] };
    let result = frame.child();
}

