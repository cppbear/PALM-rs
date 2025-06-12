// Answer 0

#[test]
fn test_hir_repetition_zero_or_one() {
    struct TestContext;

    impl TestContext {
        fn flags(&self) -> TestFlags {
            TestFlags
        }

        fn hir_repetition(&self, rep: &ast::Repetition, expr: Hir) -> Hir {
            // Original function
            let kind = match rep.op.kind {
                ast::RepetitionKind::ZeroOrOne => hir::RepetitionKind::ZeroOrOne,
                _ => unreachable!(),
            };
            let greedy = if self.flags().swap_greed() {
                !rep.greedy
            } else {
                rep.greedy
            };
            Hir::repetition(hir::Repetition {
                kind: kind,
                greedy: greedy,
                hir: Box::new(expr),
            })
        }
    }

    struct TestFlags;

    impl TestFlags {
        fn swap_greed(&self) -> bool {
            false
        }
    }

    let rep = ast::Repetition {
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::ZeroOrOne,
        },
        greedy: true,
    };
    let expr = Hir::literal("a");
    
    let result = TestContext.hir_repetition(&rep, expr);
    
    assert_eq!(result.kind, hir::RepetitionKind::ZeroOrOne);
    assert_eq!(result.greedy, true);
}

#[test]
fn test_hir_repetition_zero_or_more() {
    struct TestContext;

    impl TestContext {
        fn flags(&self) -> TestFlags {
            TestFlags
        }

        fn hir_repetition(&self, rep: &ast::Repetition, expr: Hir) -> Hir {
            // Original function
            let kind = match rep.op.kind {
                ast::RepetitionKind::ZeroOrMore => hir::RepetitionKind::ZeroOrMore,
                _ => unreachable!(),
            };
            let greedy = if self.flags().swap_greed() {
                !rep.greedy
            } else {
                rep.greedy
            };
            Hir::repetition(hir::Repetition {
                kind: kind,
                greedy: greedy,
                hir: Box::new(expr),
            })
        }
    }

    struct TestFlags;

    impl TestFlags {
        fn swap_greed(&self) -> bool {
            false
        }
    }

    let rep = ast::Repetition {
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::ZeroOrMore,
        },
        greedy: false,
    };
    let expr = Hir::literal("b");

    let result = TestContext.hir_repetition(&rep, expr);
    
    assert_eq!(result.kind, hir::RepetitionKind::ZeroOrMore);
    assert_eq!(result.greedy, false);
}

#[test]
fn test_hir_repetition_one_or_more() {
    struct TestContext;

    impl TestContext {
        fn flags(&self) -> TestFlags {
            TestFlags
        }

        fn hir_repetition(&self, rep: &ast::Repetition, expr: Hir) -> Hir {
            // Original function
            let kind = match rep.op.kind {
                ast::RepetitionKind::OneOrMore => hir::RepetitionKind::OneOrMore,
                _ => unreachable!(),
            };
            let greedy = if self.flags().swap_greed() {
                !rep.greedy
            } else {
                rep.greedy
            };
            Hir::repetition(hir::Repetition {
                kind: kind,
                greedy: greedy,
                hir: Box::new(expr),
            })
        }
    }

    struct TestFlags;

    impl TestFlags {
        fn swap_greed(&self) -> bool {
            false
        }
    }

    let rep = ast::Repetition {
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::OneOrMore,
        },
        greedy: true,
    };
    let expr = Hir::literal("c");

    let result = TestContext.hir_repetition(&rep, expr);
    
    assert_eq!(result.kind, hir::RepetitionKind::OneOrMore);
    assert_eq!(result.greedy, true);
}

#[test]
fn test_hir_repetition_range_exactly() {
    struct TestContext;

    impl TestContext {
        fn flags(&self) -> TestFlags {
            TestFlags
        }

        fn hir_repetition(&self, rep: &ast::Repetition, expr: Hir) -> Hir {
            // Original function
            let kind = match rep.op.kind {
                ast::RepetitionKind::Range(ast::RepetitionRange::Exactly(m)) => {
                    hir::RepetitionKind::Range(hir::RepetitionRange::Exactly(m))
                },
                _ => unreachable!(),
            };
            let greedy = if self.flags().swap_greed() {
                !rep.greedy
            } else {
                rep.greedy
            };
            Hir::repetition(hir::Repetition {
                kind: kind,
                greedy: greedy,
                hir: Box::new(expr),
            })
        }
    }

    struct TestFlags;

    impl TestFlags {
        fn swap_greed(&self) -> bool {
            false
        }
    }

    let rep = ast::Repetition {
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::Range(ast::RepetitionRange::Exactly(3)),
        },
        greedy: false,
    };
    let expr = Hir::literal("d");

    let result = TestContext.hir_repetition(&rep, expr);
    
    assert_eq!(result.kind, hir::RepetitionKind::Range(hir::RepetitionRange::Exactly(3)));
    assert_eq!(result.greedy, false);
}

#[test]
fn test_hir_repetition_range_at_least() {
    struct TestContext;

    impl TestContext {
        fn flags(&self) -> TestFlags {
            TestFlags
        }

        fn hir_repetition(&self, rep: &ast::Repetition, expr: Hir) -> Hir {
            // Original function
            let kind = match rep.op.kind {
                ast::RepetitionKind::Range(ast::RepetitionRange::AtLeast(m)) => {
                    hir::RepetitionKind::Range(hir::RepetitionRange::AtLeast(m))
                },
                _ => unreachable!(),
            };
            let greedy = if self.flags().swap_greed() {
                !rep.greedy
            } else {
                rep.greedy
            };
            Hir::repetition(hir::Repetition {
                kind: kind,
                greedy: greedy,
                hir: Box::new(expr),
            })
        }
    }

    struct TestFlags;

    impl TestFlags {
        fn swap_greed(&self) -> bool {
            false
        }
    }

    let rep = ast::Repetition {
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::Range(ast::RepetitionRange::AtLeast(5)),
        },
        greedy: true,
    };
    let expr = Hir::literal("e");

    let result = TestContext.hir_repetition(&rep, expr);
    
    assert_eq!(result.kind, hir::RepetitionKind::Range(hir::RepetitionRange::AtLeast(5)));
    assert_eq!(result.greedy, true);
}

#[test]
fn test_hir_repetition_range_bounded() {
    struct TestContext;

    impl TestContext {
        fn flags(&self) -> TestFlags {
            TestFlags
        }

        fn hir_repetition(&self, rep: &ast::Repetition, expr: Hir) -> Hir {
            // Original function
            let kind = match rep.op.kind {
                ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(m, n)) => {
                    hir::RepetitionKind::Range(hir::RepetitionRange::Bounded(m, n))
                },
                _ => unreachable!(),
            };
            let greedy = if self.flags().swap_greed() {
                !rep.greedy
            } else {
                rep.greedy
            };
            Hir::repetition(hir::Repetition {
                kind: kind,
                greedy: greedy,
                hir: Box::new(expr),
            })
        }
    }

    struct TestFlags;

    impl TestFlags {
        fn swap_greed(&self) -> bool {
            false
        }
    }

    let rep = ast::Repetition {
        op: ast::RepetitionOp {
            kind: ast::RepetitionKind::Range(ast::RepetitionRange::Bounded(2, 4)),
        },
        greedy: false,
    };
    let expr = Hir::literal("f");

    let result = TestContext.hir_repetition(&rep, expr);
    
    assert_eq!(result.kind, hir::RepetitionKind::Range(hir::RepetitionRange::Bounded(2, 4)));
    assert_eq!(result.greedy, false);
}

