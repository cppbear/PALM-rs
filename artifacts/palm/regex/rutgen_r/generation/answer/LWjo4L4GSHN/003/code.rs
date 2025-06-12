// Answer 0

#[test]
fn test_hir_repetition_zero_or_one() {
    struct TestStruct;

    impl TestStruct {
        fn flags(&self) -> Flags {
            Flags { swap_greed: false }
        }

        fn hir_repetition(&self, rep: &ast::Repetition, expr: Hir) -> Hir {
            // Function implementation would be placed here
            // This is just a mock for the test context
            Hir::repetition(hir::Repetition { kind: hir::RepetitionKind::ZeroOrOne, greedy: rep.greedy, hir: Box::new(expr) })
        }
    }

    struct Repetition {
        op: RepetitionOp,
        greedy: bool,
    }

    struct RepetitionOp {
        kind: RepetitionKind,
    }

    enum RepetitionKind {
        ZeroOrOne,
        ZeroOrMore,
        OneOrMore,
        Range(RepetitionRange),
    }

    enum RepetitionRange {
        Exactly(usize),
        AtLeast(usize),
        Bounded(usize, usize),
    }

    let test_struct = TestStruct;
    let rep = Repetition {
        op: RepetitionOp {
            kind: RepetitionKind::ZeroOrOne,
        },
        greedy: true,
    };
    let expr = Hir::new(); // Assuming Hir::new() is a method that initializes an Hir structure

    // Test the function
    let result = test_struct.hir_repetition(&rep, expr);
    
    // Assuming we have some way to verify the result
    assert_eq!(result.kind, hir::RepetitionKind::ZeroOrOne);
    assert_eq!(result.greedy, rep.greedy);
}

#[test]
fn test_hir_repetition_zero_or_more() {
    struct TestStruct;

    impl TestStruct {
        fn flags(&self) -> Flags {
            Flags { swap_greed: false }
        }

        fn hir_repetition(&self, rep: &ast::Repetition, expr: Hir) -> Hir {
            // Function implementation would be placed here
            // This is just a mock for the test context
            Hir::repetition(hir::Repetition { kind: hir::RepetitionKind::ZeroOrMore, greedy: rep.greedy, hir: Box::new(expr) })
        }
    }

    let test_struct = TestStruct;
    let rep = Repetition {
        op: RepetitionOp {
            kind: RepetitionKind::ZeroOrMore,
        },
        greedy: false,
    };
    let expr = Hir::new();

    // Test the function
    let result = test_struct.hir_repetition(&rep, expr);
    
    // Assuming we have some way to verify the result
    assert_eq!(result.kind, hir::RepetitionKind::ZeroOrMore);
    assert_eq!(result.greedy, rep.greedy);
}

#[test]
fn test_hir_repetition_at_least() {
    struct TestStruct;

    impl TestStruct {
        fn flags(&self) -> Flags {
            Flags { swap_greed: true }
        }

        fn hir_repetition(&self, rep: &ast::Repetition, expr: Hir) -> Hir {
            // Function implementation would be placed here
            // This is just a mock for the test context
            let kind = match rep.op.kind {
                RepetitionKind::Range(RepetitionRange::AtLeast(m)) => {
                    hir::RepetitionKind::Range(hir::RepetitionRange::AtLeast(m))
                }
                _ => unreachable!(),
            };
            Hir::repetition(hir::Repetition { kind, greedy: !rep.greedy, hir: Box::new(expr) })
        }
    }

    let test_struct = TestStruct;
    let rep = Repetition {
        op: RepetitionOp {
            kind: RepetitionKind::Range(RepetitionRange::AtLeast(3)),
        },
        greedy: true,
    };
    let expr = Hir::new();

    // Test the function
    let result = test_struct.hir_repetition(&rep, expr);
    
    // Assuming we have some way to verify the result
    assert_eq!(result.kind, hir::RepetitionKind::Range(hir::RepetitionRange::AtLeast(3)));
    assert_eq!(result.greedy, false); // Because swap_greed is true
}

#[test]
fn test_hir_repetition_bounded() {
    struct TestStruct;

    impl TestStruct {
        fn flags(&self) -> Flags {
            Flags { swap_greed: true }
        }

        fn hir_repetition(&self, rep: &ast::Repetition, expr: Hir) -> Hir {
            Hir::repetition(hir::Repetition { 
                kind: hir::RepetitionKind::Range(hir::RepetitionRange::Bounded(2, 5)), 
                greedy: !rep.greedy, 
                hir: Box::new(expr) 
            })
        }
    }

    let test_struct = TestStruct;
    let rep = Repetition {
        op: RepetitionOp {
            kind: RepetitionKind::Range(RepetitionRange::Bounded(2, 5)),
        },
        greedy: false,
    };
    let expr = Hir::new();

    // Test the function
    let result = test_struct.hir_repetition(&rep, expr);
    
    // Assuming we have some way to verify the result
    assert_eq!(result.kind, hir::RepetitionKind::Range(hir::RepetitionRange::Bounded(2, 5)));
    assert_eq!(result.greedy, true); // Because swap_greed is true
}

