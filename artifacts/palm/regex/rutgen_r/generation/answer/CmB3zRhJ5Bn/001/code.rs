// Answer 0

#[test]
fn test_into_ast_unicode_class() {
    struct Primitive {
        cls: String,
    }

    struct Ast {
        kind: AstKind,
    }

    enum AstKind {
        Class(Class),
    }

    struct Class {
        kind: ClassKind,
    }

    enum ClassKind {
        Unicode(String),
    }

    impl Primitive {
        fn into_ast(self) -> Ast {
            match self {
                Primitive { cls } => Ast {
                    kind: AstKind::Class(Class {
                        kind: ClassKind::Unicode(cls),
                    }),
                },
            }
        }
    }

    // Test input that satisfies the constraint
    let unicode_class = Primitive { cls: String::from("\\u{2603}") };
    let ast = unicode_class.into_ast();

    if let AstKind::Class(Class { kind }) = ast.kind {
        if let ClassKind::Unicode(cls) = kind {
            assert_eq!(cls, "\\u{2603}");
        } else {
            panic!("Expected Unicode class variant");
        }
    } else {
        panic!("Expected Class variant in AST");
    }
}

