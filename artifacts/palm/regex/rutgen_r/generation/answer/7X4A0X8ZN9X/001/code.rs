// Answer 0

#[test]
fn test_child_with_alternation() {
    struct DummyAst;

    struct Repetition {
        ast: &'static DummyAst,
    }

    struct Group {
        ast: &'static DummyAst,
    }

    struct Concat {
        head: &'static DummyAst,
    }

    enum Frame<'a> {
        Repetition(Repetition),
        Group(Group),
        Concat { head: &'a DummyAst, tail: &'a [DummyAst] },
        Alternation { head: &'a DummyAst, tail: &'a [DummyAst] },
    }

    let ast = DummyAst;
    
    let frame = Frame::Alternation { head: &ast, tail: &[] };

    match frame {
        Frame::Alternation { head, .. } => {
            assert_eq!(head, &ast);
        },
        _ => panic!("Expected Frame::Alternation"),
    }
}

#[test]
fn test_child_with_repetition() {
    struct DummyAst;

    struct Repetition {
        ast: &'static DummyAst,
    }

    struct Group {
        ast: &'static DummyAst,
    }

    struct Concat {
        head: &'static DummyAst,
    }

    enum Frame<'a> {
        Repetition(Repetition),
        Group(Group),
        Concat { head: &'a DummyAst, tail: &'a [DummyAst] },
        Alternation { head: &'a DummyAst, tail: &'a [DummyAst] },
    }

    let ast = DummyAst;
    let rep_frame = Frame::Repetition(Repetition { ast: &ast });

    match rep_frame {
        Frame::Repetition(rep) => {
            assert_eq!(rep.ast, &ast);
        },
        _ => panic!("Expected Frame::Repetition"),
    }
}

#[test]
fn test_child_with_group() {
    struct DummyAst;

    struct Repetition {
        ast: &'static DummyAst,
    }

    struct Group {
        ast: &'static DummyAst,
    }

    struct Concat {
        head: &'static DummyAst,
    }

    enum Frame<'a> {
        Repetition(Repetition),
        Group(Group),
        Concat { head: &'a DummyAst, tail: &'a [DummyAst] },
        Alternation { head: &'a DummyAst, tail: &'a [DummyAst] },
    }

    let ast = DummyAst;
    let group_frame = Frame::Group(Group { ast: &ast });

    match group_frame {
        Frame::Group(group) => {
            assert_eq!(group.ast, &ast);
        },
        _ => panic!("Expected Frame::Group"),
    }
}

#[test]
fn test_child_with_concat() {
    struct DummyAst;

    struct Repetition {
        ast: &'static DummyAst,
    }

    struct Group {
        ast: &'static DummyAst,
    }

    struct Concat {
        head: &'static DummyAst,
    }

    enum Frame<'a> {
        Repetition(Repetition),
        Group(Group),
        Concat { head: &'a DummyAst, tail: &'a [DummyAst] },
        Alternation { head: &'a DummyAst, tail: &'a [DummyAst] },
    }

    let ast = DummyAst;
    let concat_frame = Frame::Concat { head: &ast, tail: &[] };

    match concat_frame {
        Frame::Concat { head, .. } => {
            assert_eq!(head, &ast);
        },
        _ => panic!("Expected Frame::Concat"),
    }
}

