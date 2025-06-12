// Answer 0

#[test]
fn test_has_subexprs_with_literal() {
    struct Ast {
        variant: AstVariant,
    }

    enum AstVariant {
        Empty,
        Flags,
        Literal,
        Dot,
        Assertion,
        Class,
        Repetition,
        Group,
        Alternation,
        Concat,
    }

    impl Ast {
        fn has_subexprs(&self) -> bool {
            match self.variant {
                AstVariant::Empty
                | AstVariant::Flags
                | AstVariant::Literal
                | AstVariant::Dot
                | AstVariant::Assertion => false,
                AstVariant::Class
                | AstVariant::Repetition
                | AstVariant::Group
                | AstVariant::Alternation
                | AstVariant::Concat => true,
            }
        }
    }

    let literal_ast = Ast {
        variant: AstVariant::Literal,
    };
    assert_eq!(literal_ast.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_with_empty() {
    let empty_ast = Ast {
        variant: AstVariant::Empty,
    };
    assert_eq!(empty_ast.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_with_flags() {
    let flags_ast = Ast {
        variant: AstVariant::Flags,
    };
    assert_eq!(flags_ast.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_with_dot() {
    let dot_ast = Ast {
        variant: AstVariant::Dot,
    };
    assert_eq!(dot_ast.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_with_assertion() {
    let assertion_ast = Ast {
        variant: AstVariant::Assertion,
    };
    assert_eq!(assertion_ast.has_subexprs(), false);
}

