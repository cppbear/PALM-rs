// Answer 0

#[test]
fn test_has_subexprs_repetition() {
    struct Ast {
        variant: AstVariant,
    }

    enum AstVariant {
        Empty(),
        Flags(),
        Literal(),
        Dot(),
        Assertion(),
        Class(),
        Repetition(Box<Ast>),
        Group(Box<Ast>),
        Alternation(Vec<Ast>),
        Concat(Vec<Ast>),
    }

    impl Ast {
        fn has_subexprs(&self) -> bool {
            match self.variant {
                AstVariant::Empty()
                | AstVariant::Flags()
                | AstVariant::Literal()
                | AstVariant::Dot()
                | AstVariant::Assertion() => false,
                AstVariant::Class()
                | AstVariant::Repetition(_)
                | AstVariant::Group(_)
                | AstVariant::Alternation(_)
                | AstVariant::Concat(_) => true,
            }
        }
    }

    let sub_ast = Ast { variant: AstVariant::Literal() };
    let repetition_ast = Ast { variant: AstVariant::Repetition(Box::new(sub_ast)) };

    assert!(repetition_ast.has_subexprs());
}

