// Answer 0

#[test]
fn test_child_concat_frame() {
    struct Ast;
    
    struct Frame {
        ast: &'static Ast,
    }

    struct Concat {
        head: &'static Ast,
    }

    impl Frame {
        fn child(&self) -> &'static Ast {
            match *self {
                Frame { ast } => ast,
            }
        }
    }

    let ast_node = Ast;
    let concat_frame = Frame { ast: &ast_node };

    let result = concat_frame.child();
    assert_eq!(result, &ast_node);
}

#[test]
fn test_child_repetition_frame() {
    struct Ast;

    struct Repetition<'a> {
        ast: &'a Ast,
    }

    struct Frame<'a> {
        kind: FrameKind<'a>,
    }

    enum FrameKind<'a> {
        Repetition(Repetition<'a>),
        Group(Group<'a>),
        Concat { head: &'a Ast },
        Alternation { head: &'a Ast },
    }

    struct Group<'a> {
        ast: &'a Ast,
    }

    impl<'a> Frame<'a> {
        fn child(&self) -> &'a Ast {
            match &self.kind {
                FrameKind::Repetition(rep) => rep.ast,
                FrameKind::Group(group) => group.ast,
                FrameKind::Concat { head, .. } => head,
                FrameKind::Alternation { head, .. } => head,
            }
        }
    }

    let ast_node = Ast;
    let repetition_frame = Frame { kind: FrameKind::Repetition(Repetition { ast: &ast_node }) };

    let result = repetition_frame.child();
    assert_eq!(result, &ast_node);
}

#[test]
fn test_child_alternation_frame() {
    struct Ast;

    struct Alternation<'a> {
        head: &'a Ast,
    }

    struct Frame<'a> {
        kind: FrameKind<'a>,
    }

    enum FrameKind<'a> {
        Repetition(Repetition<'a>),
        Group(Group<'a>),
        Concat { head: &'a Ast },
        Alternation(Alternation<'a>),
    }

    struct Repetition<'a> {
        ast: &'a Ast,
    }

    struct Group<'a> {
        ast: &'a Ast,
    }

    impl<'a> Frame<'a> {
        fn child(&self) -> &'a Ast {
            match &self.kind {
                FrameKind::Repetition(rep) => rep.ast,
                FrameKind::Group(group) => group.ast,
                FrameKind::Concat { head, .. } => head,
                FrameKind::Alternation(alt) => alt.head,
            }
        }
    }

    let ast_node = Ast;
    let alternation_frame = Frame { kind: FrameKind::Alternation(Alternation { head: &ast_node }) };

    let result = alternation_frame.child();
    assert_eq!(result, &ast_node);
}

#[test]
fn test_child_group_frame() {
    struct Ast;

    struct Group<'a> {
        ast: &'a Ast,
    }

    struct Frame<'a> {
        kind: FrameKind<'a>,
    }

    enum FrameKind<'a> {
        Repetition(Repetition<'a>),
        Group(Group<'a>),
        Concat { head: &'a Ast },
        Alternation { head: &'a Ast },
    }

    struct Repetition<'a> {
        ast: &'a Ast,
    }

    impl<'a> Frame<'a> {
        fn child(&self) -> &'a Ast {
            match &self.kind {
                FrameKind::Repetition(rep) => rep.ast,
                FrameKind::Group(group) => group.ast,
                FrameKind::Concat { head, .. } => head,
                FrameKind::Alternation { head, .. } => head,
            }
        }
    }

    let ast_node = Ast;
    let group_frame = Frame { kind: FrameKind::Group(Group { ast: &ast_node }) };

    let result = group_frame.child();
    assert_eq!(result, &ast_node);
}

