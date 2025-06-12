// Answer 0

#[test]
fn test_alternation_empty() {
    let exprs: Vec<Hir> = Vec::new();
    let result = alternation(exprs);
    assert_eq!(result, Hir::empty());
}

#[test]
fn test_alternation_single_element() {
    let single_expr = Hir {
        kind: HirKind::Literal("a".into()),
        info: HirInfo::new(),
    };
    let exprs = vec![single_expr];
    let result = alternation(exprs);
    assert_eq!(result.kind, HirKind::Literal("a".into()));
}

#[test]
fn test_alternation_multiple_elements_with_all_true_attributes() {
    let expr1 = Hir {
        kind: HirKind::Literal("a".into()),
        info: {
            let mut info = HirInfo::new();
            info.set_always_utf8(true);
            info.set_all_assertions(true);
            info.set_anchored_start(true);
            info.set_anchored_end(true);
            info.set_any_anchored_start(true);
            info.set_any_anchored_end(true);
            info.set_match_empty(true);
            info
        },
    };

    let expr2 = Hir {
        kind: HirKind::Literal("b".into()),
        info: {
            let mut info = HirInfo::new();
            info.set_always_utf8(true);
            info.set_all_assertions(true);
            info.set_anchored_start(true);
            info.set_anchored_end(true);
            info.set_any_anchored_start(true);
            info.set_any_anchored_end(true);
            info.set_match_empty(true);
            info
        },
    };

    let exprs = vec![expr1, expr2];
    let result = alternation(exprs);

    assert!(matches!(result.kind, HirKind::Alternation(_)));
    assert!(result.info.is_always_utf8());
    assert!(result.info.is_all_assertions());
    assert!(result.info.is_anchored_start());
    assert!(result.info.is_anchored_end());
    assert!(result.info.is_any_anchored_start());
    assert!(result.info.is_any_anchored_end());
    assert!(result.info.is_match_empty());
}

#[test]
fn test_alternation_two_mixed_attributes() {
    let expr1 = Hir {
        kind: HirKind::Literal("a".into()),
        info: {
            let mut info = HirInfo::new();
            info.set_always_utf8(true);
            info.set_all_assertions(false);
            info.set_anchored_start(true);
            info.set_anchored_end(false);
            info.set_any_anchored_start(true);
            info.set_any_anchored_end(false);
            info.set_match_empty(false);
            info
        },
    };

    let expr2 = Hir {
        kind: HirKind::Literal("b".into()),
        info: {
            let mut info = HirInfo::new();
            info.set_always_utf8(true);
            info.set_all_assertions(true);
            info.set_anchored_start(false);
            info.set_anchored_end(true);
            info.set_any_anchored_start(false);
            info.set_any_anchored_end(true);
            info.set_match_empty(true);
            info
        },
    };

    let exprs = vec![expr1, expr2];
    let result = alternation(exprs);

    assert!(matches!(result.kind, HirKind::Alternation(_)));
    assert!(result.info.is_always_utf8());
    assert!(!result.info.is_all_assertions());
    assert!(result.info.is_anchored_start());
    assert!(result.info.is_anchored_end());
    assert!(result.info.is_any_anchored_start());
    assert!(result.info.is_any_anchored_end());
    assert!(result.info.is_match_empty());
}

