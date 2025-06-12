// Answer 0

#[test]
fn test_ast_flags_has_subexprs() {
    struct TestAst {
        kind: Ast,
    }

    impl TestAst {
        fn new_flags() -> Self {
            Self {
                kind: Ast::Flags(SetFlags {
                    span: Span { start: Position(0), end: Position(1) },
                    flags: Flags::default(),
                }),
            }
        }
    }

    let ast = TestAst::new_flags();
    assert_eq!(ast.kind.has_subexprs(), false);
}

#[test]
fn test_ast_literal_has_subexprs() {
    struct TestAst {
        kind: Ast,
    }

    impl TestAst {
        fn new_literal() -> Self {
            Self {
                kind: Ast::Literal(Literal {
                    span: Span { start: Position(0), end: Position(1) },
                    kind: LiteralKind::Unicode('a'),
                    c: 'a',
                }),
            }
        }
    }

    let ast = TestAst::new_literal();
    assert_eq!(ast.kind.has_subexprs(), false);
}

#[test]
fn test_ast_empty_has_subexprs() {
    struct TestAst {
        kind: Ast,
    }

    impl TestAst {
        fn new_empty() -> Self {
            Self {
                kind: Ast::Empty(Span { start: Position(0), end: Position(0) }),
            }
        }
    }

    let ast = TestAst::new_empty();
    assert_eq!(ast.kind.has_subexprs(), false);
}

#[test]
fn test_ast_assertion_has_subexprs() {
    struct TestAst {
        kind: Ast,
    }

    impl TestAst {
        fn new_assertion() -> Self {
            Self {
                kind: Ast::Assertion(Assertion {
                    span: Span { start: Position(0), end: Position(1) },
                    kind: AssertionKind::Start,
                }),
            }
        }
    }

    let ast = TestAst::new_assertion();
    assert_eq!(ast.kind.has_subexprs(), false);
}

#[test]
fn test_ast_dot_has_subexprs() {
    struct TestAst {
        kind: Ast,
    }

    impl TestAst {
        fn new_dot() -> Self {
            Self {
                kind: Ast::Dot(Span { start: Position(0), end: Position(1) }),
            }
        }
    }

    let ast = TestAst::new_dot();
    assert_eq!(ast.kind.has_subexprs(), false);
}

