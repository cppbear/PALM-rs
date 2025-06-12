// Answer 0

#[test]
fn test_child_repetition() {
    struct MockAst;
    struct MockRepetition {
        ast: Box<Ast>,
    }
    struct MockGroup {
        ast: Box<Ast>,
    }

    let ast_node = Ast::Literal(Literal::Character('a'));
    let repetition = MockRepetition {
        ast: Box::new(ast_node.clone()),
    };
    let frame = Frame::Repetition(&repetition);
    assert_eq!(frame.child(), &ast_node);
}

#[test]
fn test_child_group() {
    struct MockAst;
    struct MockRepetition {
        ast: Box<Ast>,
    }
    struct MockGroup {
        ast: Box<Ast>,
    }

    let ast_node = Ast::Group(MockGroup {
        ast: Box::new(Ast::Literal(Literal::Character('b'))),
    });
    let group = MockGroup {
        ast: Box::new(ast_node.clone()),
    };
    let frame = Frame::Group(&group);
    assert_eq!(frame.child(), &ast_node);
}

#[test]
fn test_child_concat() {
    struct MockAst;
    struct MockRepetition {
        ast: Box<Ast>,
    }
    struct MockGroup {
        ast: Box<Ast>,
    }
    
    let head_node = Ast::Literal(Literal::Character('c'));
    let concat_frame = Frame::Concat {
        head: &head_node,
        tail: &[],
    };
    assert_eq!(concat_frame.child(), &head_node);
}

#[test]
fn test_child_alternation() {
    struct MockAst;
    struct MockRepetition {
        ast: Box<Ast>,
    }
    struct MockGroup {
        ast: Box<Ast>,
    }
    
    let head_node = Ast::Literal(Literal::Character('d'));
    let alternation_frame = Frame::Alternation {
        head: &head_node,
        tail: &[],
    };
    assert_eq!(alternation_frame.child(), &head_node);
}

