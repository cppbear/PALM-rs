// Answer 0

#[test]
fn test_has_subexprs_concat_with_literals_and_classes() {
    let ast = Ast::Concat(Concat {
        span: Span {
            start: Position(0),
            end: Position(100),
        },
        asts: vec![
            Ast::Literal(Literal {
                span: Span {
                    start: Position(0),
                    end: Position(5),
                },
                kind: LiteralKind::SomeKind,
                c: 'a',
            }),
            Ast::Class(Class::Unicode(ClassUnicode::SomeUnicode)),
        ],
    });
    ast.has_subexprs();
}

#[test]
fn test_has_subexprs_concat_with_empty_literal() {
    let ast = Ast::Concat(Concat {
        span: Span {
            start: Position(0),
            end: Position(50),
        },
        asts: vec![
            Ast::Literal(Literal {
                span: Span {
                    start: Position(0),
                    end: Position(0),
                },
                kind: LiteralKind::SomeKind,
                c: '\0',
            }),
            Ast::Repetition(Repetition {
                span: Span {
                    start: Position(0),
                    end: Position(10),
                },
                op: RepetitionOp::SomeOp,
                greedy: true,
                ast: Box::new(Ast::Empty(Span {
                    start: Position(0),
                    end: Position(0),
                })),
            }),
        ],
    });
    ast.has_subexprs();
}

