// Answer 0

#[test]
fn test_has_subexprs_group_with_repetition() {
    let ast = Ast::Group(Group {
        span: Span {
            start: Position(0),
            end: Position(10),
        },
        kind: GroupKind::Capturing,
        hir: Box::new(Ast::Repetition(Repetition {
            span: Span {
                start: Position(1),
                end: Position(5),
            },
            op: RepetitionOp::Star,
            greedy: true,
            ast: Box::new(Ast::Literal(Literal {
                span: Span {
                    start: Position(2),
                    end: Position(3),
                },
                kind: LiteralKind::Unicode,
                c: 'a',
            })),
        })),
    });
    ast.has_subexprs();
}

#[test]
fn test_has_subexprs_group_with_alternation() {
    let ast = Ast::Group(Group {
        span: Span {
            start: Position(0),
            end: Position(10),
        },
        kind: GroupKind::Capturing,
        hir: Box::new(Ast::Alternation(Alternation {
            span: Span {
                start: Position(1),
                end: Position(9),
            },
            asts: vec![
                Ast::Literal(Literal {
                    span: Span {
                        start: Position(2),
                        end: Position(3),
                    },
                    kind: LiteralKind::Unicode,
                    c: 'b',
                }),
                Ast::Literal(Literal {
                    span: Span {
                        start: Position(4),
                        end: Position(5),
                    },
                    kind: LiteralKind::Unicode,
                    c: 'c',
                }),
            ],
        })),
    });
    ast.has_subexprs();
}

#[test]
fn test_has_subexprs_group_with_concat() {
    let ast = Ast::Group(Group {
        span: Span {
            start: Position(0),
            end: Position(10),
        },
        kind: GroupKind::Capturing,
        hir: Box::new(Ast::Concat(Concat {
            span: Span {
                start: Position(1),
                end: Position(9),
            },
            asts: vec![
                Ast::Literal(Literal {
                    span: Span {
                        start: Position(2),
                        end: Position(3),
                    },
                    kind: LiteralKind::Unicode,
                    c: 'd',
                }),
                Ast::Literal(Literal {
                    span: Span {
                        start: Position(4),
                        end: Position(5),
                    },
                    kind: LiteralKind::Unicode,
                    c: 'e',
                }),
            ],
        })),
    });
    ast.has_subexprs();
}

