// Answer 0

#[test]
fn test_repetition_with_zero_or_one() {
    struct DummyHir;
    impl DummyHir {
        fn is_always_utf8(&self) -> bool { true }
        fn is_all_assertions(&self) -> bool { false }
        fn is_anchored_start(&self) -> bool { false }
        fn is_anchored_end(&self) -> bool { false }
        fn is_any_anchored_start(&self) -> bool { false }
        fn is_any_anchored_end(&self) -> bool { false }
        fn is_match_empty(&self) -> bool { true }
    }

    struct DummyRepetition {
        kind: RepetitionKind,
        greedy: bool,
        hir: Box<DummyHir>,
    }

    let dummy_hir = DummyHir;
    let repetition = DummyRepetition {
        kind: RepetitionKind::ZeroOrOne,
        greedy: true,
        hir: Box::new(dummy_hir),
    };
    
    let result = repetition(repetition);
    
    assert_eq!(result.kind, HirKind::Repetition(repetition));
    assert!(result.info.is_match_empty());
}

#[test]
fn test_repetition_with_zero_or_more() {
    struct DummyHir;
    impl DummyHir {
        fn is_always_utf8(&self) -> bool { false }
        fn is_all_assertions(&self) -> bool { false }
        fn is_anchored_start(&self) -> bool { false }
        fn is_anchored_end(&self) -> bool { false }
        fn is_any_anchored_start(&self) -> bool { false }
        fn is_any_anchored_end(&self) -> bool { false }
        fn is_match_empty(&self) -> bool { true }
    }

    struct DummyRepetition {
        kind: RepetitionKind,
        greedy: bool,
        hir: Box<DummyHir>,
    }

    let dummy_hir = DummyHir;
    let repetition = DummyRepetition {
        kind: RepetitionKind::ZeroOrMore,
        greedy: true,
        hir: Box::new(dummy_hir),
    };
    
    let result = repetition(repetition);
    
    assert_eq!(result.kind, HirKind::Repetition(repetition));
    assert!(result.info.is_match_empty());
}

#[test]
fn test_repetition_with_empty_exactly() {
    struct DummyHir;
    impl DummyHir {
        fn is_always_utf8(&self) -> bool { true }
        fn is_all_assertions(&self) -> bool { false }
        fn is_anchored_start(&self) -> bool { true }
        fn is_anchored_end(&self) -> bool { false }
        fn is_any_anchored_start(&self) -> bool { false }
        fn is_any_anchored_end(&self) -> bool { false }
        fn is_match_empty(&self) -> bool { true }
    }

    struct DummyRepetition {
        kind: RepetitionKind,
        greedy: bool,
        hir: Box<DummyHir>,
    }

    let dummy_hir = DummyHir;
    let repetition = DummyRepetition {
        kind: RepetitionKind::Range(RepetitionRange::Exactly(0)),
        greedy: true,
        hir: Box::new(dummy_hir),
    };

    let result = repetition(repetition);
    
    assert_eq!(result.kind, HirKind::Repetition(repetition));
    assert!(result.info.is_match_empty());
}

