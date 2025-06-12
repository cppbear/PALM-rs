// Answer 0

#[test]
fn test_repeat_range_literals_non_zero_min() {
    use regex_syntax::hir::{Hir, Literal, Repetition, RepetitionKind};
    use regex_syntax::Literals;

    struct TestF {
        received_hir: Option<Hir>,
    }

    impl TestF {
        fn new() -> Self {
            Self { received_hir: None }
        }
    }

    let mut lits = Literals::new();
    
    let e = Hir::literal(Literal::unicode(b'a')); // A simple literal to test with
    let min = 2;
    let max = Some(5);
    let greedy = true;

    let mut test_f = TestF::new();
    
    repeat_range_literals(
        &e,
        min,
        max,
        greedy,
        &mut lits,
        |hir, _| {
            test_f.received_hir = Some(hir.clone());
        },
    );

    assert!(test_f.received_hir.is_some());
    
    if let Some(hir) = test_f.received_hir {
        match hir {
            Hir::Concat(parts) => {
                assert_eq!(parts.len(), min as usize);
                for part in parts {
                    match part {
                        Hir::Literal(Literal::Unicode(b'a')) => {}
                        _ => panic!("Unexpected Hir variant"),
                    }
                }
            }
            _ => panic!("Expected Hir::Concat"),
        }
    }
}

#[test]
fn test_repeat_range_literals_min_greater_than_max() {
    use regex_syntax::hir::{Hir, Literal};
    use regex_syntax::Literals;

    struct TestF {
        received_hir: Option<Hir>,
    }

    impl TestF {
        fn new() -> Self {
            Self { received_hir: None }
        }
    }

    let mut lits = Literals::new();
    
    let e = Hir::literal(Literal::unicode(b'b')); // Another simple literal
    let min = 3;
    let max = Some(2); // Invalid case, max < min
    let greedy = false;

    let mut test_f = TestF::new();
    
    repeat_range_literals(
        &e,
        min,
        max,
        greedy,
        &mut lits,
        |hir, _| {
            test_f.received_hir = Some(hir.clone());
        },
    );

    assert!(test_f.received_hir.is_some());
    
    if let Some(hir) = test_f.received_hir {
        // Since max < min, we check if it returns a valid structure
        match hir {
            Hir::Concat(parts) => {
                assert_eq!(parts.len(), min as usize);
                for part in parts {
                    match part {
                        Hir::Literal(Literal::Unicode(b'b')) => {}
                        _ => panic!("Unexpected Hir variant"),
                    }
                }
            }
            _ => panic!("Expected Hir::Concat"),
        }
    }
}

