// Answer 0

#[test]
fn test_has_subexprs_concat() {
    struct Ast {
        kind: AstKind,
    }

    enum AstKind {
        Concat(Box<Ast>, Box<Ast>),
        Empty,
        Flags,
        Literal,
        Dot,
        Assertion,
        Class,
        Repetition,
        Group,
        Alternation,
    }

    impl Ast {
        fn has_subexprs(&self) -> bool {
            match self.kind {
                AstKind::Empty
                | AstKind::Flags
                | AstKind::Literal
                | AstKind::Dot
                | AstKind::Assertion => false,
                AstKind::Class
                | AstKind::Repetition
                | AstKind::Group
                | AstKind::Alternation
                | AstKind::Concat(_, _) => true,
            }
        }
    }

    let left = Ast { kind: AstKind::Literal }; // Left subexpression
    let right = Ast { kind: AstKind::Literal }; // Right subexpression
    let concat_ast = Ast { kind: AstKind::Concat(Box::new(left), Box::new(right)) };

    assert!(concat_ast.has_subexprs());
}

