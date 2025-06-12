// Answer 0

#[test]
fn test_is_empty_with_empty_ast() {
    struct Ast {
        kind: AstKind,
    }

    enum AstKind {
        Empty,
        NonEmpty,
    }

    impl Ast {
        pub fn is_empty(&self) -> bool {
            match *self {
                Ast { kind: AstKind::Empty } => true,
                _ => false,
            }
        }
    }

    let empty_ast = Ast { kind: AstKind::Empty };
    assert!(empty_ast.is_empty());
}

#[test]
fn test_is_empty_with_non_empty_ast() {
    struct Ast {
        kind: AstKind,
    }

    enum AstKind {
        Empty,
        NonEmpty,
    }

    impl Ast {
        pub fn is_empty(&self) -> bool {
            match *self {
                Ast { kind: AstKind::Empty } => true,
                _ => false,
            }
        }
    }

    let non_empty_ast = Ast { kind: AstKind::NonEmpty };
    assert!(!non_empty_ast.is_empty());
}

