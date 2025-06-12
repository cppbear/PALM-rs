// Answer 0

#[test]
fn test_repetition_non_empty() {
    struct MockHir;
    
    impl MockHir {
        fn is_always_utf8(&self) -> bool {
            true
        }

        fn is_all_assertions(&self) -> bool {
            false
        }

        fn is_anchored_start(&self) -> bool {
            false
        }

        fn is_anchored_end(&self) -> bool {
            false
        }

        fn is_any_anchored_start(&self) -> bool {
            false
        }

        fn is_any_anchored_end(&self) -> bool {
            false
        }

        fn is_match_empty(&self) -> bool {
            false
        }
    }

    let mock_hir = Box::new(MockHir);
    let rep = Repetition {
        kind: RepetitionKind::OneOrMore,
        greedy: true,
        hir: mock_hir,
    };

    let result = Hir::repetition(rep);
    
    assert_eq!(result.kind, HirKind::Repetition(rep));
    assert!(!result.info.is_match_empty());
    assert!(!result.info.is_anchored_start());
    assert!(!result.info.is_anchored_end());
    assert!(result.info.is_always_utf8());
    assert!(!result.info.is_all_assertions());
    assert!(!result.info.is_any_anchored_start());
    assert!(!result.info.is_any_anchored_end());
}

#[test]
fn test_repetition_zero_or_more_non_empty() {
    struct MockHir;

    impl MockHir {
        fn is_always_utf8(&self) -> bool {
            true
        }

        fn is_all_assertions(&self) -> bool {
            false
        }

        fn is_anchored_start(&self) -> bool {
            false
        }

        fn is_anchored_end(&self) -> bool {
            false
        }

        fn is_any_anchored_start(&self) -> bool {
            false
        }

        fn is_any_anchored_end(&self) -> bool {
            false
        }

        fn is_match_empty(&self) -> bool {
            false
        }
    }

    let mock_hir = Box::new(MockHir);
    let rep = Repetition {
        kind: RepetitionKind::ZeroOrMore,
        greedy: true,
        hir: mock_hir,
    };

    let result = Hir::repetition(rep);
    
    assert_eq!(result.kind, HirKind::Repetition(rep));
    assert!(!result.info.is_match_empty());
    assert!(!result.info.is_anchored_start());
    assert!(!result.info.is_anchored_end());
    assert!(result.info.is_always_utf8());
    assert!(!result.info.is_all_assertions());
    assert!(!result.info.is_any_anchored_start());
    assert!(!result.info.is_any_anchored_end());
}

