// Answer 0

#[test]
fn test_capture_index_non_capturing() {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct Position(usize);

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct Flags;

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Span {
        start: Position,
        end: Position,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    enum GroupKind {
        NonCapturing(Flags),
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Group {
        span: Span,
        kind: GroupKind,
        ast: Box<Ast>,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    enum Ast {
        Empty(Span),
    }

    let span = Span {
        start: Position(0),
        end: Position(1),
    };

    let group = Group {
        span,
        kind: GroupKind::NonCapturing(Flags),
        ast: Box::new(Ast::Empty(span)),
    };

    assert_eq!(group.capture_index(), None);
}

