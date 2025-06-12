// Answer 0

#[test]
fn test_hirkind_has_subexprs_concat() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Span;

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Ast;

    #[derive(Clone, Debug, Eq, PartialEq)]
    enum HirKind {
        Empty,
        Literal(Literal),
        Class(Class),
        Anchor(Anchor),
        WordBoundary(WordBoundary),
        Repetition(Repetition),
        Group(Group),
        Concat(Vec<Hir>),
        Alternation(Vec<Hir>),
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Literal;

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Class;

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Anchor;

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct WordBoundary;

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Repetition;

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Group;

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Hir {
        kind: HirKind,
    }

    impl HirKind {
        pub fn has_subexprs(&self) -> bool {
            match self {
                HirKind::Empty
                | HirKind::Literal(_)
                | HirKind::Class(_)
                | HirKind::Anchor(_)
                | HirKind::WordBoundary(_) => false,
                HirKind::Group(_)
                | HirKind::Repetition(_)
                | HirKind::Concat(_)
                | HirKind::Alternation(_) => true,
            }
        }
    }

    let hir_concat = Hir {
        kind: HirKind::Concat(vec![
            Hir {
                kind: HirKind::Literal(Literal),
            },
            Hir {
                kind: HirKind::Class(Class),
            },
        ]),
    };

    assert!(hir_concat.kind.has_subexprs());
}

