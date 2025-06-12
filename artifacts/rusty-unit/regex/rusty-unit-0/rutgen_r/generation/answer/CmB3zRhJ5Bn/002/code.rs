// Answer 0

#[test]
fn test_into_ast_perl() {
    struct Primitive {
        kind: PrimitiveKind,
    }

    enum PrimitiveKind {
        Perl(String),
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
        Perl(String),
    }

    impl Primitive {
        fn into_ast(self) -> Ast {
            match self.kind {
                PrimitiveKind::Perl(cls) => Ast { kind: AstKind::Class(Class { kind: ClassKind::Perl(cls) }) },
            }
        }
    }

    let input = Primitive { kind: PrimitiveKind::Perl(String::from("test")) };
    let result = input.into_ast();

    if let AstKind::Class(Class { kind }) = result.kind {
        if let ClassKind::Perl(cls) = kind {
            assert_eq!(cls, "test");
        } else {
            panic!("Expected ClassKind::Perl");
        }
    } else {
        panic!("Expected AstKind::Class");
    }
}

